use bevy::app::{Plugin, Startup};

use crate::systems::spawning::background::spawn_background;

pub struct RunningPlugin;

impl Plugin for RunningPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_background);
    }
}
