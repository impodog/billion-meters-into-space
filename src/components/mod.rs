use crate::prelude::*;

pub mod cam;
pub mod force;
pub mod player;
pub mod seed;
pub mod vel;

pub use cam::*;
pub use force::*;
pub use player::*;
pub use vel::*;

pub struct ComponentsPlugin;

impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (cam::setup_camera, vel::setup_velocity));
        app.add_systems(
            Update,
            (
                vel::player_record_pos,
                (
                    vel::velocity,
                    vel::acceleration,
                    player::move_player,
                    cam::remove_outbound,
                    force::execute_force,
                    force::test_bump,
                ),
            )
                .chain()
                .run_if(in_state(Status::Play)),
        );
        app.add_systems(
            PostUpdate,
            (vel::player_move, player::update_text).run_if(in_state(Status::Play)),
        );
        app.add_systems(OnEnter(Status::Play), (player::spawn_player,));
    }
}
