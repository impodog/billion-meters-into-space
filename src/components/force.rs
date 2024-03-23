use super::*;

#[derive(Component)]
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

            commands.entity(entity).despawn_recursive();

            Ok(())
        };
        let _ = job();
    });
}

pub(super) fn test_bump(
    mut commands: Commands,
    q_subst: Query<(Entity, &Substance, &Velocity, &Transform)>,
) {
    q_subst
        .iter()
        .for_each(|(entity, subst, velocity, transform)| {
            q_subst.iter().for_each(
                |(other_entity, other_subst, other_velocity, other_transform)| {
                    if entity != other_entity {
                        let rect = subst.rect_with_transform(transform);
                        let other_rect = other_subst.rect_with_transform(other_transform);
                        if rect.intersects(&other_rect) {
                            let left: Vec2 = velocity.into();
                            let right: Vec2 = other_velocity.into();
                            let left_new = (left * (subst.mass - other_subst.mass)
                                + 2.0 * other_subst.mass * right)
                                / (subst.mass + other_subst.mass);
                            commands.spawn(Force {
                                a: entity,
                                b: other_entity,
                                size: (left - left_new) * subst.mass,
                            });
                        }
                    }
                },
            );
        });
}
