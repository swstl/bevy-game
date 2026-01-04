//////////////////////////////////////////////////////////////
//////////////////////// Synchronizer ////////////////////////
//////////////////////////////////////////////////////////////
use super::Recieved;
use crate::plugins::GameLayer;
use super::resource::{LobbyInfo, WSMessageChannels, WSMessages};
use crate::components::entities::{LocalPlayer, PlayerBody};
use crate::components::vitals::Movement;
use crate::plugins::player::GLTF_PATH;
use crate::plugins::player::PLAYER_SCALE;
use crate::plugins::player::bundle::SimplePlayerBundle;
use avian3d::prelude::*;
use bevy::prelude::*;
use rand;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::Entry::Vacant;
use std::time::Duration;

const THRESHOLD: f32 = 0.1;

const POSITION_THRESHOLD: f32 = 0.5; // Start correcting at 0.5 units drift
const TELEPORT_THRESHOLD: f32 = 5.0; // Teleport if more than 5 units off
const CORRECTION_SPEED: f32 = 0.15; // How fast to lerp (0.0-1.0)
const IDLE_UPDATE_TIME: f32 = 0.2; // Time between idle updates

////////////////////////////////////////////////////////
//////////////////////// Define ////////////////////////
////////////////////////////////////////////////////////
#[derive(Component, Serialize, Deserialize, Debug, Clone)]
pub struct Synchronizer {
    pub id: i64,
    pub pos: Vec3,
    pub rot: Quat,
    pub vel: Vec3,
    pub speed: f32,
    pub jump: bool,
    pub animation_playing: AnimationNodeIndex,
}

impl Synchronizer {
    pub fn default() -> Self {
        Synchronizer {
            id: rand::random::<i64>(),
            pos: Vec3::default(),
            rot: Quat::default(),
            vel: Vec3::default(),
            speed: f32::default(),
            jump: false,
            animation_playing: AnimationNodeIndex::default(),
        }
    }

    fn sync(&self, channels: &WSMessageChannels) {
        let msg = WSMessages::Sync(self.to_bytes());

        if let Err(e) = channels.outgoing.send(msg) {
            eprintln!("Failed to send sync message: {:?}", e);
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(53);
        // id 8 bytes
        bytes.extend_from_slice(&self.id.to_le_bytes());

        // pos 12 bytes
        bytes.extend_from_slice(&self.pos.x.to_le_bytes());
        bytes.extend_from_slice(&self.pos.y.to_le_bytes());
        bytes.extend_from_slice(&self.pos.z.to_le_bytes());

        // rot 16 bytes
        bytes.extend_from_slice(&self.rot.x.to_le_bytes());
        bytes.extend_from_slice(&self.rot.y.to_le_bytes());
        bytes.extend_from_slice(&self.rot.z.to_le_bytes());
        bytes.extend_from_slice(&self.rot.w.to_le_bytes());

        // vel 12 bytes
        bytes.extend_from_slice(&self.vel.x.to_le_bytes());
        bytes.extend_from_slice(&self.vel.y.to_le_bytes());
        bytes.extend_from_slice(&self.vel.z.to_le_bytes());

        // speed 4 bytes
        bytes.extend_from_slice(&self.speed.to_le_bytes());

        // jump 1 byte
        bytes.push(self.jump as u8);

        // animation 4 bytes
        bytes.extend_from_slice(&self.animation_playing.index().to_le_bytes());

        bytes
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.len() < 36 {
            return Err("Not enough bytes");
        }

        Ok(Synchronizer {
            id: i64::from_le_bytes(bytes[0..8].try_into().unwrap()),
            pos: Vec3::new(
                f32::from_le_bytes(bytes[8..12].try_into().unwrap()),
                f32::from_le_bytes(bytes[12..16].try_into().unwrap()),
                f32::from_le_bytes(bytes[16..20].try_into().unwrap()),
            ),
            rot: Quat::from_xyzw(
                f32::from_le_bytes(bytes[20..24].try_into().unwrap()),
                f32::from_le_bytes(bytes[24..28].try_into().unwrap()),
                f32::from_le_bytes(bytes[28..32].try_into().unwrap()),
                f32::from_le_bytes(bytes[32..36].try_into().unwrap()),
            ),
            vel: Vec3::new(
                f32::from_le_bytes(bytes[36..40].try_into().unwrap()),
                f32::from_le_bytes(bytes[40..44].try_into().unwrap()),
                f32::from_le_bytes(bytes[44..48].try_into().unwrap()),
            ),
            speed: f32::from_le_bytes(bytes[48..52].try_into().unwrap()),
            jump: bytes[48] != 0,
            animation_playing: AnimationNodeIndex::new(u32::from_le_bytes(
                bytes[53..57].try_into().unwrap(),
            ) as usize),
        })
    }
}

///////////////////////////////////////////////////////////
///////////////// Handle outgoing traffic /////////////////
///////////////////////////////////////////////////////////
pub(crate) fn multiplayer_sender(
    query: Query<(&Transform, &Movement, &LinearVelocity, &mut Synchronizer), Without<Recieved>>,
    body: Single<&Transform, (With<PlayerBody>, Without<Recieved>)>,
    channels: Res<WSMessageChannels>,
) {
    for (transform, movement, velocity, mut syncronizer) in query {
        let mut changed = false;

        if syncronizer.pos.distance(transform.translation) > THRESHOLD {
            syncronizer.pos = transform.translation;
            changed = true;
        }

        if syncronizer.rot.angle_between(body.rotation).abs() > THRESHOLD {
            syncronizer.rot = body.rotation;
            changed = true;
        }

        if syncronizer.vel.distance(velocity.0) > THRESHOLD {
            syncronizer.vel = velocity.0;
            changed = true;
        }

        syncronizer.speed = movement.speed;

        if changed {
            syncronizer.sync(&channels);
        }
    }
}

////////////////////////////////////////////////////////////
///////////////// Handle incomming traffic /////////////////
////////////////////////////////////////////////////////////
pub(crate) fn handle_sync(
    mut channels: ResMut<WSMessageChannels>,
    mut lobby: ResMut<LobbyInfo>,
    mut commands: Commands,
    mut query: Query<(&mut Transform, &mut LinearVelocity, &mut Synchronizer), With<Recieved>>,
    mut ap: Query<(&mut AnimationPlayer, &mut AnimationTransitions), Without<LocalPlayer>>,
    children_query: Query<&Children>,
    ass: Res<AssetServer>,
) {
    while let Ok(msg) = channels.incomming.try_recv() {
        match msg {
            WSMessages::Sync(inc_bytes) => {
                let Ok(inc_sync) = Synchronizer::from_bytes(&inc_bytes) else {
                    eprintln!("Failed to decode sync message");
                    continue;
                };

                // spawn if new
                if let Vacant(e) = lobby.players.entry(inc_sync.id) {
                    let entity = spawn_online_player(&inc_sync, &mut commands, &ass);
                    e.insert(entity);
                }
                // update if exists
                else if let Some(&entity) = lobby.players.get(&inc_sync.id)
                    && let Ok((mut transform, mut velocity, mut synchronizer)) =
                        query.get_mut(entity)
                {
                    // Calculate position difference
                    let position_error = inc_sync.pos.distance(transform.translation);

                    // Position sync strategy
                    if position_error > TELEPORT_THRESHOLD {
                        // Large error: teleport immediately
                        transform.translation = inc_sync.pos;
                    } else if position_error > POSITION_THRESHOLD {
                        // Small error: smooth lerp correction
                        transform.translation =
                            transform.translation.lerp(inc_sync.pos, CORRECTION_SPEED);
                    }
                    // else: position is close enough, rely on velocity

                    // Always update rotation and velocity
                    transform.rotation = inc_sync.rot;
                    velocity.0 = inc_sync.vel;

                    // Update animation if changed
                    if synchronizer.animation_playing != inc_sync.animation_playing {
                        synchronizer.animation_playing = inc_sync.animation_playing;
                        find_and_play_animation(
                            entity,
                            inc_sync.animation_playing,
                            &children_query,
                            &mut ap,
                        );
                    }
                }
            }

            WSMessages::Connected(_inc_sync) => {}
            WSMessages::Disconnected(_inc_sync) => {}
            _ => {}
        }
    }
}

fn find_and_play_animation(
    entity: Entity,
    animation_index: AnimationNodeIndex,
    children_query: &Query<&Children>,
    animation_query: &mut Query<
        (&mut AnimationPlayer, &mut AnimationTransitions),
        Without<LocalPlayer>,
    >,
) {
    // Try current entity
    if let Ok((mut player, mut transitions)) = animation_query.get_mut(entity) {
        transitions
            .play(&mut player, animation_index, Duration::from_millis(250))
            .repeat();
        return;
    }

    // Recursively search children
    if let Ok(children) = children_query.get(entity) {
        for &child in children {
            find_and_play_animation(child, animation_index, children_query, animation_query);
        }
    }
}

fn spawn_online_player(
    inc: &Synchronizer,
    commands: &mut Commands,
    ass: &Res<AssetServer>,
) -> Entity {
    commands
        .spawn((
            Name::new("OnlinePlayer"),
            SimplePlayerBundle::new(),
            Synchronizer {
                id: inc.id,
                pos: inc.pos,
                rot: inc.rot,
                vel: inc.vel,
                speed: inc.speed,
                jump: inc.jump,
                animation_playing: inc.animation_playing,
            },
            Recieved,
            Visibility::default(),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Name::new("PlayerBody"),
                    PlayerBody,
                    Collider::cuboid(
                        1.75 * PLAYER_SCALE.x,
                        2.8 * PLAYER_SCALE.y,
                        1.0 * PLAYER_SCALE.z,
                    ),
                    CollisionLayers::new(
                        [GameLayer::OnlinePlayer], // I am OnlinePlayer
                        [GameLayer::Environment],  // I collide with Environment only
                    ),
                    Visibility::default(),
                    Recieved,
                ))
                .with_child((
                    SceneRoot(ass.load(GltfAssetLabel::Scene(0).from_asset(GLTF_PATH))),
                    Transform::from_scale(PLAYER_SCALE).with_translation(Vec3::new(
                        0.0,
                        -1.4 * PLAYER_SCALE.y,
                        0.0,
                    )),
                    Recieved,
                ));
        })
        .id()
}
