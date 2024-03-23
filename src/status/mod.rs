use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, States)]
pub enum Status {
    Menu,
    Play,
    Over,
}

pub struct StatusPlugin;

fn setup_status(mut _commands: Commands) {}

impl Plugin for StatusPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(Status::Play);
        app.add_systems(Startup, (setup_status,));
    }
}
