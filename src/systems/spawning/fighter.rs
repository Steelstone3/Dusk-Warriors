use bevy::prelude::Commands;

use crate::{assets::images::fighters::FighterSprite, components::fighter::Fighter};

pub fn spawn_fighter(_commands: Commands) {
    let _fighter = Fighter::new(FighterSprite::FireKnight);

    // spawn_animated_sprite_event.send(SpawnAnimatedSpriteEvent {
    //     frame_timing: fighter.frame_timing,
    //     frame_count: fighter.frame_count,
    //     tile_size: fighter.tile_size,
    //     tile_columns: fighter.frame_count,
    //     spawn_sprite_event: SpawnSpriteEvent {
    //         sprite_path: fighter.sprite_path.to_string(),
    //         size: fighter.size,
    //         transform,
    //         entity: commands.spawn(fighter).id(),
    //     },
    // });
}
