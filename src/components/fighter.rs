use bevy::ecs::component::Component;

use crate::assets::images::fighters::FighterSprite;

#[allow(dead_code)]
#[derive(Component)]
pub struct Fighter {
    fighter_sprite: FighterSprite,
}

impl Fighter {
    pub fn new(fighter_sprite: FighterSprite) -> Self {
        Self { fighter_sprite }
    }
}
