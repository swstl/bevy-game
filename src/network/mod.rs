pub mod resource;
pub mod synchronizer;
pub mod core;

use crate::network::{core::connect_multiplayer, synchronizer::{handle_sync, multiplayer_sender}};
use bevy::prelude::*;
use tokio::runtime::Builder;
use std::sync::Arc;
use crate::network::resource::MultiplayerRuntime;


#[derive(Component)]
pub struct Recieved;


pub struct MultiplayerPlugin;


impl Plugin for MultiplayerPlugin {
    fn build(&self, app: &mut App) {
        let mp_runtime = Arc::new(
            Builder::new_multi_thread()
                .worker_threads(2)  // 1 for sender, 1 for receiver
                .thread_name("mp-workers")
                .enable_all()
                .build()
                .expect("Failed to create Tokio runtime for multiplayer")
        );

        app.insert_resource(MultiplayerRuntime(mp_runtime));
        app.add_systems(Startup, connect_multiplayer);
        app.add_systems(Update, multiplayer_sender);
        app.add_systems(Update, handle_sync);
    }
}
