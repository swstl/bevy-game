use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::network::{Recieved, resource::{LobbyInfo, WSMessageChannels, WSMessages}};
use crate::plugins::player::bundle::PlayerBundle;
use rand;

const THRESHOLD: f32 = 0.1;

#[derive(Component, Serialize, Deserialize, Debug, Clone)]
pub struct Synchronizer{
    id: i64,
    pos: Vec3,
    rot: Quat
}

impl Synchronizer {
    pub fn default() -> Self{
        Synchronizer{
            id: rand::random::<i64>(),
            pos: Vec3::default(),
            rot: Quat::default()
        }
    }

    fn sync(&self, channels: &WSMessageChannels){

        let msg = WSMessages::Sync(self.to_bytes());

        if let Err(e) = channels.outgoing.send(msg){
            eprintln!("Failed to send sync message: {:?}", e);
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(36);
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
        })
    }
}

pub (crate) fn multiplayer_sender(
    query: Query<(&Transform, &mut Synchronizer), Without<Recieved>>,
    channels: Res<WSMessageChannels>
){
    for (transform, mut syncronizer) in query {
        let mut changed = false;

        if syncronizer.pos.distance(transform.translation) > THRESHOLD {
            syncronizer.pos = transform.translation;
            changed = true;
        }

        if syncronizer.rot.angle_between(transform.rotation).abs() > THRESHOLD {
            syncronizer.rot = transform.rotation;
            changed = true;
        }

        //TODO: this 
        if changed {
            syncronizer.sync(&channels);
        }
    }
}


// TODO: spawn player if not exist and sync based on the id for that player
// the recieved objects might not even have synchronizer on it btw
pub (crate) fn handle_sync(
    mut channels: ResMut<WSMessageChannels>,
    mut lobby: ResMut<LobbyInfo>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(&mut Transform, &Synchronizer), With<Recieved>>,
){
    let mut changed = false;

    // spawn new entities if there are some
    while let Ok(msg) = channels.incomming.try_recv(){
        match msg {
            WSMessages::Sync(inc_sync) => {
                let inc_sync = match Synchronizer::from_bytes(&inc_sync){
                    Ok(s) => s,
                    Err(e) => {
                        eprintln!("Failed to decode sync message: {}", e);
                        continue;
                    }
                };

                if !lobby.connected_players.contains_key(&inc_sync.id){
                    commands.spawn((
                        PlayerBundle::new(
                            meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
                            materials.add(Color::srgb(0.8, 0.3, 1.0)),
                            inc_sync.pos
                        ),
                        // this also acts as their spawn positions
                        Synchronizer {
                            id: inc_sync.id,
                            pos: inc_sync.pos,
                            rot: inc_sync.rot,
                        },
                        Recieved,
                    ));
                }

                lobby.connected_players.insert(inc_sync.id, inc_sync);

                changed = true;
            }

            WSMessages::Connected => {

            }

            WSMessages::Disconnected => {

            }
            _ => {}
        }
    }

    // move the entities to new positions:
    if !changed { return }
    for (mut transform, synchronizer) in query {
        let Some(sync_data) = lobby.connected_players.get(&synchronizer.id) else {
            continue
        };

        transform.translation = sync_data.pos;
        transform.rotation = sync_data.rot;
    }
}
