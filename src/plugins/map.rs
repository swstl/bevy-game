use crate::components::objects::Ground;
use avian3d::prelude::*;
use bevy::prelude::*;
use rand::Rng;


const MAP_SIZE: usize = 50;


pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, generate_random_map);
    }
}

fn generate_random_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = rand::rng();

    let wall_mat = materials.add(Color::srgb(0.5, 0.5, 0.5));
    let floor_mat = materials.add(Color::srgb(0.3, 0.8, 0.3));

    // for x in 0..MAP_SIZE {
    //     for z in 0..MAP_SIZE {
    //         // Random chance to spawn a wall
    //         if rng.random_bool(0.1) {
    //             commands.spawn((
    //                 Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
    //                 MeshMaterial3d(wall_mat.clone()),
    //                 Transform::from_xyz(x as f32, 1.0, z as f32),
    //                 RigidBody::Static,
    //                 Friction::ZERO,
    //                 Ground,
    //                 Collider::cuboid(1.0, 1.0, 1.0)
    //             ));
    //         }
    //     }
    // }

    // Spawn floor
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(MAP_SIZE as f32, 0.1, MAP_SIZE as f32))),
        MeshMaterial3d(floor_mat.clone()),
        Transform::from_xyz(MAP_SIZE as f32/2.0, 0.0, MAP_SIZE as f32/2.0),
        RigidBody::Static,
        Friction::ZERO,
        Ground,
        Collider::cuboid(MAP_SIZE as f32, 0.1, MAP_SIZE as f32),
    ));
}
