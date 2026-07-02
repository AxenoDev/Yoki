use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures::Stream;
use minecraft_packet::Connection;
use minecraft_protocol::{DirectionBound, State};
use yoki_binutils::ProtocolError;

use super::client_state::ClientState;
use super::packet_registry::PacketRegistry;

type AsyncClosure =
    Box<dyn FnOnce() -> Pin<Box<dyn Future<Output = PacketRegistry> + Send>> + Send + 'static>;

enum Producer {
    SyncClosure(Box<dyn FnOnce() -> PacketRegistry + Send + 'static>),
    AsyncClosure(AsyncClosure),
    Iterator(Box<dyn Iterator<Item = PacketRegistry> + Send + 'static>),
    StateChange(DirectionBound, State),
}

pub struct Batch {
    producers: VecDeque<Producer>,
}

impl Batch {
    pub const fn new() -> Self {
        Self {
            producers: VecDeque::new(),
        }
    }

    pub fn queue_both_state_change(&mut self, new_state: State) {
        self.queue_clientbound_state_change(new_state);
        self.queue_serverbound_state_change(new_state);
    }

    pub fn queue_clientbound_state_change(&mut self, new_state: State) {
        self.producers
            .push_back(Producer::StateChange(DirectionBound::Clientbound, new_state));
    }

    pub fn queue_serverbound_state_change(&mut self, new_state: State) {
        self.producers
            .push_back(Producer::StateChange(DirectionBound::Serverbound, new_state));
    }

    pub fn queue<F>(&mut self, f: F)
    where
        F: FnOnce() -> PacketRegistry + Send + 'static,
    {
        self.producers.push_back(Producer::SyncClosure(Box::new(f)));
    }

    pub fn queue_packet(&mut self, packet: PacketRegistry) {
        self.queue(move || packet);
    }

    pub fn queue_async<F, Fut>(&mut self, f: F)
    where
        F: FnOnce() -> Fut + Send + 'static,
        Fut: Future<Output = PacketRegistry> + Send + 'static,
    {
        let closure =
            move || -> Pin<Box<dyn Future<Output = PacketRegistry> + Send>> { Box::pin(f()) };
        self.producers
            .push_back(Producer::AsyncClosure(Box::new(closure)));
    }

    pub fn chain_iter<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = PacketRegistry>,
        I::IntoIter: Send + 'static,
    {
        self.producers
            .push_back(Producer::Iterator(Box::new(iter.into_iter())));
    }

    pub fn into_stream(self) -> BatchStream {
        BatchStream {
            producers: self.producers,
            current: Current::Idle,
        }
    }

    pub async fn execute(
        self,
        conn: &mut Connection,
        client_state: &mut ClientState,
    ) -> Result<(), ProtocolError> {
        use futures::StreamExt;

        let mut stream = self.into_stream();
        while let Some(item) = stream.next().await {
            match item {
                BatchItem::Packet(packet) => {
                    let raw = packet.encode_clientbound(client_state.protocol_version())?;
                    conn.send_raw(&raw).await?;
                }
                BatchItem::StateChange(direction, new_state) => {
                    client_state.set_state(direction, new_state);
                }
            }
        }

        Ok(())
    }
}

impl Default for Batch {
    fn default() -> Self {
        Self::new()
    }
}

enum Current {
    Idle,
    Future(Pin<Box<dyn Future<Output = PacketRegistry> + Send>>),
    Iterator(Box<dyn Iterator<Item = PacketRegistry> + Send>),
}

pub struct BatchStream {
    producers: VecDeque<Producer>,
    current: Current,
}

pub enum BatchItem {
    Packet(PacketRegistry),
    StateChange(DirectionBound, State),
}

impl Stream for BatchStream {
    type Item = BatchItem;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();

        loop {
            match &mut this.current {
                Current::Future(fut) => match fut.as_mut().poll(cx) {
                    Poll::Ready(item) => {
                        this.current = Current::Idle;
                        return Poll::Ready(Some(BatchItem::Packet(item)));
                    }
                    Poll::Pending => return Poll::Pending,
                },
                Current::Iterator(iter) => {
                    if let Some(item) = iter.next() {
                        return Poll::Ready(Some(BatchItem::Packet(item)));
                    }
                    this.current = Current::Idle;
                }
                Current::Idle => match this.producers.pop_front() {
                    Some(Producer::SyncClosure(f)) => {
                        return Poll::Ready(Some(BatchItem::Packet(f())));
                    }
                    Some(Producer::StateChange(direction, new_state)) => {
                        return Poll::Ready(Some(BatchItem::StateChange(direction, new_state)));
                    }
                    Some(Producer::AsyncClosure(f)) => {
                        this.current = Current::Future(f());
                    }
                    Some(Producer::Iterator(iter)) => {
                        this.current = Current::Iterator(iter);
                    }
                    None => return Poll::Ready(None),
                },
            }
        }
    }
}
