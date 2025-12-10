//////////////////////////////////////////
//////////// Player animation ////////////
//////////////////////////////////////////
use std::time::Duration;
use bevy::prelude::*;
use super::PlayerBody;
use super::GLTF_PATH;

const NUMBER_OF_ANIMATIONS: usize = 18;
#[derive(Component)]
pub struct AnimatedPlayer;

// A resource that stores a reference to an animation we want to play.
// This will help prevent loading the animation multiple times
/// this stores the refrence to the animation that can be played 
/// for the object or entity
// https://bevy.org/examples/animation/animated-mesh/
#[derive(Resource)]
pub struct PlayerAnimations{
    pub graph_handle: Handle<AnimationGraph>,
    pub die: AnimationNodeIndex,
    pub duck: AnimationNodeIndex,
    pub hitreact: AnimationNodeIndex,
    pub idle: AnimationNodeIndex,
    pub idlegun: AnimationNodeIndex,
    pub idleshoot: AnimationNodeIndex,
    pub jump: AnimationNodeIndex,
    pub jumpidle: AnimationNodeIndex,
    pub jumpland: AnimationNodeIndex,
    pub no: AnimationNodeIndex,
    pub punch: AnimationNodeIndex,
    pub run: AnimationNodeIndex,
    pub rungun: AnimationNodeIndex,
    pub runshoot: AnimationNodeIndex,
    pub walk: AnimationNodeIndex,
    pub walkgun: AnimationNodeIndex,
    pub wave: AnimationNodeIndex,
    pub yes: AnimationNodeIndex,
}



/////////////////////////////////
//////////// Startup ////////////
/////////////////////////////////
pub fn load_animation(
    mut commands: Commands,
    ass: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
){
    let available_animations: Vec<_> = (0..NUMBER_OF_ANIMATIONS)
        .map(|i| ass.load(GltfAssetLabel::Animation(i).from_asset(GLTF_PATH)))
        .collect();

    let (graph, indices) = AnimationGraph::from_clips(available_animations);

    let graph_handle = graphs.add(graph);

    // store refrence:
    commands.insert_resource(
        PlayerAnimations {
            graph_handle,
            die: indices[0],
            duck: indices[1],
            hitreact: indices[2],
            idle: indices[3],
            idlegun: indices[4],
            idleshoot: indices[5],
            jump: indices[6],
            jumpidle: indices[7],
            jumpland: indices[8],
            no: indices[9],
            punch: indices[10],
            run: indices[11],
            rungun: indices[12],
            runshoot: indices[13],
            walk: indices[14],
            walkgun: indices[15],
            wave: indices[16],
            yes: indices[17],
        }
    )
}



/////////////////////////////////
//////////// Updates ////////////
/////////////////////////////////

pub fn animate_meshes(
    mut commands: Commands,
    animations: Res<PlayerAnimations>,
    mut a_players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
    bodies: Query<(), With<PlayerBody>>,
    hierarchy: Query<&ChildOf>,
){
    for (entity, mut a_player) in &mut a_players {
        let mut current = entity;
        let mut found_player_body = false;

        // traverse up to find the playerbody (if any)
        while let Ok(child_of) = hierarchy.get(current) {
            if bodies.contains(child_of.0) {
                found_player_body = true;
                break;
            }
            current = child_of.0;
        }

        // only add animation to player bodies
        if found_player_body {
            let mut transitions = AnimationTransitions::new();

            // starts the idle animation
            transitions
                .play(&mut a_player, animations.idle, Duration::ZERO)
                .repeat();

            commands
                .entity(entity)
                .insert(AnimationGraphHandle(animations.graph_handle.clone()))
                .insert(transitions)
                .insert(AnimatedPlayer);
        }
    }
}

