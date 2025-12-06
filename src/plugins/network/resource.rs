/////////////////////////////////////////////////////////
///////////////////////// resource ////////////////////////
/////////////////////////////////////////////////////////
use std::collections::HashMap;
use bevy::prelude::*;
use tokio::{sync::mpsc::{UnboundedReceiver as Receiver, UnboundedSender as Sender}};


#[derive(Debug, Resource)]
pub enum WSMessages {
    Message(String),
    Connected(Vec<u8>),
    Disconnected(Vec<u8>),
    Sync(Vec<u8>),
}


#[derive(Resource, Debug)]
pub struct WSMessageChannels {
    pub incomming: Receiver<WSMessages>,
    pub outgoing: Sender<WSMessages>
}


#[derive(Resource, Debug, Default)]
pub struct LobbyInfo {
    pub players: HashMap<i64, Entity>
}

