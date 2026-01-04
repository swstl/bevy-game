/////////////////////////////////////////////////////////
///////////////////////// Network mod ////////////////////////
/////////////////////////////////////////////////////////
pub mod resource;
pub mod synchronizer;

#[cfg(not(target_arch = "wasm32"))]
pub mod native;
#[cfg(not(target_arch = "wasm32"))]
use native::connect_multiplayer;

#[allow(inactive_code)]
#[cfg(target_arch = "wasm32")]
pub mod wasm;
#[cfg(target_arch = "wasm32")]
use wasm::connect_multiplayer;

use synchronizer::{handle_sync, multiplayer_sender};
use bevy::prelude::*;
use tokio::runtime::Builder;
use std::sync::Arc;


#[derive(Component)]
pub struct Recieved;


pub struct MultiplayerPlugin;


impl Plugin for MultiplayerPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            use native::MultiplayerRuntime;
            let mp_runtime = Arc::new(
                Builder::new_multi_thread()
                    .worker_threads(2)
                    .thread_name("mp-workers")
                    .enable_all()
                    .build()
                    .expect("Failed to create Tokio runtime for multiplayer")
            );
            app.insert_resource(MultiplayerRuntime(mp_runtime));
        }

        app.add_systems(Startup, connect_multiplayer);
        app.add_systems(Update, (multiplayer_sender, handle_sync));
    }
}
