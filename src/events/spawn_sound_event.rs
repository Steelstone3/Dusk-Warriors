use bevy::{audio::PlaybackSettings, ecs::event::Event};

#[allow(dead_code)]
#[derive(Event)]
pub struct SpawnSoundEvent {
    pub sound_path: String,
    pub playback_settings: PlaybackSettings,
}
