//////////////////////////////////////////////////////////////
//////////////////////// Synchronizer ////////////////////////
//////////////////////////////////////////////////////////////
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use super::Recieved; 
use super::resource::{LobbyInfo, WSMessageChannels, WSMessages};
use crate::plugins::player::bundle::PlayerBundle;
use std::collections::hash_map::Entry::Vacant;
use rand;

const THRESHOLD: f32 = 0.1;


////////////////////////////////////////////////////////
//////////////////////// Define ////////////////////////
////////////////////////////////////////////////////////
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





///////////////////////////////////////////////////////////
///////////////// Handle outgoing traffic /////////////////
///////////////////////////////////////////////////////////
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







////////////////////////////////////////////////////////////
///////////////// Handle incomming traffic /////////////////
////////////////////////////////////////////////////////////
pub (crate) fn handle_sync(
    mut channels: ResMut<WSMessageChannels>,
    mut lobby: ResMut<LobbyInfo>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<&mut Transform, With<Recieved>>,
){

    // spawn new entities if there are some
    while let Ok(msg) = channels.incomming.try_recv(){
        match msg {
            WSMessages::Sync(inc_sync) => {
                let Ok(inc_sync) =  Synchronizer::from_bytes(&inc_sync)
                else {
                    eprintln!("Failed to decode sync message");
                    continue;
                };

                // spawn if new
                if let Vacant(e) = lobby.players.entry(inc_sync.id) {
                    let entity = spawn_online_player(
                        &inc_sync,
                        &mut commands,
                        &mut meshes,
                        &mut materials
                    );
                    e.insert(entity);
                }

                // move if not
                else if let Some(&entity) = lobby.players.get(&inc_sync.id) 
                && let Ok(mut transform) = query.get_mut(entity)
                {
                    transform.translation = inc_sync.pos;
                    transform.rotation = inc_sync.rot;
                }
            }

            WSMessages::Connected(_inc_sync) => {
            }

            WSMessages::Disconnected(_inc_sync) => {

            }
            _ => {}
        }
    }
}

fn spawn_online_player(
    inc: &Synchronizer,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>
) -> Entity {
    commands.spawn((
        PlayerBundle::new(
            meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            materials.add(Color::srgb(0.8, 0.3, 1.0)),
            inc.pos
        ),
        // this also acts as their spawn positions
        Synchronizer {
            id: inc.id,
            pos: inc.pos,
            rot: inc.rot,
        },
        Recieved,
    )).id()
}
