use std::collections::HashMap;
use bevy::prelude::*;
use tokio::{sync::mpsc::{UnboundedReceiver as Receiver, UnboundedSender as Sender}};

use crate::network::synchronizer::Synchronizer;


#[derive(Debug, Resource)]
pub enum WSMessages {
    Message(String),
    Connected,
    Disconnected,
    Sync(Vec<u8>),
}


#[derive(Resource, Debug)]
pub struct WSMessageChannels {
    pub incomming: Receiver<WSMessages>,
    pub outgoing: Sender<WSMessages>
}

#[derive(Resource, Debug)]
pub struct LobbyInfo {
    pub connected_players: HashMap<i64, Synchronizer>
}

