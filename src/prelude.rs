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

pub static WIDTH: f32 = 2000.0;
pub static HEIGHT: f32 = 1600.0;
pub static FRICTION: f32 = 1.0;
pub static PLAYER_MASS: f32 = 2000.0;
pub static PLAYER_BODY_MASS: f32 = 500.0;
pub static PLAYER_HEALTH: f32 = 360.0;
pub static PLASMA_MASS: f32 = 40.0;
pub static PLASMA_SHOOT_SPEED: f32 = 20.0;
pub static APPROX_PLAYER_SIZE: f32 = 70.0;
pub static TARGET_DISTANCE: f32 = 1e6;
pub static DAMAGE_COEFFICIENT: f32 = 0.01;
pub static SUPPLY_MASS: f32 = 10.0;
pub static SUPPLY_FUEL_MIN: f32 = 300.0;
pub static SUPPLY_FUEL_MAX: f32 = 1000.0;
pub static SUPPLY_HEALTH_MIN: f32 = 30.0;
pub static SUPPLY_HEALTH_MAX: f32 = 50.0;
pub static SUPPLY_SPAWN_COEFFICIENT: f32 = 1e3;
pub static ROCK_MASS: f32 = 300.0;
pub static ROCK_HEALTH: f32 = 100.0;
pub static PLANE_MASS: f32 = 150.0;
pub static PLANE_HEALTH: f32 = 50.0;
pub static PLANE_ACC: f32 = 0.1;
