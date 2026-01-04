/////////////////////////////////
//////////// Imports ////////////
/////////////////////////////////
use bevy::prelude::*;
use avian3d::prelude::*;
use crate::components::vitals::Movement;


/////////////////////////////////////////////////
//////////// Implement player bundle ////////////
/////////////////////////////////////////////////
#[derive(Bundle)]
pub struct PlayerBundle {
    mesh: Mesh3d,
    material: MeshMaterial3d<StandardMaterial>,
    transform: Transform,
    rigid_body: RigidBody,
    // friction: Friction,
    locked_axes: LockedAxes,
    collider: Collider,
    collision_events: CollisionEventsEnabled,
    movement: Movement,
}

impl PlayerBundle {
    pub fn new(
        mesh: Handle<Mesh>,
        material: Handle<StandardMaterial>,
        position: Vec3,
    ) -> Self {
        Self {
            mesh: Mesh3d(mesh),
            material: MeshMaterial3d(material),
            transform: Transform::from_translation(position),
            rigid_body: RigidBody::Dynamic,
            // friction: Friction::ZERO,
            locked_axes: LockedAxes::ROTATION_LOCKED,
            collider: Collider::cuboid(1.0, 1.0, 1.0),
            collision_events: CollisionEventsEnabled,
            movement: Movement {
                speed: 100.0,
                sprint_aplifier: 3.0,
                jump_strength: 7.0,
                is_grounded: false,
                extra_jumps: 2,
                current_jumps: 0,
            },
        }
    }
}




//////////////////////////////////////////////
//////////// simple player bundle ////////////
//////////////////////////////////////////////

#[derive(Bundle)]
pub struct SimplePlayerBundle {
    transform: Transform,
    rigid_body: RigidBody,
    // friction: Friction,
    locked_axes: LockedAxes,
    movement: Movement,
}

impl SimplePlayerBundle {
    pub fn new() -> Self {
        Self {
            transform: Transform::from_translation(Vec3::new(
                10.0, 10.0, 10.0
            )),
            rigid_body: RigidBody::Dynamic,
            // friction: Friction::ZERO,
            locked_axes: LockedAxes::ROTATION_LOCKED,
            movement: Movement {
                speed: 100.0,
                sprint_aplifier: 3.0,
                jump_strength: 6.0,
                is_grounded: false,
                extra_jumps: 2,
                current_jumps: 0,
            },
        }
    }
}
