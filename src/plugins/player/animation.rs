use bevy::prelude::*;

use super::GLTF_PATH;

/////////////////////////////////
//////////// Startup ////////////
/////////////////////////////////
// A component that stores a reference to an animation we want to play.
// This will help prevent loading the animation multiple times
// https://bevy.org/examples/animation/animated-mesh/
#[derive(Component)]
struct PlayableAnimation {
    graph_handle: Handle<AnimationGraph>,
    index: AnimationNodeIndex,
}


pub fn load_animation(
    mut commands: Commands,
    ass: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
){

    let (graph, index) = AnimationGraph::from_clip(
        ass.load(GltfAssetLabel::Animation(0).from_asset(GLTF_PATH))
    );

    let graph_handle = graphs.add(graph);

    // store refrence:
    let playable_animation = PlayableAnimation {
        graph_handle,
        index
    };

    commands.spawn(playable_animation);
}





/////////////////////////////////
//////////// Updates ////////////
/////////////////////////////////

