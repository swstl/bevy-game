use crate::network::resource;
use crate::network::resource::WSMessages;
use bevy::prelude::*;
use futures_util::{SinkExt, StreamExt};
use std::{collections::HashMap, thread};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::connect_async;
use tokio::sync::mpsc::{UnboundedSender as Sender, UnboundedReceiver as Receiver, self};


const APP_ID: &str = "67";
const URL: &str = "wss://broadcast.dogfetus.no";

pub(crate) fn connect_multiplayer(
    mut commands: Commands,
    mp_runtime: Res<resource::MultiplayerRuntime>,
) {
    let (to_others_tx, to_others_rx) = mpsc::unbounded_channel();
    let (to_us_tx, to_us_rx) = mpsc::unbounded_channel();

    spawn_ws_threads(mp_runtime, to_others_rx, to_us_tx);

    commands.insert_resource(resource::WSMessageChannels {
        incomming: to_us_rx,
        outgoing: to_others_tx,
    });

    commands.insert_resource(resource::LobbyInfo {
        connected_players: HashMap::new(),
    });
}



fn spawn_ws_threads(
    mp_runtime: Res<resource::MultiplayerRuntime>,
    to_others: Receiver<WSMessages>,
    to_us: Sender<WSMessages>,
) {
    mp_runtime.0.spawn(async {
        let mut request = URL.into_client_request().expect("Invalid URL");
        request
            .headers_mut()
            .insert("app_id", APP_ID.parse().expect("Invalid APP_ID"));

         match connect_async(request).await {
            Ok((socket, _response)) => {
                info!("WebSocket connected successfully");
                let (sender, receiver) = socket.split();

                tokio::spawn(ws_sender(sender, to_others));
                tokio::spawn(ws_receiver(receiver, to_us));
            }
            Err(e) => {
                error!("Failed to connect to WebSocket: {:?}", e);
            }
        }
    });
}

// Spawn sender task
async fn ws_sender(
    mut ws_sender: futures_util::stream::SplitSink<
        tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
        Message,
    >,
    mut to_others: Receiver<WSMessages>,
) {
    while let Some(msg) = to_others.recv().await {
        let json = serde_json::to_string(&msg).unwrap();
        if let Err(e) = ws_sender.send(Message::Text(json.into())).await {
            eprintln!("Failed to send message: {:?}", e);
            break;
        }
    }
}

async fn ws_receiver(
    mut ws_receiver: futures_util::stream::SplitStream<
        tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
    >,
    to_us: Sender<WSMessages>,
) {
    // Spawn receiver task
    while let Some(message) = ws_receiver.next().await {
        match message {
            Ok(msg) => {
                let text = msg.into_text().unwrap();
                match serde_json::from_str::<WSMessages>(&text) {
                    Ok(ws_msg) => {
                        if let Err(e) = to_us.send(ws_msg) {
                            eprintln!("Failed to send message: {:?}", e);
                            break;
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to deserialize message: {:?}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("WebSocket error: {:?}", e);
                break;
            }
        }
    }
}
