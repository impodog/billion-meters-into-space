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
        distance as f64 * vel.length() as f64,
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
        distance as f64 / 3.0 * vel.length() as f64,
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
