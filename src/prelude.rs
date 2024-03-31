pub use crate::components::*;
pub use crate::resources::*;
pub use crate::status::*;
pub(crate) use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};
pub(crate) use rand::prelude::*;
pub(crate) use rand_chacha::ChaCha8Rng;
pub(crate) use std::collections::HashSet;

pub static WIDTH: f32 = 1600.0;
pub static HEIGHT: f32 = 900.0;
pub static FRICTION: f32 = 1.0;
pub static PLAYER_MASS: f32 = 2000.0;
pub static PLAYER_BODY_MASS: f32 = 500.0;
pub static PLAYER_HEALTH: f32 = 360.0;
pub static PLASMA_MASS: f32 = 40.0;
pub static PLASMA_SHOOT_SPEED: f32 = 20.0;
pub static APPROX_PLAYER_SIZE: f32 = 70.0;
pub static TARGET_DISTANCE: f32 = 2e6;
pub static DAMAGE_COEFFICIENT: f32 = 0.01;
pub static SUPPLY_MASS: f32 = 10.0;
pub static SUPPLY_FUEL_MIN: f32 = 300.0;
pub static SUPPLY_FUEL_MAX: f32 = 600.0;
pub static SUPPLY_HEALTH_MIN: f32 = 30.0;
pub static SUPPLY_HEALTH_MAX: f32 = 50.0;
pub static SUPPLY_SPAWN_COEFFICIENT: f32 = 1e3;
pub static ROCK_MASS: f32 = 300.0;
pub static ROCK_HEALTH: f32 = 100.0;
pub static PLANE_MASS: f32 = 150.0;
pub static PLANE_HEALTH: f32 = 50.0;
pub static PLANE_ACC: f32 = 0.1;
pub static SUN_MASS_MIN: f32 = 8000.0;
pub static SUN_MASS_MAX: f32 = 15000.0;
pub static SUN_HEALTH: f32 = 10000.0;
pub static SUN_SIZE_MAX: f32 = 400.0;
pub static SUN_SPAWN_COEFFICIENT: f32 = 1e2;
pub static GRAVITY_CONSTANT: f32 = 0.667;
pub static NON_CENTER: f32 = 3.0;
pub static STATION_MASS: f32 = 500.0;
pub static STATION_HEALTH: f32 = 100.0;
pub static STATION_SPAWN_COEFFICIENT: f32 = 2e2;
pub static APPROX_STATION_SIZE: f32 = 200.0;
pub static MISSILE_MASS: f32 = 30.0;
pub static MISSILE_HEALTH: f32 = 5.0;
pub static MISSILE_SPEED: f32 = 20.0;
pub static FAKE_PLAYER_MASS: f32 = 100.0;
pub static FAKE_PLAYER_HEALTH: f32 = 50.0;
pub static FAKE_PLAYER_SPAWN_COEFFICIENT: f32 = 5e2;
pub static FAKE_PLAYER_SPEED_MIN: f32 = 6.0;
pub static FAKE_PLAYER_SPEED_MAX: f32 = 12.0;
pub static MAX_SPEED: f32 = 30.0;

use wasm_bindgen::prelude::*;

fn app() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: bevy::window::WindowResolution::new(WIDTH, HEIGHT)
                        .with_scale_factor_override(1.0),
                    title: "Billion Meters Into Space".to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ComponentsPlugin,
            StatusPlugin,
            ResourcesPlugin,
        ))
        .run();
}

#[wasm_bindgen]
pub fn run() {
    app();
}
