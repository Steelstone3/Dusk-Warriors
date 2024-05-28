use bevy::app::{Plugin, Startup};

use crate::{
    events::spawn_animated_sprite_event::SpawnAnimatedSpriteEvent,
    systems::spawning::fighter::spawn_fighter,
};

pub struct DeveloperPlugin;

impl Plugin for DeveloperPlugin {
    fn build(&self, _app: &mut bevy::prelude::App) {
        _app.add_event::<SpawnAnimatedSpriteEvent>()
            .add_systems(Startup, spawn_fighter);
    }
}
