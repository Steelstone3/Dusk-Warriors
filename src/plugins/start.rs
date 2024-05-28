use bevy::app::{Plugin, Startup};

use crate::systems::spawning::camera::spawn_camera;

pub struct StartPlugin;

impl Plugin for StartPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_camera);
    }
}
