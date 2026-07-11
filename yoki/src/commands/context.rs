use std::collections::HashMap;
use std::sync::Arc;

use crate::ServerState;
use crate::commands::args::ArgValue;
use crate::server::players::ConnectedPlayer;

pub struct CommandContext<'a> {
    pub sender: Arc<ConnectedPlayer>,
    pub server_state: &'a ServerState,
    args: HashMap<&'static str, ArgValue>,
}

impl<'a> CommandContext<'a> {
    pub fn new(
        sender: Arc<ConnectedPlayer>,
        server_state: &'a ServerState,
        args: HashMap<&'static str, ArgValue>,
    ) -> Self {
        Self {
            sender,
            server_state,
            args,
        }
    }

    pub fn reply(&self, message: impl Into<String>) {
        self.sender.send(message);
    }

    pub fn broadcast(&self, message: impl Into<String>) {
        self.server_state.broadcast(message);
    }

    pub fn get_str(&self, name: &str) -> Option<&str> {
        match self.args.get(name) {
            Some(ArgValue::Str(value)) => Some(value),
            _ => None,
        }
    }

    pub fn get_int(&self, name: &str) -> Option<i32> {
        match self.args.get(name) {
            Some(ArgValue::Int(value)) => Some(*value),
            _ => None,
        }
    }

    pub fn get_float(&self, name: &str) -> Option<f32> {
        match self.args.get(name) {
            Some(ArgValue::Float(value)) => Some(*value),
            _ => None,
        }
    }

    pub fn get_double(&self, name: &str) -> Option<f64> {
        match self.args.get(name) {
            Some(ArgValue::Double(value)) => Some(*value),
            _ => None,
        }
    }

    pub fn get_bool(&self, name: &str) -> Option<bool> {
        match self.args.get(name) {
            Some(ArgValue::Bool(value)) => Some(*value),
            _ => None,
        }
    }

    pub fn get_player(&self, name: &str) -> Option<Arc<ConnectedPlayer>> {
        match self.args.get(name) {
            Some(ArgValue::Player(player)) => Some(Arc::clone(player)),
            _ => None,
        }
    }
}
