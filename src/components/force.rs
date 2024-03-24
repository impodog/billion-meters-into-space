use super::*;

#[derive(Debug, Component)]
pub struct Force {
    pub a: Entity,
    pub b: Entity,
    pub size: Vec2,
}

pub(super) fn execute_force(
    mut commands: Commands,
    q_force: Query<(Entity, &Force)>,
    mut q_subst: Query<(&Substance, &mut Acceleration)>,
) {
    q_force.iter().for_each(|(entity, force)| {
        let mut job = || -> Result<(), ()> {
            {
                let (mass_a, mut acc_a) = q_subst.get_mut(force.a).map_err(|_| ())?;
                let diff_a = force.size / mass_a.mass;
                acc_a.x += diff_a.x;
                acc_a.y += diff_a.y;
            }

            {
                let (mass_b, mut acc_b) = q_subst.get_mut(force.b).map_err(|_| ())?;
                let diff_b = force.size / mass_b.mass;
                acc_b.x -= diff_b.x;
                acc_b.y -= diff_b.y;
            }

            Ok(())
        };
        let _ = job();

        commands.entity(entity).despawn_recursive();
    });
}

pub(super) fn test_bump(
    mut commands: Commands,
    q_subst: Query<(
        Entity,
        &Substance,
        &Velocity,
        &Transform,
        Option<&Player>,
        Option<&Enemy>,
    )>,
    mut e_bump: EventWriter<BumpEvent>,
) {
    let mut fresh = q_subst.iter().map(|(x, ..)| x).collect::<HashSet<_>>();
    q_subst
        .iter()
        .for_each(|(entity, subst, velocity, transform, first_player, _)| {
            q_subst
                .iter()
                .filter(|(x, ..)| fresh.contains(x) && *x != entity)
                .for_each(
                    |(
                        other_entity,
                        other_subst,
                        other_velocity,
                        other_transform,
                        player,
                        enemy,
                    )| {
                        let rect = subst.rect_with_transform(transform);
                        let other_rect = other_subst.rect_with_transform(other_transform);
                        if rect.intersects(&other_rect) {
                            let left: Vec2 = velocity.into();
                            let right: Vec2 = other_velocity.into();
                            let left_new = (left * (subst.mass - other_subst.mass)
                                + 2.0 * right * other_subst.mass)
                                / (subst.mass + other_subst.mass);
                            let force = (left_new - left) * subst.mass;
                            commands.spawn(Force {
                                a: entity,
                                b: other_entity,
                                size: force,
                            });

                            e_bump.send(
                                if player.is_some() || (!first_player.is_some() && enemy.is_some())
                                {
                                    BumpEvent {
                                        a: other_entity,
                                        b: entity,
                                        size: force.length(),
                                    }
                                } else {
                                    BumpEvent {
                                        a: entity,
                                        b: other_entity,
                                        size: force.length(),
                                    }
                                },
                            );
                        }
                    },
                );
            fresh.remove(&entity);
        });
}
