use bevy::prelude::*;

/////////////////////////////////
////////// Game States //////////
/////////////////////////////////

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Menu,
    Playing,
}

/////////////////////////////////
////////// UI Components ////////
/////////////////////////////////

#[derive(Component)]
struct MainMenuUI;

#[derive(Component)]
enum MenuButton {
    Play,
    Settings,
    ResetCharacter,
}

/////////////////////////////////
///////// Menu Plugin ///////////
/////////////////////////////////

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(OnEnter(GameState::Menu), setup_menu)
            .add_systems(OnExit(GameState::Menu), cleanup_menu)
            .add_systems(Update, handle_menu_buttons.run_if(in_state(GameState::Menu)))
            .add_systems(Update, handle_escape_to_menu.run_if(in_state(GameState::Playing)));
    }
}

/////////////////////////////////
///////// Menu Setup ////////////
/////////////////////////////////

fn setup_menu(mut commands: Commands) {
    // Root container
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
            BackgroundColor(Color::srgb(0.1, 0.1, 0.12)),
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("GAME MENU"),
                TextFont {
                    font_size: 60.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                Node {
                    margin: UiRect::bottom(Val::Px(50.0)),
                    ..default()
                },
            ));

            // Play button
            parent
                .spawn((
                    Button,
                    MenuButton::Play,
                    Node {
                        width: Val::Px(250.0),
                        height: Val::Px(65.0),
                        margin: UiRect::all(Val::Px(10.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.25)),
                    Outline {
                        width: Val::Px(2.0),
                        offset: Val::Px(0.0),
                        color: Color::srgb(0.4, 0.4, 0.5),
                    },
                    BorderRadius::all(Val::Px(8.0)),
                ))
                .with_child((
                    Text::new("PLAY"),
                    TextFont {
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));

            // Settings button
            parent
                .spawn((
                    Button,
                    MenuButton::Settings,
                    Node {
                        width: Val::Px(250.0),
                        height: Val::Px(65.0),
                        margin: UiRect::all(Val::Px(10.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.25)),
                    Outline {
                        width: Val::Px(2.0),
                        offset: Val::Px(0.0),
                        color: Color::srgb(0.4, 0.4, 0.5),
                    },
                    BorderRadius::all(Val::Px(8.0)),
                ))
                .with_child((
                    Text::new("SETTINGS"),
                    TextFont {
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));

            // Reset Character button
            parent
                .spawn((
                    Button,
                    MenuButton::ResetCharacter,
                    Node {
                        width: Val::Px(250.0),
                        height: Val::Px(65.0),
                        margin: UiRect::all(Val::Px(10.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.25)),
                    Outline {
                        width: Val::Px(2.0),
                        offset: Val::Px(0.0),
                        color: Color::srgb(0.4, 0.4, 0.5),
                    },
                    BorderRadius::all(Val::Px(8.0)),
                ))
                .with_child((
                    Text::new("RESET CHARACTER"),
                    TextFont {
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
        });
}

fn cleanup_menu(mut commands: Commands, menu_query: Query<Entity, With<MainMenuUI>>) {
    for entity in menu_query.iter() {
        commands.entity(entity).despawn();
    }
}

/////////////////////////////////
/////// Button Interaction //////
/////////////////////////////////

fn handle_menu_buttons(
    mut interaction_query: Query<
        (&Interaction, &MenuButton, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, button_type, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::srgb(0.4, 0.4, 0.5));
                match button_type {
                    MenuButton::Play => {
                        next_state.set(GameState::Playing);
                    }
                    MenuButton::Settings => {
                        // TODO: Implement settings menu
                        info!("Settings button pressed - not yet implemented");
                    }
                    MenuButton::ResetCharacter => {
                        // Reset character data
                        info!("Character reset - clearing save data");
                        // TODO: Implement actual character reset logic
                    }
                }
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgb(0.3, 0.3, 0.35));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::srgb(0.2, 0.2, 0.25));
            }
        }
    }
}

fn handle_escape_to_menu(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Menu);
    }
}
