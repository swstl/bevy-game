///////////////////////////////////////////////////////
///////////////////////// wasm ////////////////////////
///////////////////////////////////////////////////////

use crate::network::resource::{WSMessages, WSMessageChannels, LobbyInfo};
use bevy::prelude::*;
use futures_util::{SinkExt, StreamExt};
use std::collections::HashMap;
use tokio::sync::mpsc::{self, UnboundedSender as Sender, UnboundedReceiver as Receiver};
use tokio_tungstenite_wasm::WebSocketStream;
use tokio_tungstenite_wasm::Message;

const APP_ID: &str = "67";
const URL: &str = "wss://broadcast.dogfetus.no";

///////////////////////////////////////////////////////////////
//////////////////////// Initial setup ////////////////////////
///////////////////////////////////////////////////////////////
pub fn connect_multiplayer(
    mut commands: Commands,
) {
    let (to_others_tx, to_others_rx) = mpsc::unbounded_channel();
    let (to_us_tx, to_us_rx) = mpsc::unbounded_channel();
 
    spawn_ws_tasks(to_others_rx, to_us_tx);

    commands.insert_resource(WSMessageChannels {
        incomming: to_us_rx,
        outgoing: to_others_tx,
    });
 
    commands.init_resource::<resource::LobbyInfo>();
}

fn spawn_ws_tasks(
    to_others: Receiver<WSMessages>,
    to_us: Sender<WSMessages>,
) {
    wasm_bindgen_futures::spawn_local(async move {
        let url_with_app_id = format!("{}?app_id={}", URL, APP_ID);
        let ws = tokio_tungstenite_wasm::connect(url_with_app_id);

        match ws.await {
            Ok(socket) => {
                // info!("WebSocket connected successfully");
                let (sender, receiver) = socket.split();
                wasm_bindgen_futures::spawn_local(ws_sender(sender, to_others));
                wasm_bindgen_futures::spawn_local(ws_receiver(receiver, to_us));
            }
            Err(e) => {
                error!("Failed to connect to WebSocket: {:?}", e);
            }
        }
    });
}

async fn ws_sender(
    mut ws_sender: futures_util::stream::SplitSink<WebSocketStream, Message>,
    mut to_others: Receiver<WSMessages>,
) {
    while let Some(msg) = to_others.recv().await {
        let to_send = match msg {
            WSMessages::Sync(s) => {
                Message::Binary(s.into())
            }
            _ => Message::Text("hei".into()),
        };
        if let Err(e) = ws_sender.send(to_send).await {
            eprintln!("Failed to send message: {:?}", e);
            break;
        }
    }
}

async fn ws_receiver(
    mut ws_receiver: futures_util::stream::SplitStream<WebSocketStream>,
    to_us: Sender<WSMessages>,
) {
    while let Some(message) = ws_receiver.next().await {
        match message {
            Ok(msg) => {
                let bytes = msg.into_data();
                to_us.send(WSMessages::Sync(bytes.to_vec())).ok();
            }
            Err(e) => {
                eprintln!("WebSocket error: {:?}", e);
            }
        };
    }
}
