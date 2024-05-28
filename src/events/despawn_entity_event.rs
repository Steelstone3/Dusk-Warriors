use bevy::ecs::{entity::Entity, event::Event};

#[allow(dead_code)]
#[derive(Event)]
pub struct DespawnEntityEvent {
    pub entity: Entity,
}
