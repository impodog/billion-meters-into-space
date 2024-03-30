use super::*;

#[derive(Debug, Component)]
pub struct Enemy {
    pub health: f32,
}

pub(super) fn test_enemy_plasma_bump(
    mut commands: Commands,
    mut q_enemy: Query<&mut Enemy>,
    q_plasma: Query<Entity, With<PlasmaMarker>>,
    mut e_bump: EventReader<BumpEvent>,
) {
    e_bump.read().for_each(|event| {
        if let Ok(mut enemy) = q_enemy.get_mut(event.a) {
            if let Ok(plasma) = q_plasma.get(event.b) {
                commands.entity(plasma).despawn_recursive();
                enemy.health -= DAMAGE_COEFFICIENT * event.size;
            }
        }
    });
}

pub(super) fn test_enemy_player_bump(
    mut q_enemy: Query<&mut Enemy>,
    mut q_plasma: Query<&mut Player>,
    mut e_bump: EventReader<BumpEvent>,
) {
    e_bump.read().for_each(|event| {
        if let Ok(mut player) = q_plasma.get_mut(event.a) {
            if let Ok(mut enemy) = q_enemy.get_mut(event.b) {
                player.health -= DAMAGE_COEFFICIENT * event.size;
                enemy.health -= DAMAGE_COEFFICIENT * event.size;
            }
        }
    });
}

pub(super) fn test_enemy_kill(mut commands: Commands, q_enemy: Query<(Entity, &Enemy)>) {
    q_enemy.iter().for_each(|(entity, enemy)| {
        if enemy.health <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    });
}

pub(super) fn spawn_rock(
    mut commands: Commands,
    q_player: Query<(&Transform, &Velocity), With<Player>>,
    rock: Res<RockImage>,
    mut seed: ResMut<Seed>,
) {
    let (pos, vel) = q_player.single();
    let pos = pos.translation;
    let distance = pos.length();
    if seed.chance(
        distance as f64 / 2.0 * vel.length() as f64,
        TARGET_DISTANCE as f64,
    ) {
        let pos = seed.rand_non_center_vec3() + pos;
        let size = Vec2::new(100.0, 100.0);
        commands.spawn((
            Enemy {
                health: ROCK_HEALTH,
            },
            Velocity::default(),
            Acceleration::default(),
            Substance {
                mass: ROCK_MASS,
                size,
            },
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(size),
                    ..Default::default()
                },
                texture: rock.0[seed.rand_range(0..rock.0.len())].clone(),
                transform: Transform::from_translation(pos),
                ..Default::default()
            },
        ));
    }
}

#[derive(Debug, Component)]
pub struct PlaneMarker;

pub(super) fn spawn_plane(
    mut commands: Commands,
    q_player: Query<(&Transform, &Velocity), With<Player>>,
    plane: Res<PlaneImage>,
    mut seed: ResMut<Seed>,
) {
    let (pos, vel) = q_player.single();
    let pos = pos.translation;
    let distance = pos.length();
    if seed.chance(
        distance as f64 / 5.0 * vel.length() as f64,
        TARGET_DISTANCE as f64,
    ) {
        let pos = seed.rand_non_center_vec3() + pos;
        let size = Vec2::new(100.0, 100.0);
        commands.spawn((
            Enemy {
                health: PLANE_HEALTH,
            },
            PlaneMarker,
            Velocity::default(),
            Acceleration::default(),
            Substance {
                mass: PLANE_MASS,
                size,
            },
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(size),
                    ..Default::default()
                },
                texture: plane.0.clone(),
                transform: Transform::from_translation(pos),
                ..Default::default()
            },
        ));
    }
}

pub(super) fn plane_move(
    mut q_plane: Query<(&PlaneMarker, &Transform, &mut Acceleration)>,
    q_player: Query<&Transform, With<Player>>,
) {
    let player_pos = q_player.single().translation;
    q_plane.iter_mut().for_each(|(_, transform, mut acc)| {
        let diff = player_pos - transform.translation;
        let direction = diff.normalize_or_zero();
        acc.x += direction.x * PLANE_ACC;
        acc.y += direction.y * PLANE_ACC;
    });
}

#[derive(Component)]
pub struct SunMarker;

pub(super) fn spawn_sun(
    mut commands: Commands,
    q_player: Query<(&Transform, &Velocity), With<Player>>,
    sun: Res<SunImage>,
    mut seed: ResMut<Seed>,
) {
    let (pos, vel) = q_player.single();
    let pos = pos.translation;
    if seed.chance(
        SUN_SPAWN_COEFFICIENT as f64 * vel.length() as f64,
        TARGET_DISTANCE as f64,
    ) {
        let pos = seed.rand_non_center_vec3() + pos;
        let size = Vec2::new(SUN_SIZE, SUN_SIZE);
        commands.spawn((
            Enemy { health: SUN_HEALTH },
            SunMarker,
            DontJustKillMe,
            NoTurning,
            Velocity::default(),
            Acceleration::default(),
            Substance {
                mass: SUN_MASS,
                size,
            },
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(size),
                    ..Default::default()
                },
                texture: sun.0.clone(),
                transform: Transform::from_translation(pos),
                ..Default::default()
            },
        ));
    }
}

pub(super) fn test_sun_other_bump(
    mut commands: Commands,
    q_sun: Query<(Entity, &Substance, &Transform), With<SunMarker>>,
    q_other: Query<(Entity, &Substance, &Transform), Without<SunMarker>>,
) {
    q_sun.iter().for_each(|(sun, sub_subst, sun_transform)| {
        q_other
            .iter()
            .for_each(|(other, other_subst, other_transform)| {
                commands.spawn(Force {
                    a: sun,
                    b: other,
                    size: (other_transform.translation.xy() - sun_transform.translation.xy())
                        .normalize_or_zero()
                        * (GRAVITY_CONSTANT * sub_subst.mass * other_subst.mass
                            / sun_transform
                                .translation
                                .distance_squared(other_transform.translation)),
                });
            });
    });
}
#[derive(Component)]
pub struct StationMarker {
    pub timer: Timer,
}

impl Default for StationMarker {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.5, TimerMode::Repeating),
        }
    }
}

pub(super) fn spawn_station(
    mut commands: Commands,
    q_player: Query<(&Transform, &Velocity), With<Player>>,
    station: Res<StationImage>,
    mut seed: ResMut<Seed>,
) {
    let (pos, vel) = q_player.single();
    let pos = pos.translation;
    if seed.chance(
        STATION_SPAWN_COEFFICIENT as f64 * vel.length() as f64,
        TARGET_DISTANCE as f64,
    ) {
        let pos = seed.rand_non_center_vec3() + pos;
        let size = Vec2::new(90.0, 180.0);
        commands.spawn((
            Enemy {
                health: STATION_HEALTH,
            },
            StationMarker::default(),
            DontJustKillMe,
            NoTurning,
            Velocity::default(),
            Acceleration::default(),
            Substance {
                mass: STATION_MASS,
                size,
            },
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(size),
                    ..Default::default()
                },
                texture: station.0.clone(),
                transform: Transform::from_translation(pos),
                ..Default::default()
            },
        ));
    }
}

pub(super) fn station_shoot_missile(
    mut commands: Commands,
    mut q_station: Query<(&Transform, &mut StationMarker)>,
    q_player: Query<(&Transform, &Velocity), (With<Player>, Without<StationMarker>)>,
    missile: Res<MissileImage>,
    timer: Res<Time>,
) {
    let (player_pos, player_vel) = q_player.single();
    let player_vel: Vec2 = player_vel.into();
    let player_vel = Vec3::new(player_vel.x, player_vel.y, 0.0);
    q_station
        .iter_mut()
        .for_each(|(station_transform, mut station_data)| {
            station_data.timer.tick(timer.delta());
            if !station_data.timer.finished() {
                return;
            }

            let diff = player_pos.translation - station_transform.translation + player_vel;
            let direction = diff.normalize_or_zero();
            let velocity = direction * MISSILE_SPEED;
            let size = Vec2::new(40.0, 80.0);
            commands.spawn((
                Enemy {
                    health: MISSILE_HEALTH,
                },
                Velocity {
                    x: velocity.x,
                    y: velocity.y,
                },
                Acceleration::default(),
                Substance {
                    mass: MISSILE_MASS,
                    size,
                },
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(size),
                        ..Default::default()
                    },
                    texture: missile.0.clone(),
                    transform: Transform {
                        translation: station_transform.translation
                            + direction * APPROX_STATION_SIZE,
                        rotation: Quat::from_rotation_z(
                            velocity.y.atan2(velocity.x) - std::f32::consts::PI / 2.0,
                        ),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ));
        });
}

#[derive(Component)]
pub struct FakePlayerMarker;

pub(super) fn spawn_fake_player(
    mut commands: Commands,
    q_player: Query<(&Transform, &Velocity), With<Player>>,
    player: Res<PlayerImageRes>,
    mut seed: ResMut<Seed>,
) {
    let (pos, vel) = q_player.single();
    let pos = pos.translation;
    if seed.chance(
        FAKE_PLAYER_SPAWN_COEFFICIENT as f64 * vel.length() as f64,
        TARGET_DISTANCE as f64,
    ) {
        let pos = seed.rand_non_center_vec3() + pos;
        let size = Vec2::new(50.0, 100.0);
        let velocity = seed.rand_non_center_vec3().normalize_or_zero()
            * seed.rand_range(FAKE_PLAYER_SPEED_MIN..FAKE_PLAYER_SPEED_MAX);
        commands.spawn((
            Enemy {
                health: FAKE_PLAYER_HEALTH,
            },
            FakePlayerMarker,
            Velocity {
                x: velocity.x,
                y: velocity.y,
            },
            Acceleration::default(),
            Substance {
                mass: FAKE_PLAYER_MASS,
                size,
            },
            PlayerImage { index: 0 },
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(size),
                    ..Default::default()
                },
                texture: player.start.clone(),
                transform: Transform {
                    translation: pos,
                    rotation: Quat::from_rotation_z(
                        velocity.y.atan2(velocity.x) - std::f32::consts::PI / 2.0,
                    ),
                    ..Default::default()
                },
                ..Default::default()
            },
        ));
    }
}
