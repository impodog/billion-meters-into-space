pub use crate::components::*;
pub use crate::resources::*;
pub use crate::status::*;
pub(crate) use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};
pub(crate) use rand::prelude::*;
pub(crate) use rand_chacha::ChaCha8Rng;

pub static WIDTH: f32 = 2000.0;
pub static HEIGHT: f32 = 1600.0;
pub static FRICTION: f32 = 1.0;
pub static PLAYER_MASS: f32 = 1000.0;
pub static PLASMA_MASS: f32 = 10.0;
pub static PLASMA_SHOOT_SPEED: f32 = 5.0;
pub static APPROX_PLAYER_SIZE: f32 = 70.0;
