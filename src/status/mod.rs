use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, States)]
pub enum Status {
    Play,
    Over,
}

/// A event indicating a collision between two entities.
/// If one entity is a player, it will be the first entity.
#[derive(Debug, Clone, Copy, Event)]
pub struct BumpEvent {
    pub a: Entity,
    pub b: Entity,
    pub size: f32,
}

pub struct StatusPlugin;

fn setup_status(mut _commands: Commands) {}

impl Plugin for StatusPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(Status::Play);
        app.add_event::<BumpEvent>();
        app.add_systems(Startup, (setup_status,));
    }
}
