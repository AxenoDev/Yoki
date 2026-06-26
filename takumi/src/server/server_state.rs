use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug)]
pub struct ServerState {
    online_players: AtomicUsize,
    max_players: usize,
    allow_unsupported_versions: bool,
    reply_to_status: bool,
    accept_transfers: bool,
}

impl ServerState {
    pub fn new(max_players: usize) -> Self {
        Self {
            online_players: AtomicUsize::new(0),
            max_players,
            allow_unsupported_versions: false,
            reply_to_status: true,
            accept_transfers: false,
        }
    }

    pub fn online_players(&self) -> usize {
        self.online_players.load(Ordering::Relaxed)
    }

    pub fn max_players(&self) -> usize {
        self.max_players
    }

    pub const fn allow_unsupported_versions(&self) -> bool {
        self.allow_unsupported_versions
    }

    pub const fn reply_to_status(&self) -> bool {
        self.reply_to_status
    }

    pub const fn accept_transfers(&self) -> bool {
        self.accept_transfers
    }

    pub fn increment(&self) {
        self.online_players.fetch_add(1, Ordering::Relaxed);
    }

    pub fn decrement(&self) {
        self.online_players.fetch_sub(1, Ordering::Relaxed);
    }
}

impl Default for ServerState {
    fn default() -> Self {
        Self::new(20)
    }
}
