pub mod map;
pub mod network;
pub mod player;
use avian3d::prelude::PhysicsLayer;

// Define collision layers
#[derive(PhysicsLayer, Clone, Copy, Debug, Default)]
enum GameLayer {
    #[default]
    LocalPlayer,
    OnlinePlayer,
    Environment,
}
