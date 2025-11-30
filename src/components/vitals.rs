use bevy::prelude::*;

#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

#[derive(Component)]
pub struct Hunger {
    pub current: f32,
    pub max: f32,
}

#[derive(Component)]
pub struct Stamina {
    pub current: f32,
    pub max: f32,
}

#[derive(Component)]
pub struct Movement {
    pub speed: f32,
    pub sprint_aplifier: f32,
    pub jump_strength: f32,
    pub is_grounded: bool,
}
