use bevy::math::bounding::Bounded2d;

use super::*;

#[derive(Debug, Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Component)]
pub struct Acceleration {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Component)]
pub struct Substance {
    pub mass: f32,
    pub size: Vec2,
}

#[derive(Debug, Resource)]
pub struct PlayerPosModify {
    pub x: f32,
    pub y: f32,
}

impl Velocity {
    pub fn is_zero(&self) -> bool {
        self.x == 0.0 && self.y == 0.0
    }
}

impl Into<Vec2> for &Velocity {
    fn into(self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}

impl Acceleration {
    pub fn is_zero(&self) -> bool {
        self.x == 0.0 && self.y == 0.0
    }
}

impl Into<Vec2> for &Acceleration {
    fn into(self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}

impl Substance {
    pub fn rect_with_center(&self, center: Vec2) -> Rect {
        Rect::new(
            center.x - self.size.x / 2.0,
            center.y - self.size.y / 2.0,
            center.x + self.size.x / 2.0,
            center.y + self.size.y / 2.0,
        )
    }

    pub fn rect_with_transform(&self, transform: &Transform) -> Aabb2d {
        Rectangle::new(
            self.size.x / 2.0 * transform.scale.x,
            self.size.y / 2.0 * transform.scale.y,
        )
        .aabb_2d(
            Vec2::new(transform.translation.x, transform.translation.y),
            transform.rotation.to_axis_angle().1,
        )
    }
}

pub(super) fn setup_velocity(mut commands: Commands) {
    commands.insert_resource(PlayerPosModify { x: 0.0, y: 0.0 });
}

pub(super) fn velocity(mut q_vel: Query<(&Velocity, &mut Transform)>) {
    q_vel.iter_mut().for_each(|(vel, mut transform)| {
        transform.translation.x += vel.x;
        transform.translation.y += vel.y;
    });
}

pub(super) fn acceleration(mut q_vel: Query<(&mut Velocity, &mut Acceleration, &mut Transform)>) {
    q_vel
        .iter_mut()
        .for_each(|(mut vel, mut acc, mut transform)| {
            if !acc.is_zero() {
                vel.x += acc.x;
                vel.y += acc.y;
                transform.rotation =
                    Quat::from_axis_angle(Vec3::new(0.0, 0.0, -1.0), acc.x.atan2(acc.y));
                acc.x = 0.0;
                acc.y = 0.0;

                vel.x *= FRICTION;
                vel.y *= FRICTION;

                if vel.x.abs() < 0.001 {
                    vel.x = 0.0;
                }
                if vel.y.abs() < 0.001 {
                    vel.y = 0.0;
                }
            }
        });
}

pub(super) fn player_record_pos(
    q_player: Query<&Transform, With<Player>>,
    mut r_player: ResMut<PlayerPosModify>,
) {
    let transform = q_player.single();
    r_player.x = transform.translation.x;
    r_player.y = transform.translation.y;
}

pub(super) fn player_move(
    r_player: Res<PlayerPosModify>,
    q_player: Query<&Transform, With<Player>>,
    mut q_transform: Query<&mut Transform, (With<CameraMarker>, Without<Player>)>,
) {
    let transform = q_player.single();
    q_transform.single_mut().translation.x += transform.translation.x - r_player.x;
    q_transform.single_mut().translation.y += transform.translation.y - r_player.y;
}