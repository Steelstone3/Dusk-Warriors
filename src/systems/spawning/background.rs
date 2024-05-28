use bevy::{
    ecs::event::EventWriter, math::Vec2, prelude::Commands, transform::components::Transform,
};

use crate::{
    assets::images::background::BackgroundSprite, components::background::Background,
    events::spawn_sprite_event::SpawnSpriteEvent,
};

pub fn spawn_background(
    mut commands: Commands,
    mut spawn_sprite_event: EventWriter<SpawnSpriteEvent>,
) {
    let background = Background::new(BackgroundSprite::Mountain);

    spawn_sprite_event.send(SpawnSpriteEvent {
        sprite_path: background.background_sprite.to_string(),
        size: Vec2::new(1920.0, 1080.0),
        transform: Transform::default(),
        entity: commands.spawn(background).id(),
    });
}
