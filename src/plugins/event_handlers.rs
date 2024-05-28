use bevy::app::{Plugin, Update};

use crate::events::event_handlers::spawn_sprite::spawn_sprite;

pub struct EventHandlersPlugin;

impl Plugin for EventHandlersPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, spawn_sprite);
    }
}
