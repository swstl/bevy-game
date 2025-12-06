mod plugins;
mod components;

use avian3d::PhysicsPlugins;
use avian3d::prelude::PhysicsDebugPlugin;
use plugins::map::MapPlugin;
use plugins::player::PlayerPlugin;
use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use bevy::window::CursorOptions;
use plugins::network::MultiplayerPlugin;


fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    fit_canvas_to_parent: true, 
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            }),
            PhysicsDebugPlugin,
            PhysicsPlugins::default(),
            PlayerPlugin,
            MapPlugin,
            MultiplayerPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, grab_mouse)
        .run();
}

fn setup(
    mut commands: Commands,
) {
    // Add a light to illuminate the scene
    commands.spawn((
        Name::new("WorldLight"),
        PointLight {
            intensity: 200_000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}

// This system grabs the mouse when the left mouse button is pressed
// and releases it when the escape key / capslock is pressed
fn grab_mouse(
    mut cursor_options: Single<&mut CursorOptions>,
    mouse: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
) {
    if mouse.just_pressed(MouseButton::Left) 
    {
        cursor_options.visible = false;
        cursor_options.grab_mode = CursorGrabMode::Locked;
    }

    if key.just_pressed(KeyCode::Escape) ||
       key.just_pressed(KeyCode::CapsLock) 
    {
        cursor_options.visible = true;
        cursor_options.grab_mode = CursorGrabMode::None;
    }
}
