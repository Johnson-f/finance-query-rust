use actix::prelude::*;
use serde_json::Value;
use std::collections::HashMap;
use tokio::task::JoinHandle;
use tokio::sync::mpsc;
use tracing::debug;

/// Session entry with unique ID for tracking
#[derive(Clone)]
struct SessionEntry {
    id: usize,
    sender: mpsc::UnboundedSender<Value>,
}

/// Message to broadcast to all connections in a channel
#[derive(Message)]
#[rtype(result = "()")]
pub struct BroadcastMessage {
    pub channel: String,
    pub message: Value,
}

/// Message to connect a WebSocket session to a channel
#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub session_id: usize,
    pub sender: mpsc::UnboundedSender<Value>,
    pub channel: String,
}

/// Message to start a background task for a channel (only started if channel doesn't exist)
#[derive(Message)]
#[rtype(result = "()")]
pub struct StartTask {
    pub channel: String,
    pub task: tokio::task::JoinHandle<()>,
}

/// Message to disconnect a WebSocket session from a channel
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub session_id: usize,
    pub channel: String,
}

/// Connection manager actor that handles WebSocket connections
pub struct ConnectionManager {
    /// Map of channel name to list of session entries
    sessions: HashMap<String, Vec<SessionEntry>>,
    /// Map of channel name to background task handle
    tasks: HashMap<String, JoinHandle<()>>,
}

impl Default for ConnectionManager {
    fn default() -> Self {
        ConnectionManager {
            sessions: HashMap::new(),
            tasks: HashMap::new(),
        }
    }
}

impl Actor for ConnectionManager {
    type Context = Context<Self>;
}

impl Handler<Connect> for ConnectionManager {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) {
        debug!("Connecting WebSocket session {} to channel: {}", msg.session_id, msg.channel);
        
        let is_new_channel = !self.sessions.contains_key(&msg.channel);
        
        if is_new_channel {
            self.sessions.insert(msg.channel.clone(), Vec::new());
        }
        
        if let Some(sessions) = self.sessions.get_mut(&msg.channel) {
            sessions.push(SessionEntry {
                id: msg.session_id,
                sender: msg.sender,
            });
            debug!("Channel {} now has {} connections", msg.channel, sessions.len());
        }
    }
}

impl Handler<StartTask> for ConnectionManager {
    type Result = ();

    fn handle(&mut self, msg: StartTask, _: &mut Context<Self>) {
        // Start task only if channel doesn't exist yet (matches Python pattern)
        if !self.tasks.contains_key(&msg.channel) {
            debug!("Starting background task for new channel: {}", msg.channel);
            self.tasks.insert(msg.channel, msg.task);
        } else {
            debug!("Channel {} already has a task running, ignoring new task", msg.channel);
            // Abort the new task since we already have one
            msg.task.abort();
        }
    }
}

impl Handler<Disconnect> for ConnectionManager {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        debug!("Disconnecting WebSocket session {} from channel: {}", msg.session_id, msg.channel);
        
        if let Some(sessions) = self.sessions.get_mut(&msg.channel) {
            sessions.retain(|entry| entry.id != msg.session_id);
            
            if sessions.is_empty() {
                debug!("Channel {} has no more connections, cleaning up", msg.channel);
                self.sessions.remove(&msg.channel);
                
                // Cancel background task if it exists
                if let Some(task) = self.tasks.remove(&msg.channel) {
                    task.abort();
                    debug!("Cancelled background task for channel: {}", msg.channel);
                }
            } else {
                debug!("Channel {} still has {} connections", msg.channel, sessions.len());
            }
        }
    }
}

impl Handler<BroadcastMessage> for ConnectionManager {
    type Result = ();

    fn handle(&mut self, msg: BroadcastMessage, _: &mut Context<Self>) {
        if let Some(sessions) = self.sessions.get(&msg.channel) {
            let mut disconnected = Vec::new();
            
            for entry in sessions {
                if entry.sender.send(msg.message.clone()).is_err() {
                    // Session is disconnected, mark for removal
                    disconnected.push(entry.id);
                }
            }
            
            // Remove disconnected sessions
            if let Some(sessions) = self.sessions.get_mut(&msg.channel) {
                for id in disconnected {
                    sessions.retain(|e| e.id != id);
                }
                
                // Clean up if no sessions remain
                if sessions.is_empty() {
                    self.sessions.remove(&msg.channel);
                    if let Some(task) = self.tasks.remove(&msg.channel) {
                        task.abort();
                    }
                }
            }
        }
    }
}

impl ConnectionManager {
    /// Register a background task for a channel
    pub fn register_task(&mut self, channel: String, task: JoinHandle<()>) {
        self.tasks.insert(channel, task);
    }
    
    /// Get the number of active connections for a channel
    pub fn connection_count(&self, channel: &str) -> usize {
        self.sessions.get(channel).map(|s| s.len()).unwrap_or(0)
    }
}

/// Wrapper for ConnectionManager to use in AppState
pub type ConnectionManagerAddr = Addr<ConnectionManager>;
