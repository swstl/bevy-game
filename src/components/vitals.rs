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
    pub extra_jumps: u32,
    pub current_jumps: u32,
}

impl Movement {
    pub fn can_jump(&self) -> bool {
        self.is_grounded || self.current_jumps < self.extra_jumps
    }
}
