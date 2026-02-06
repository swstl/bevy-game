/////////////////////////////////
//////////// Imports ////////////
/////////////////////////////////
pub mod animation;
pub mod bundle;
pub mod camera;

use std::time::Duration;

use crate::components::entities::LocalPlayer;
use crate::components::entities::Player;
use crate::components::entities::PlayerAnimation;
use crate::components::entities::PlayerBody;
use crate::components::objects::Ground;
use crate::components::vitals::Movement;
use crate::plugins::GameLayer;
use crate::plugins::menu::GameState;
use crate::plugins::network::Recieved;
use crate::plugins::network::synchronizer::Synchronizer;
use animation::animate_player_meshes;
use animation::load_animation;
use avian3d::prelude::*;
use bevy::prelude::*;
use bundle::SimplePlayerBundle;

use camera::move_camera;
use camera::setup_camera;

pub const GLTF_PATH: &str = "character.glb";
pub const PLAYER_SCALE: Vec3 = Vec3::splat(0.3);

///////////////////////////////////////
//////////// Player plugin ////////////
///////////////////////////////////////
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), (spawn_player, setup_camera, load_animation))
            .add_systems(Update, (animate_player_meshes, move_player, move_camera).run_if(in_state(GameState::Playing)))
            .add_systems(OnExit(GameState::Playing), cleanup_player);
    }
}

/////////////////////////////////
//////////// Startup ////////////
/////////////////////////////////
fn spawn_player(mut commands: Commands, ass: Res<AssetServer>) {
    commands
        .spawn((
            Name::new("LocalPlayer"),
            Synchronizer::default(),
            SimplePlayerBundle::new(),
            Player,
            Visibility::default(),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Name::new("PlayerBody"),
                    PlayerBody,
                    LocalPlayer,
                    Collider::cuboid(
                        1.75 * PLAYER_SCALE.x,
                        2.8 * PLAYER_SCALE.y,
                        1.0 * PLAYER_SCALE.z,
                    ),
                    CollisionEventsEnabled,
                    CollisionLayers::new([GameLayer::LocalPlayer], [GameLayer::Environment]),
                    Visibility::default(),
                ))
                .with_child((
                    SceneRoot(ass.load(GltfAssetLabel::Scene(0).from_asset(GLTF_PATH))),
                    Transform::from_scale(PLAYER_SCALE).with_translation(Vec3::new(
                        0.0,
                        -1.4 * PLAYER_SCALE.y,
                        0.0,
                    )),
                    PlayerAnimation,
                ))
                .observe(on_ground_collision);

            parent.spawn((
                Name::new("PlayerCamera"),
                Camera3d::default(),
                Transform::from_xyz(0.0, 2.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ));
        });
}

///////////////////////////////////////////
//////////// Moving the player ////////////
///////////////////////////////////////////
type CameraQuery<'w, 's> =
    Single<'w, 's, &'static Transform, (With<Camera>, Without<Player>, Without<PlayerBody>)>;

type BodyQuery<'w, 's> = Single<
    'w,
    's,
    &'static mut Transform,
    (
        With<PlayerBody>,
        With<LocalPlayer>,
        Without<Player>,
        Without<Camera>,
    ),
>;

type PlayerQuery<'w, 's> = Single<
    'w,
    's,
    (
        &'static mut Transform,
        &'static mut LinearVelocity,
        &'static mut Movement,
        &'static mut Synchronizer,
    ),
    With<Player>,
>;

#[allow(clippy::too_many_arguments)]
fn move_player(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    camera: CameraQuery,
    player: PlayerQuery,
    mut body: BodyQuery,
    mut local_ap: Query<(&mut AnimationPlayer, &mut AnimationTransitions), With<LocalPlayer>>,
    mut current_animation: Local<AnimationNodeIndex>,
    animations: Res<animation::PlayerAnimations>,
) {
    let (mut transform, mut velocity, mut player, mut syncronizer) = player.into_inner();
    for (mut a_player, mut transitions) in &mut local_ap {
        let mut speed = player.speed;
        let mut direction = Vec3::ZERO;
        let mut animation_to_play;

        let camera_forward = camera.forward();
        let camera_right = camera.right();
        let sideways = Vec3::new(camera_right.x, 0.0, camera_right.z).normalize();
        let forward = Vec3::new(camera_forward.x, 0.0, camera_forward.z).normalize();

        if transform.translation.y < 0.0 {
            transform.translation.y = 0.2;
            velocity.y = 0.0;
        }

        if keyboard.any_just_pressed([KeyCode::Space]) && player.can_jump() {
            player.current_jumps += !player.is_grounded as u32;
            velocity.y = player.jump_strength;
            player.is_grounded = false;
            play_animation(
                &mut a_player,
                &mut transitions,
                &mut current_animation,
                animations.jump,
                false,
                true,
            );
        }
        if keyboard.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
            speed = player.speed * player.sprint_aplifier;
        }
        if keyboard.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
            direction -= sideways;
        }
        if keyboard.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
            direction += sideways;
        }
        if keyboard.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
            direction += forward;
        }
        if keyboard.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
            direction -= forward;
        }

        velocity.x = direction.x * speed * time.delta_secs();
        velocity.z = direction.z * speed * time.delta_secs();

        rotate_body_by_movement(&mut body, direction, &time);

        if direction.length_squared() > 0.0 {
            if speed > player.speed {
                animation_to_play = animations.run;
            } else {
                animation_to_play = animations.walk;
            }
        } else {
            animation_to_play = animations.idle;
        };

        if !player.is_grounded {
            animation_to_play = animations.jumpidle;
        }

        if *current_animation == animations.jump {
            let Some(animation) = a_player.animation(*current_animation) else {
                continue;
            };
            if !animation.is_finished() {
                animation_to_play = animations.jump;
            }
        }

        // play new animation if changed
        play_animation(
            &mut a_player,
            &mut transitions,
            &mut current_animation,
            animation_to_play,
            true,
            false,
        );

        syncronizer.animation_playing = *current_animation;
    }
}

/////////////////////////////////
//////////// Helpers ////////////
/////////////////////////////////
fn play_animation(
    a_player: &mut AnimationPlayer,
    transitions: &mut AnimationTransitions,
    current_animation: &mut AnimationNodeIndex,
    animation_to_play: AnimationNodeIndex,
    repeat: bool,
    force_now: bool,
) {
    if (*current_animation != animation_to_play) || force_now {
        if repeat {
            transitions
                .play(a_player, animation_to_play, Duration::from_secs_f32(0.2))
                .repeat();
        } else {
            transitions.play(a_player, animation_to_play, Duration::from_secs_f32(0.2));
        }
        *current_animation = animation_to_play;
    }
}

fn rotate_body_by_movement(body: &mut Transform, direction: Vec3, time: &Time) {
    if direction.length_squared() > 0.0 {
        let angle = -direction.z.atan2(direction.x);
        let rot = Quat::from_rotation_y(angle + std::f32::consts::PI / 2.0);
        body.rotation = body.rotation.slerp(rot, 10.0 * time.delta_secs());
    }
}

/////////////////////////////////////////
//////////// Check collision ////////////
/////////////////////////////////////////

fn on_ground_collision(
    trigger: On<CollisionStart>,
    ground_query: Query<(&Transform, AnyOf<(&Ground, &Recieved)>)>,
    player_transform: Single<&Transform, With<Player>>,
    mut player: Single<&mut Movement, With<Player>>,
) {
    let entity2 = trigger.collider2;

    // Check if colliding with ground
    if let Ok((ground_transform, _)) = ground_query.get(entity2) {
        // Only set grounded if ground is below player
        if ground_transform.translation.y <= player_transform.translation.y {
            player.is_grounded = true;
            player.current_jumps = 0;
        }
    }
}

fn cleanup_player(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
) {
    for entity in player_query.iter() {
        if let Ok(mut entity_mut) = commands.get_entity(entity) {
            entity_mut.despawn();
        }
    }
}
//
// fn check_grounded(
//     mut player: Single<(&Transform, &mut Movement), With<Player>>,
//     spatial_query: SpatialQuery,
// ) {
//     let (transform, mut movement) = player.into_inner();
//
//     let ray_origin = transform.translation;
//     let ray_direction = Dir3::NEG_Y;
//     let max_distance = 0.1; // Small distance to detect ground
//
//     if let Some(_hit) = spatial_query.cast_ray(
//         ray_origin,
//         ray_direction,
//         max_distance,
//         true,
//         &SpatialQueryFilter::default(),
//     ) {
//         movement.is_grounded = true;
//         movement.current_jumps = 0;
//     } else {
//         movement.is_grounded = false;
//     }
// }
