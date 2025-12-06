/////////////////////////////////
//////////// Imports ////////////
/////////////////////////////////
use bevy::prelude::*;
use bevy::input::mouse::AccumulatedMouseMotion;




//////////////////////////////////////
//////////// Camera Setup ////////////
//////////////////////////////////////
#[derive(Debug, Resource)]
pub struct CameraSettings {
    pub camera_distance: f32,
    pub sensitivity: f32,
}

pub fn setup_camera(
    mut commands: Commands,
) {
    commands.insert_resource(CameraSettings {
        camera_distance: 8.0,
        sensitivity: 0.004,
    });
}




////////////////////////////////////////////////
//////////// Controlling the camera ////////////
////////////////////////////////////////////////
pub fn move_camera(
    mut camera: Single<&mut Transform, With<Camera>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    settings: Res<CameraSettings>,
) {
    let delta = mouse_motion.delta;

    let delta_yaw = delta.x * settings.sensitivity;
    let delta_pitch = -delta.y * settings.sensitivity;

    // Obtain the existing pitch, yaw, and roll values from the transform.
    let (yaw, pitch, roll) = camera.rotation.to_euler(EulerRot::YXZ);
    let yaw = yaw + delta_yaw;
    let pitch = (pitch + delta_pitch).clamp(-1.54, 1.54); // Prevent flipping at zenith/nadir
    camera.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);

    // Adjust the translation to maintain the correct orientation toward the orbit target.
    let target = Vec3::ZERO;
    camera.translation = target - camera.forward() * settings.camera_distance;
}

