use std::{collections::HashMap, sync::Arc};

use bevy::prelude::*;
use tokio::{runtime::Runtime, sync::mpsc::{UnboundedReceiver as Receiver, UnboundedSender as Sender}};
use serde::{Deserialize, Serialize};

use crate::network::synchronizer::Synchronizer;


#[derive(Debug, Resource, Deserialize, Serialize)]
pub enum WSMessages {
    Message(String),
    Connected,
    Disconnected,
    Component {
        data: serde_json::Value,
    },
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

#[derive(Resource, Clone)]
pub struct MultiplayerRuntime(pub Arc<Runtime>);

