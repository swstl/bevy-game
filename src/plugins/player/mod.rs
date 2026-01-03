/////////////////////////////////
//////////// Imports ////////////
/////////////////////////////////
pub mod animation;
pub mod bundle;
pub mod camera;

use std::time::Duration;

use crate::components::entities::Player;
use crate::components::entities::PlayerBody;
use crate::components::objects::Ground;
use crate::components::vitals::Movement;
use crate::plugins::network::Recieved;
use crate::plugins::network::synchronizer::Synchronizer;
use crate::plugins::player::animation::AnimatedPlayer;
use animation::animate_meshes;
use animation::load_animation;
use avian3d::prelude::*;
use bevy::prelude::*;
use bundle::SimplePlayerBundle;

use camera::move_camera;
use camera::setup_camera;

const GLTF_PATH: &str = "character.glb";

///////////////////////////////////////
//////////// Player plugin ////////////
///////////////////////////////////////
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_player, setup_camera, load_animation))
            .add_systems(Update, (animate_meshes, move_player, move_camera));
    }
}



/////////////////////////////////
//////////// Startup ////////////
/////////////////////////////////
fn spawn_player(mut commands: Commands, ass: Res<AssetServer>) {
    let scale = Vec3::splat(0.3);

    commands
        .spawn((
            Name::new("LocalPlayer"),
            Synchronizer::default(),
            SimplePlayerBundle::new(),
            Player,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Name::new("PlayerBody"),
                    PlayerBody,
                    Collider::cuboid(1.75 * scale.x, 2.8 * scale.y, 1.0 * scale.z),
                    CollisionEventsEnabled,
                ))
                .with_child((
                    SceneRoot(ass.load(GltfAssetLabel::Scene(0).from_asset(GLTF_PATH))),
                    Transform::from_scale(scale).with_translation(Vec3::new(
                        0.0,
                        -1.4 * scale.y,
                        0.0,
                    )),
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
    Single<
    'w, 
    's, 
    &'static Transform, 
    (
        With<Camera>, 
        Without<Player>, 
        Without<PlayerBody>
    )
>;

type BodyQuery<'w, 's> = Single<
    'w, 
    's, 
    &'static mut Transform,
    (
        With<PlayerBody>,
        Without<Player>, 
        Without<Camera>
    )
>;

type PlayerQuery<'w, 's> = Single<
    'w,
    's,
    (
        &'static mut Transform,
        &'static mut LinearVelocity,
        &'static mut Movement,
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
    mut animation_players: Query<(&mut AnimationPlayer, &mut AnimationTransitions), With<AnimatedPlayer>>,
    mut current_animation: Local<AnimationNodeIndex>,
    animations: Res<animation::PlayerAnimations>,
) {
    let (mut transform, mut velocity, mut player) = player.into_inner();
    for (mut a_player, mut transitions) in &mut animation_players {

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

        if keyboard.any_just_pressed([KeyCode::Space])
            && player.can_jump()
        {
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

        rotate_body_by_movement(
            &mut body,
            direction,
            &time
        );

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
            .play(
                a_player, 
                animation_to_play, 
                Duration::from_secs_f32(0.2)
            ).repeat();
        } else {
            transitions
            .play(
                a_player, 
                animation_to_play, 
                Duration::from_secs_f32(0.2)
            );
        }
        *current_animation = animation_to_play;
    }
}

fn rotate_body_by_movement(
    body: &mut Transform,
    direction: Vec3,
    time: &Time,
) {
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
    event: On<CollisionStart>,
    ground_query: Query<AnyOf<(&Ground, &Recieved)>>,
    mut player: Single<&mut Movement, With<Player>>,
) {
    if ground_query.get(event.collider2).is_ok() {
        player.is_grounded = true;
        player.current_jumps = 0;
    }
}
