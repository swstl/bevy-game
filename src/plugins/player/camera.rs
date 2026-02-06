/////////////////////////////////
//////////// Imports ////////////
/////////////////////////////////
use bevy::prelude::*;
use bevy::input::mouse::AccumulatedMouseMotion;

use crate::plugins::menu::GameSettings;




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
    game_settings: Res<GameSettings>,
    time: Res<Time>,
) {
    let delta = mouse_motion.delta;

    let delta_yaw = -delta.x * settings.sensitivity;
    let delta_pitch = -delta.y * settings.sensitivity;

    // Obtain the existing pitch, yaw, and roll values from the transform.
    let (yaw, pitch, _roll) = camera.rotation.to_euler(EulerRot::YXZ);
    let yaw = yaw + delta_yaw;
    let pitch = (pitch + delta_pitch).clamp(-1.54, 1.54); // Prevent flipping at zenith/nadir

    // Apply world tilt setting as camera roll
    let tilt = game_settings.world_tilt.to_radians();

    // Apply drunk mode as sinusoidal wobble
    let drunk = game_settings.drunk_mode / 100.0;
    let t = time.elapsed_secs();
    let drunk_roll = drunk * 0.15 * (t * 1.7).sin();
    let drunk_yaw_offset = drunk * 0.05 * (t * 1.3).cos();
    let drunk_pitch_offset = drunk * 0.03 * (t * 2.1).sin();

    camera.rotation = Quat::from_euler(
        EulerRot::YXZ,
        yaw + drunk_yaw_offset,
        pitch + drunk_pitch_offset,
        tilt + drunk_roll,
    );

    // Adjust the translation to maintain the correct orientation toward the orbit target.
    let target = Vec3::ZERO;

    // Apply camera shake
    let shake = game_settings.camera_shake / 100.0;
    let shake_offset = if shake > 0.0 {
        Vec3::new(
            shake * 0.1 * (t * 25.0).sin(),
            shake * 0.1 * (t * 31.0).cos(),
            shake * 0.05 * (t * 19.0).sin(),
        )
    } else {
        Vec3::ZERO
    };

    camera.translation = target - camera.forward() * settings.camera_distance + shake_offset;
}

