use crate::prelude::*;

pub mod cam;
pub mod enemy;
pub mod force;
pub mod over;
pub mod player;
pub mod seed;
pub mod supply;
pub mod vel;

pub use cam::*;
pub use enemy::*;
pub use force::*;
pub use player::*;
pub use seed::*;
pub use supply::*;
pub use vel::*;

pub struct ComponentsPlugin;

impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (cam::setup_camera, vel::setup_velocity, seed::setup_seed),
        );
        app.add_systems(PostStartup, (cam::setup_background,));
        app.add_systems(
            Update,
            (
                (
                    force::test_bump,
                    player::shoot_player,
                    cam::remove_outbound,
                    enemy::spawn_rock,
                    enemy::spawn_plane,
                    enemy::plane_move,
                    enemy::spawn_sun,
                    enemy::spawn_station,
                    enemy::station_shoot_missile,
                    enemy::spawn_fake_player,
                    supply::spawn_supply,
                ),
                (
                    vel::player_record_pos,
                    force::execute_force,
                    vel::acceleration,
                    vel::velocity,
                )
                    .chain(),
                (
                    enemy::test_enemy_plasma_bump,
                    enemy::test_enemy_player_bump,
                    enemy::test_enemy_kill,
                    enemy::test_sun_other_bump,
                    supply::test_player_supply_bump,
                ),
            )
                .chain()
                .run_if(in_state(Status::Play)),
        );
        app.add_systems(
            PostUpdate,
            (
                vel::player_move,
                player::update_player,
                player::test_game_end,
            )
                .run_if(in_state(Status::Play)),
        );
        app.add_systems(Update, (over::test_restart,));
        app.add_systems(
            OnEnter(Status::Play),
            (player::spawn_player, over::remove_ending_msg),
        );
        app.add_systems(
            OnEnter(Status::Over),
            (
                vel::despawn_when_end,
                over::show_ending_msg,
                cam::restore_camera_position,
            ),
        );
    }
}
