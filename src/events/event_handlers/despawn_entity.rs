use bevy::ecs::{event::EventReader, system::Commands};

use crate::events::despawn_entity_event::DespawnEntityEvent;

#[allow(dead_code)]
pub fn despawn_entity(
    mut commands: Commands,
    mut despawn_entity_events: EventReader<DespawnEntityEvent>,
) {
    for despawn_entity_event in despawn_entity_events.read() {
        commands.entity(despawn_entity_event.entity).despawn()
    }
}
