use crate::prelude::*;
pub mod fonts;
pub mod globals;
pub mod images;

pub use fonts::*;
pub use globals::*;
pub use images::*;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (images::setup_image, fonts::setup_fonts));
        app.add_systems(
            Update,
            (images::player_image,).run_if(in_state(Status::Play)),
        );
        app.add_systems(OnEnter(Status::Play), (globals::setup_global_stat,));
    }
}
