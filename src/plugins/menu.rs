use bevy::ecs::hierarchy::ChildSpawnerCommands;
use bevy::prelude::*;
use rand::Rng;

use crate::components::entities::Player;
use crate::plugins::player::camera::CameraSettings;

/////////////////////////////////
////////// Game States //////////
/////////////////////////////////

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Menu,
    Playing,
}

/// Tracks that the player has been spawned at least once.
#[derive(Resource, Default)]
pub struct HasPlayed;

/////////////////////////////////
////////// Game Settings ////////
/////////////////////////////////

#[derive(Resource)]
pub struct GameSettings {
    pub fov: f32,
    pub camera_distance: f32,
    pub mouse_sensitivity: f32,
    pub camera_shake: f32,
    // weird stuff
    pub hue_shift: f32,
    pub world_tilt: f32,
    pub gravity_multiplier: f32,
    pub player_size: f32,
    pub drunk_mode: f32,
    pub time_scale: f32,
    pub chromatic_aberration: f32,
    pub big_head_mode: f32,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            fov: 75.0,
            camera_distance: 8.0,
            mouse_sensitivity: 4.0,
            camera_shake: 0.0,
            hue_shift: 0.0,
            world_tilt: 0.0,
            gravity_multiplier: 100.0,
            player_size: 100.0,
            drunk_mode: 0.0,
            time_scale: 100.0,
            chromatic_aberration: 0.0,
            big_head_mode: 100.0,
        }
    }
}

/////////////////////////////////
////////// Setting Kinds ////////
/////////////////////////////////

#[derive(Component, Clone, Copy, PartialEq)]
enum SettingKind {
    Fov,
    CameraDistance,
    MouseSensitivity,
    CameraShake,
    HueShift,
    WorldTilt,
    Gravity,
    PlayerSize,
    DrunkMode,
    TimeScale,
    ChromaticAberration,
    BigHeadMode,
}

const NORMAL_SETTINGS: &[SettingKind] = &[
    SettingKind::Fov,
    SettingKind::CameraDistance,
    SettingKind::MouseSensitivity,
    SettingKind::CameraShake,
];

const WEIRD_SETTINGS: &[SettingKind] = &[
    SettingKind::HueShift,
    SettingKind::WorldTilt,
    SettingKind::Gravity,
    SettingKind::PlayerSize,
    SettingKind::DrunkMode,
    SettingKind::TimeScale,
    SettingKind::ChromaticAberration,
    SettingKind::BigHeadMode,
];

impl SettingKind {
    fn label(&self) -> &'static str {
        match self {
            Self::Fov => "FOV",
            Self::CameraDistance => "Camera Distance",
            Self::MouseSensitivity => "Mouse Sensitivity",
            Self::CameraShake => "Camera Shake",
            Self::HueShift => "Hue Shift",
            Self::WorldTilt => "World Tilt",
            Self::Gravity => "Gravity",
            Self::PlayerSize => "Player Size",
            Self::DrunkMode => "Drunk Mode",
            Self::TimeScale => "Time Scale",
            Self::ChromaticAberration => "Chromatic Aberration",
            Self::BigHeadMode => "Big Head Mode",
        }
    }

    fn min(&self) -> f32 {
        match self {
            Self::Fov => 30.0,
            Self::CameraDistance => 2.0,
            Self::MouseSensitivity => 0.5,
            Self::CameraShake => 0.0,
            Self::HueShift => 0.0,
            Self::WorldTilt => -45.0,
            Self::Gravity => 10.0,
            Self::PlayerSize => 10.0,
            Self::DrunkMode => 0.0,
            Self::TimeScale => 10.0,
            Self::ChromaticAberration => 0.0,
            Self::BigHeadMode => 100.0,
        }
    }

    fn max(&self) -> f32 {
        match self {
            Self::Fov => 170.0,
            Self::CameraDistance => 50.0,
            Self::MouseSensitivity => 20.0,
            Self::CameraShake => 100.0,
            Self::HueShift => 360.0,
            Self::WorldTilt => 45.0,
            Self::Gravity => 500.0,
            Self::PlayerSize => 500.0,
            Self::DrunkMode => 100.0,
            Self::TimeScale => 300.0,
            Self::ChromaticAberration => 100.0,
            Self::BigHeadMode => 500.0,
        }
    }

    fn step(&self) -> f32 {
        match self {
            Self::Fov => 5.0,
            Self::CameraDistance => 1.0,
            Self::MouseSensitivity => 0.5,
            Self::CameraShake => 5.0,
            Self::HueShift => 15.0,
            Self::WorldTilt => 5.0,
            Self::Gravity => 10.0,
            Self::PlayerSize => 10.0,
            Self::DrunkMode => 5.0,
            Self::TimeScale => 10.0,
            Self::ChromaticAberration => 5.0,
            Self::BigHeadMode => 25.0,
        }
    }

    fn get(&self, s: &GameSettings) -> f32 {
        match self {
            Self::Fov => s.fov,
            Self::CameraDistance => s.camera_distance,
            Self::MouseSensitivity => s.mouse_sensitivity,
            Self::CameraShake => s.camera_shake,
            Self::HueShift => s.hue_shift,
            Self::WorldTilt => s.world_tilt,
            Self::Gravity => s.gravity_multiplier,
            Self::PlayerSize => s.player_size,
            Self::DrunkMode => s.drunk_mode,
            Self::TimeScale => s.time_scale,
            Self::ChromaticAberration => s.chromatic_aberration,
            Self::BigHeadMode => s.big_head_mode,
        }
    }

    fn set(&self, s: &mut GameSettings, val: f32) {
        let clamped = val.clamp(self.min(), self.max());
        match self {
            Self::Fov => s.fov = clamped,
            Self::CameraDistance => s.camera_distance = clamped,
            Self::MouseSensitivity => s.mouse_sensitivity = clamped,
            Self::CameraShake => s.camera_shake = clamped,
            Self::HueShift => s.hue_shift = clamped,
            Self::WorldTilt => s.world_tilt = clamped,
            Self::Gravity => s.gravity_multiplier = clamped,
            Self::PlayerSize => s.player_size = clamped,
            Self::DrunkMode => s.drunk_mode = clamped,
            Self::TimeScale => s.time_scale = clamped,
            Self::ChromaticAberration => s.chromatic_aberration = clamped,
            Self::BigHeadMode => s.big_head_mode = clamped,
        }
    }

    fn format_value(&self, val: f32) -> String {
        match self {
            Self::Fov => format!("{}°", val as i32),
            Self::CameraDistance => format!("{:.0}", val),
            Self::MouseSensitivity => format!("{:.1}", val),
            Self::CameraShake
            | Self::DrunkMode
            | Self::ChromaticAberration => format!("{}%", val as i32),
            Self::HueShift => format!("{}°", val as i32),
            Self::WorldTilt => format!("{}°", val as i32),
            Self::Gravity
            | Self::PlayerSize
            | Self::TimeScale
            | Self::BigHeadMode => format!("{}%", val as i32),
        }
    }
}

/////////////////////////////////
////////// UI Components ////////
/////////////////////////////////

#[derive(Component)]
struct MainMenuUI;

#[derive(Component)]
struct SettingsUI;

#[derive(Component)]
struct MenuCamera;

#[derive(Component)]
enum MenuButton {
    Play,
    Settings,
    ResetCharacter,
}

#[derive(Component)]
struct SettingValueDisplay(SettingKind);

#[derive(Component)]
struct SettingAdjustButton {
    kind: SettingKind,
    delta: f32,
}

#[derive(Component)]
struct BackButton;

#[derive(Component)]
struct ResetButton;

/////////////////////////////////
///////// Menu Plugin ///////////
/////////////////////////////////

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .init_resource::<GameSettings>()
            .add_systems(OnEnter(GameState::Menu), setup_menu)
            .add_systems(OnExit(GameState::Menu), cleanup_menu)
            .add_systems(
                Update,
                (handle_menu_buttons, handle_settings_buttons)
                    .run_if(in_state(GameState::Menu)),
            )
            .add_systems(Update, apply_settings)
            .add_systems(
                Update,
                handle_escape_to_menu.run_if(in_state(GameState::Playing)),
            );
    }
}

/////////////////////////////////
///////// Menu Setup ////////////
/////////////////////////////////

const BG_COLOR: Color = Color::srgb(0.1, 0.1, 0.12);
const BTN_COLOR: Color = Color::srgb(0.2, 0.2, 0.25);
const BTN_HOVER: Color = Color::srgb(0.3, 0.3, 0.35);
const BTN_PRESS: Color = Color::srgb(0.4, 0.4, 0.5);
const OUTLINE_COLOR: Color = Color::srgb(0.4, 0.4, 0.5);
const GOLD: Color = Color::srgb(0.9, 0.75, 0.3);
const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const DIM_TEXT: Color = Color::srgb(0.5, 0.5, 0.55);
const LABEL_COLOR: Color = Color::srgb(0.7, 0.7, 0.75);
const SECTION_COLOR: Color = Color::srgb(0.8, 0.4, 0.6);

fn setup_menu(mut commands: Commands, has_played: Option<Res<HasPlayed>>) {
    let is_resuming = has_played.is_some();

    commands.spawn((MenuCamera, Camera2d));

    commands
        .spawn((
            MainMenuUI,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(BG_COLOR),
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("WTF"),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(GOLD),
                Node {
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                },
            ));

            // Subtitle
            parent.spawn((
                Text::new("Definitely Not A Placeholder Title"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(DIM_TEXT),
                Node {
                    margin: UiRect::bottom(Val::Px(50.0)),
                    ..default()
                },
            ));

            let play_text = if is_resuming { "RESUME" } else { "PLAY" };
            spawn_menu_button(parent, MenuButton::Play, play_text);
            spawn_menu_button(parent, MenuButton::Settings, "SETTINGS");
            spawn_menu_button(parent, MenuButton::ResetCharacter, "RESET CHARACTER");
        });
}

fn spawn_menu_button(parent: &mut ChildSpawnerCommands, button: MenuButton, label: &str) {
    parent
        .spawn((
            Button,
            button,
            Node {
                width: Val::Px(250.0),
                height: Val::Px(65.0),
                margin: UiRect::all(Val::Px(10.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(BTN_COLOR),
            Outline {
                width: Val::Px(2.0),
                offset: Val::Px(0.0),
                color: OUTLINE_COLOR,
            },
            BorderRadius::all(Val::Px(8.0)),
        ))
        .with_child((
            Text::new(label),
            TextFont {
                font_size: 24.0,
                ..default()
            },
            TextColor(TEXT_COLOR),
        ));
}

/////////////////////////////////
////// Settings Screen //////////
/////////////////////////////////

// Approximate size of each setting widget for collision detection
const SETTING_W: f32 = 240.0;
const SETTING_H: f32 = 62.0;
const BACK_W: f32 = 200.0;
const BACK_H: f32 = 50.0;
const TITLE_W: f32 = 250.0;
const TITLE_H: f32 = 50.0;

struct PlacedRect {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

impl PlacedRect {
    fn overlaps(&self, other: &PlacedRect, padding: f32) -> bool {
        self.x - padding < other.x + other.w
            && self.x + self.w + padding > other.x
            && self.y - padding < other.y + other.h
            && self.y + self.h + padding > other.y
    }
}

fn random_position(
    rng: &mut impl Rng,
    w: f32,
    h: f32,
    placed: &[PlacedRect],
    padding: f32,
) -> (f32, f32) {
    // area we can place in (percentage-based, 0..100 mapped to screen)
    let max_x = 100.0 - w;
    let max_y = 100.0 - h;
    for _ in 0..200 {
        let x = rng.random_range(1.0..max_x.max(2.0));
        let y = rng.random_range(1.0..max_y.max(2.0));
        let candidate = PlacedRect { x, y, w, h };
        if !placed.iter().any(|p| candidate.overlaps(p, padding)) {
            return (x, y);
        }
    }
    // fallback: just pick something
    (rng.random_range(1.0..max_x.max(2.0)), rng.random_range(1.0..max_y.max(2.0)))
}

fn spawn_settings_screen(commands: &mut Commands, settings: &GameSettings) {
    let mut rng = rand::rng();
    let mut placed: Vec<PlacedRect> = Vec::new();

    // All settings to place
    let all_settings: Vec<SettingKind> = NORMAL_SETTINGS
        .iter()
        .chain(WEIRD_SETTINGS.iter())
        .copied()
        .collect();

    // Widget sizes in percentage of screen (rough estimates assuming ~1200x800)
    let sw = SETTING_W / 12.0; // ~20% of screen width
    let sh = SETTING_H / 8.0; // ~7.75% of screen height
    let bw = BACK_W / 12.0;
    let bh = BACK_H / 8.0;
    let tw = TITLE_W / 12.0;
    let th = TITLE_H / 8.0;

    // Reserve a spot for the title (top center-ish)
    let title_x = 50.0 - tw / 2.0 + rng.random_range(-10.0..10.0);
    let title_y = rng.random_range(1.0..8.0);
    placed.push(PlacedRect {
        x: title_x,
        y: title_y,
        w: tw,
        h: th,
    });

    // Generate positions for each setting
    let mut positions: Vec<(f32, f32)> = Vec::new();
    for _ in &all_settings {
        let (x, y) = random_position(&mut rng, sw, sh, &placed, 1.0);
        placed.push(PlacedRect {
            x,
            y,
            w: sw,
            h: sh,
        });
        positions.push((x, y));
    }

    // Generate position for back button and reset button
    let (back_x, back_y) = random_position(&mut rng, bw, bh, &placed, 1.0);
    placed.push(PlacedRect { x: back_x, y: back_y, w: bw, h: bh });
    let (reset_x, reset_y) = random_position(&mut rng, bw, bh, &placed, 1.0);

    // Generate a random rotation for each setting (-8 to 8 degrees)
    let rotations: Vec<f32> = all_settings
        .iter()
        .map(|_| rng.random_range(-8.0_f32..8.0).to_radians())
        .collect();

    commands
        .spawn((
            SettingsUI,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Relative,
                ..default()
            },
            BackgroundColor(BG_COLOR),
        ))
        .with_children(|parent| {
            // Title - placed at its reserved spot
            parent.spawn((
                Text::new("SETTINGS"),
                TextFont {
                    font_size: 36.0,
                    ..default()
                },
                TextColor(GOLD),
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(title_x),
                    top: Val::Percent(title_y),
                    ..default()
                },
            ));

            // Spawn each setting at its random position
            for (i, kind) in all_settings.iter().enumerate() {
                let (x, y) = positions[i];
                let rot = rotations[i];
                spawn_setting_row_scattered(parent, *kind, settings, x, y, rot);
            }

            // Back button at random position
            parent
                .spawn((
                    Button,
                    BackButton,
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Percent(back_x),
                        top: Val::Percent(back_y),
                        width: Val::Px(BACK_W),
                        height: Val::Px(BACK_H),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(BTN_COLOR),
                    Outline {
                        width: Val::Px(2.0),
                        offset: Val::Px(0.0),
                        color: OUTLINE_COLOR,
                    },
                    BorderRadius::all(Val::Px(8.0)),
                    Transform::from_rotation(Quat::from_rotation_z(
                        rng.random_range(-5.0_f32..5.0).to_radians(),
                    )),
                ))
                .with_child((
                    Text::new("BACK"),
                    TextFont {
                        font_size: 22.0,
                        ..default()
                    },
                    TextColor(TEXT_COLOR),
                ));

            // Reset button at random position
            parent
                .spawn((
                    Button,
                    ResetButton,
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Percent(reset_x),
                        top: Val::Percent(reset_y),
                        width: Val::Px(BACK_W),
                        height: Val::Px(BACK_H),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.3, 0.15, 0.15)),
                    Outline {
                        width: Val::Px(2.0),
                        offset: Val::Px(0.0),
                        color: Color::srgb(0.6, 0.3, 0.3),
                    },
                    BorderRadius::all(Val::Px(8.0)),
                    Transform::from_rotation(Quat::from_rotation_z(
                        rng.random_range(-5.0_f32..5.0).to_radians(),
                    )),
                ))
                .with_child((
                    Text::new("RESET"),
                    TextFont {
                        font_size: 22.0,
                        ..default()
                    },
                    TextColor(Color::srgb(1.0, 0.5, 0.5)),
                ));
        });
}

fn spawn_setting_row_scattered(
    parent: &mut ChildSpawnerCommands,
    kind: SettingKind,
    settings: &GameSettings,
    x_pct: f32,
    y_pct: f32,
    rotation: f32,
) {
    let value = kind.get(settings);

    parent
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(x_pct),
                top: Val::Percent(y_pct),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                width: Val::Px(SETTING_W),
                ..default()
            },
            Transform::from_rotation(Quat::from_rotation_z(rotation)),
        ))
        .with_children(|row| {
            // Label
            row.spawn((
                Text::new(kind.label()),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(LABEL_COLOR),
                Node {
                    margin: UiRect::bottom(Val::Px(4.0)),
                    ..default()
                },
            ));

            // Controls: [ < ]  value  [ > ]
            row.spawn(Node {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                width: Val::Percent(100.0),
                ..default()
            })
            .with_children(|controls| {
                spawn_adjust_btn(controls, kind, -kind.step(), "<");

                controls.spawn((
                    Text::new(kind.format_value(value)),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(TEXT_COLOR),
                    TextLayout::new_with_justify(Justify::Center),
                    SettingValueDisplay(kind),
                    Node {
                        width: Val::Px(120.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                ));

                spawn_adjust_btn(controls, kind, kind.step(), ">");
            });
        });
}

fn spawn_adjust_btn(
    parent: &mut ChildSpawnerCommands,
    kind: SettingKind,
    delta: f32,
    label: &str,
) {
    parent
        .spawn((
            Button,
            SettingAdjustButton { kind, delta },
            Node {
                width: Val::Px(40.0),
                height: Val::Px(36.0),
                margin: UiRect::horizontal(Val::Px(4.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(BTN_COLOR),
            BorderRadius::all(Val::Px(6.0)),
            Outline {
                width: Val::Px(1.0),
                offset: Val::Px(0.0),
                color: OUTLINE_COLOR,
            },
        ))
        .with_child((
            Text::new(label),
            TextFont {
                font_size: 20.0,
                ..default()
            },
            TextColor(TEXT_COLOR),
        ));
}

/////////////////////////////////
////////// Cleanup //////////////
/////////////////////////////////

fn cleanup_menu(
    mut commands: Commands,
    menu_query: Query<Entity, With<MainMenuUI>>,
    settings_query: Query<Entity, With<SettingsUI>>,
    camera_query: Query<Entity, With<MenuCamera>>,
) {
    for entity in menu_query.iter() {
        commands.entity(entity).despawn();
    }
    for entity in settings_query.iter() {
        commands.entity(entity).despawn();
    }
    for entity in camera_query.iter() {
        commands.entity(entity).despawn();
    }
}

/////////////////////////////////
/////// Button Interaction //////
/////////////////////////////////

fn handle_menu_buttons(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &MenuButton, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    mut next_state: ResMut<NextState<GameState>>,
    player_query: Query<Entity, With<Player>>,
    menu_query: Query<Entity, With<MainMenuUI>>,
    settings: Res<GameSettings>,
) {
    for (interaction, button_type, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(BTN_PRESS);
                match button_type {
                    MenuButton::Play => {
                        next_state.set(GameState::Playing);
                    }
                    MenuButton::Settings => {
                        // Despawn main menu, spawn settings screen
                        for entity in menu_query.iter() {
                            commands.entity(entity).despawn();
                        }
                        spawn_settings_screen(&mut commands, &settings);
                    }
                    MenuButton::ResetCharacter => {
                        info!("Character reset - despawning player");
                        for entity in player_query.iter() {
                            commands.entity(entity).despawn();
                        }
                        commands.remove_resource::<HasPlayed>();
                        next_state.set(GameState::Playing);
                    }
                }
            }
            Interaction::Hovered => {
                *color = BackgroundColor(BTN_HOVER);
            }
            Interaction::None => {
                *color = BackgroundColor(BTN_COLOR);
            }
        }
    }
}

fn handle_settings_buttons(
    mut commands: Commands,
    mut adjust_query: Query<
        (&Interaction, &SettingAdjustButton, &mut BackgroundColor),
        (Changed<Interaction>, Without<BackButton>, Without<ResetButton>),
    >,
    mut back_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<BackButton>, Without<SettingAdjustButton>, Without<ResetButton>),
    >,
    mut reset_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ResetButton>, Without<BackButton>, Without<SettingAdjustButton>),
    >,
    mut settings: ResMut<GameSettings>,
    mut displays: Query<(&mut Text, &SettingValueDisplay)>,
    settings_ui: Query<Entity, With<SettingsUI>>,
    has_played: Option<Res<HasPlayed>>,
) {
    // Handle reset button - reset to defaults and rebuild settings screen
    for (interaction, mut color) in &mut reset_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(BTN_PRESS);
                *settings = GameSettings::default();
                // Despawn and respawn settings screen (re-scatters everything)
                for entity in settings_ui.iter() {
                    commands.entity(entity).despawn();
                }
                spawn_settings_screen(&mut commands, &settings);
                return;
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgb(0.4, 0.2, 0.2));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::srgb(0.3, 0.15, 0.15));
            }
        }
    }

    // Handle adjust buttons
    for (interaction, adjust, mut color) in &mut adjust_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(BTN_PRESS);
                let current = adjust.kind.get(&settings);
                adjust.kind.set(&mut settings, current + adjust.delta);
                let new_val = adjust.kind.get(&settings);
                // Update the display text
                for (mut text, display) in &mut displays {
                    if display.0 == adjust.kind {
                        *text = Text::new(adjust.kind.format_value(new_val));
                    }
                }
            }
            Interaction::Hovered => {
                *color = BackgroundColor(BTN_HOVER);
            }
            Interaction::None => {
                *color = BackgroundColor(BTN_COLOR);
            }
        }
    }

    // Handle back button
    for (interaction, mut color) in &mut back_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(BTN_PRESS);
                // Despawn settings, respawn main menu
                for entity in settings_ui.iter() {
                    commands.entity(entity).despawn();
                }
                let is_resuming = has_played.is_some();
                // Rebuild main menu
                commands
                    .spawn((
                        MainMenuUI,
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                        BackgroundColor(BG_COLOR),
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Text::new("WTF"),
                            TextFont {
                                font_size: 48.0,
                                ..default()
                            },
                            TextColor(GOLD),
                            Node {
                                margin: UiRect::bottom(Val::Px(10.0)),
                                ..default()
                            },
                        ));
                        parent.spawn((
                            Text::new("Definitely Not A Placeholder Title"),
                            TextFont {
                                font_size: 18.0,
                                ..default()
                            },
                            TextColor(DIM_TEXT),
                            Node {
                                margin: UiRect::bottom(Val::Px(50.0)),
                                ..default()
                            },
                        ));
                        let play_text = if is_resuming { "RESUME" } else { "PLAY" };
                        spawn_menu_button(parent, MenuButton::Play, play_text);
                        spawn_menu_button(parent, MenuButton::Settings, "SETTINGS");
                        spawn_menu_button(
                            parent,
                            MenuButton::ResetCharacter,
                            "RESET CHARACTER",
                        );
                    });
            }
            Interaction::Hovered => {
                *color = BackgroundColor(BTN_HOVER);
            }
            Interaction::None => {
                *color = BackgroundColor(BTN_COLOR);
            }
        }
    }
}

/////////////////////////////////
/////// Apply Settings //////////
/////////////////////////////////

fn apply_settings(
    settings: Res<GameSettings>,
    camera_settings: Option<ResMut<CameraSettings>>,
    mut light_query: Query<&mut PointLight>,
) {
    if !settings.is_changed() {
        return;
    }

    // Sync camera distance + sensitivity
    if let Some(mut cam) = camera_settings {
        cam.camera_distance = settings.camera_distance;
        cam.sensitivity = settings.mouse_sensitivity * 0.001;
    }

    // Apply hue shift to lights
    let hue = settings.hue_shift;
    if hue > 0.0 {
        let h = hue / 360.0;
        let color = Color::hsl(h * 360.0, 0.7, 0.8);
        for mut light in &mut light_query {
            light.color = color;
        }
    } else {
        for mut light in &mut light_query {
            light.color = Color::WHITE;
        }
    }
}

/////////////////////////////////
//////// Escape to Menu /////////
/////////////////////////////////

fn handle_escape_to_menu(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) || keyboard.just_pressed(KeyCode::CapsLock) {
        next_state.set(GameState::Menu);
    }
}
