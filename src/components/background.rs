use bevy::ecs::component::Component;

use crate::assets::images::background::BackgroundSprite;

#[derive(Component)]
pub struct Background {
    pub background_sprite: BackgroundSprite,
}

impl Background {
    pub fn new(background_sprite: BackgroundSprite) -> Self {
        Self { background_sprite }
    }
}
