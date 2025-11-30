pub mod bundle;

use crate::components::entities::Player;
use crate::components::vitals::Movement;
use crate::components::objects::Ground;
use crate::network::synchronizer::Synchronizer;
use avian3d::prelude::*;
use bevy::prelude::*;

use bundle::PlayerBundle;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement);
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn((
            Player,
            PlayerBundle::new(
                meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
                materials.add(Color::srgb(0.8, 0.3, 1.0)),
                Vec3::new(0.0, 2.0, 0.0)
            ),
            Synchronizer::default()
        ))
        .with_children(|parent| {
            parent.spawn((
                Camera3d::default(),
                Transform::from_xyz(0.0, 2.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ));
        })
        .observe(on_ground_collision);
}

fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut LinearVelocity, &mut Movement), With<Player>>,
) {
    for (mut transform, mut velocity, mut player) in &mut query {
        let mut speed = player.speed;

        if transform.translation.y <= 0.2 {
            transform.translation.y = 0.2;
            velocity.y = 0.0;
        }
        if keyboard.any_pressed([KeyCode::Space])
            && (player.is_grounded || transform.translation.y <= 0.21)
        {
            velocity.y = player.jump_strength;
            player.is_grounded = false;
        }
        if keyboard.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
            speed = player.speed * player.sprint_aplifier;
        }

        let mut direction = Vec3::ZERO;

        if keyboard.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
            direction.x -= 1.0;
        }
        if keyboard.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
            direction.x += 1.0;
        }
        if keyboard.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
            direction.z -= 1.0;
        }
        if keyboard.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
            direction.z += 1.0;
        }

        if direction.length_squared() > 0.0 {
            direction = direction.normalize();
        } else {
            direction = Vec3::ZERO;
        }

        velocity.x = direction.x * speed * time.delta_secs();
        velocity.z = direction.z * speed * time.delta_secs();
    }
}

fn on_ground_collision(
    event: On<CollisionStart>,
    mut player_query: Query<&mut Movement, With<Player>>,
    ground_query: Query<&Ground>,
) {
    if ground_query.get(event.collider2).is_ok() &&
        let Ok(mut player) = player_query.get_mut(event.collider1)
    {
        player.is_grounded = true;
    }
}
