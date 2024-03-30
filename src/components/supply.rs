use super::*;

#[derive(Debug, Component)]
pub struct Supply {
    pub fuel: f32,
    pub health: f32,
}

impl Default for Supply {
    fn default() -> Self {
        Self {
            fuel: 0.0,
            health: 0.0,
        }
    }
}

pub(super) fn test_player_supply_bump(
    mut commands: Commands,
    mut q_player: Query<(&mut Player, &mut Substance)>,
    mut q_supply: Query<&Supply>,
    mut e_bump: EventReader<BumpEvent>,
) {
    e_bump.read().for_each(|event| {
        if let Ok((mut player, mut subst)) = q_player.get_mut(event.a) {
            if let Ok(supply) = q_supply.get_mut(event.b) {
                subst.mass += supply.fuel;
                player.health += supply.health;
                commands.entity(event.b).despawn_recursive();
            }
        }
    });
}

pub(super) fn spawn_supply(
    mut commands: Commands,
    q_player: Query<(&Transform, &Velocity), With<Player>>,
    mut seed: ResMut<Seed>,
    supply: Res<SupplyImage>,
) {
    let (pos, vel) = q_player.single();
    let pos = pos.translation;
    if seed.chance(
        SUPPLY_SPAWN_COEFFICIENT as f64 * vel.length() as f64,
        TARGET_DISTANCE as f64,
    ) {
        let pos = seed.rand_center_vec3() + pos;
        let size = Vec2::new(50.0, 100.0);
        commands.spawn((
            Supply {
                fuel: seed.rand_range(SUPPLY_FUEL_MIN..SUPPLY_FUEL_MAX),
                health: seed.rand_range(SUPPLY_HEALTH_MIN..SUPPLY_HEALTH_MAX),
            },
            Velocity::default(),
            Acceleration::default(),
            Substance {
                mass: SUPPLY_MASS,
                size,
            },
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(size),
                    ..Default::default()
                },
                texture: supply.0.clone(),
                transform: Transform::from_translation(pos),
                ..Default::default()
            },
        ));
    }
}
