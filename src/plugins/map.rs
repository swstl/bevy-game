use crate::{components::objects::Ground, plugins::GameLayer};
use crate::plugins::menu::GameState;
use avian3d::prelude::*;
use bevy::prelude::*;
use rand::{Rng, SeedableRng, rngs::StdRng};

const MAP_SIZE: usize = 50;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), generate_random_map)
            .add_systems(OnExit(GameState::Playing), cleanup_map);
    }
}

// fn generate_random_map(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     let floor_mat = materials.add(Color::srgb(0.3, 0.8, 0.3));
//
//     // Spawn floor
//     commands.spawn((
//         Name::new("Floor"),
//         Mesh3d(meshes.add(Cuboid::new(MAP_SIZE as f32, 0.1, MAP_SIZE as f32))),
//         MeshMaterial3d(floor_mat.clone()),
//         Transform::from_xyz(MAP_SIZE as f32/2.0, 0.0, MAP_SIZE as f32/2.0),
//         RigidBody::Static,
//         Friction::ZERO,
//         Ground,
//         CollisionLayers::new(
//             [GameLayer::Environment],
//             [GameLayer::LocalPlayer, GameLayer::OnlinePlayer],
//         ),
//         Collider::cuboid(MAP_SIZE as f32, 0.1, MAP_SIZE as f32),
//     ));
// }
//

fn generate_random_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = StdRng::seed_from_u64(12345);  // Change this number to change the map

    let floor_mat = materials.add(Color::srgb(0.3, 0.8, 0.3));
    let platform_mat = materials.add(Color::srgb(0.8, 0.6, 0.2));

    // Spawn floor
    commands.spawn((
        Name::new("Floor"),
        Mesh3d(meshes.add(Cuboid::new(MAP_SIZE as f32, 0.1, MAP_SIZE as f32))),
        MeshMaterial3d(floor_mat.clone()),
        Transform::from_xyz(MAP_SIZE as f32 / 2.0, 0.0, MAP_SIZE as f32 / 2.0),
        RigidBody::Static,
        Friction::ZERO,
        Ground,
        CollisionLayers::new(
            [GameLayer::Environment],
            [GameLayer::LocalPlayer, GameLayer::OnlinePlayer],
        ),
        Collider::cuboid(MAP_SIZE as f32, 0.1, MAP_SIZE as f32),
    ));

    // Generate parkour path
    let num_platforms = 30;
    let mut current_pos = Vec3::new(5.0, 1.0, 5.0); // Starting position

    for i in 0..num_platforms {
        // Random platform size (smaller = harder)
        let size_x = rng.random_range(1.5..3.5);
        let size_z = rng.random_range(1.5..3.5);
        let height = 0.3;

        // Spawn platform
        commands.spawn((
            Name::new(format!("Platform_{}", i)),
            Mesh3d(meshes.add(Cuboid::new(size_x, height, size_z))),
            MeshMaterial3d(platform_mat.clone()),
            Transform::from_translation(current_pos),
            RigidBody::Static,
            Friction::ZERO,
            Ground,
            CollisionLayers::new(
                [GameLayer::Environment],
                [GameLayer::LocalPlayer, GameLayer::OnlinePlayer],
            ),
            Collider::cuboid(size_x, height, size_z),
        ));

        // Calculate next platform position
        let jump_distance = rng.random_range(2.0..4.0); // Horizontal jump distance
        let height_gain = rng.random_range(0.3..1.2); // Vertical climb
        let angle = rng.random_range(0.0..std::f32::consts::TAU); // Random direction

        current_pos.x += angle.cos() * jump_distance;
        current_pos.z += angle.sin() * jump_distance;
        current_pos.y += height_gain;

        // Keep platforms within bounds
        current_pos.x = current_pos.x.clamp(2.0, MAP_SIZE as f32 - 2.0);
        current_pos.z = current_pos.z.clamp(2.0, MAP_SIZE as f32 - 2.0);
    }
}

fn cleanup_map(
    mut commands: Commands,
    ground_query: Query<Entity, With<Ground>>,
) {
    for entity in ground_query.iter() {
        commands.entity(entity).despawn();
    }
}
