use super::*;

#[derive(Component)]
pub struct Player {
    pub health: u32,
}

#[derive(Component)]
pub struct PlayerTextMarker;

impl Default for Player {
    fn default() -> Self {
        Self { health: 100 }
    }
}

pub(super) fn spawn_player(mut commands: Commands, r_font: Res<DefaultFont>) {
    let size = Vec2::new(50.0, 100.0);
    commands.spawn((
        Player::default(),
        PlayerImage { index: 0 },
        Velocity { x: 0.0, y: 0.0 },
        Acceleration { x: 0.0, y: 0.0 },
        Substance { mass: 1000.0, size },
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(50.0, 100.0)),
                ..Default::default()
            },
            ..Default::default()
        },
    ));
    let style = TextStyle {
        font: r_font.0.clone(),
        font_size: 15.0,
        color: Color::WHITE,
    };
    let style_big = TextStyle {
        font: r_font.0.clone(),
        font_size: 20.0,
        color: Color::CYAN,
    };
    commands.spawn((
        Text2dBundle {
            text: Text::from_sections([
                TextSection::new("Health: ", style.clone()),
                TextSection::new("100", style_big.clone()),
                TextSection::new("\nVelocity: ", style.clone()),
                TextSection::new("(0.0, 0.0)", style_big.clone()),
                TextSection::new("\nMass: ", style.clone()),
                TextSection::new("1000.0", style_big.clone()),
                TextSection::new("\nCoordinates: ", style.clone()),
                TextSection::new("(0.0, 0.0)", style_big.clone()),
            ]),
            transform: Transform::from_translation(Vec3::new(0.0, 100.0, 0.0)),
            ..Default::default()
        },
        PlayerTextMarker,
    ));
}

pub(super) fn move_player(
    mut commands: Commands,
    mut q_player: Query<(&Velocity, &mut Acceleration, &mut Substance, &Transform), With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    plasma: Res<PlasmaImage>,
) {
    q_player
        .iter_mut()
        .for_each(|(vel, mut acc, mut subst, transform)| {
            let mut direction = Vec2::ZERO;
            if input.just_pressed(KeyCode::KeyW) {
                direction.y += 1.0;
            }
            if input.just_pressed(KeyCode::KeyS) {
                direction.y -= 1.0;
            }
            if input.just_pressed(KeyCode::KeyA) {
                direction.x -= 1.0;
            }
            if input.just_pressed(KeyCode::KeyD) {
                direction.x += 1.0;
            }
            if direction != Vec2::ZERO {
                direction = direction.normalize() * PLASMA_SHOOT_SPEED;
                subst.mass -= PLASMA_MASS;
                let vel: Vec2 = vel.into();

                let player_speed =
                    (vel * (subst.mass + PLASMA_MASS) + direction * PLASMA_MASS) / subst.mass;
                acc.x += player_speed.x - vel.x;
                acc.y += player_speed.y - vel.y;

                let size = Vec2::new(10.0, 10.0);
                direction = -direction;
                let vel = Velocity {
                    x: direction.x,
                    y: direction.y,
                };
                direction = direction.normalize();
                commands.spawn((
                    vel,
                    Acceleration { x: 0.0, y: 0.0 },
                    Substance {
                        mass: PLASMA_MASS,
                        size,
                    },
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(10.0, 10.0)),
                            ..Default::default()
                        },
                        transform: Transform::from_translation(Vec3::new(
                            direction.x * APPROX_PLAYER_SIZE + transform.translation.x,
                            direction.y * APPROX_PLAYER_SIZE + transform.translation.y,
                            0.0,
                        )),
                        texture: plasma.0.clone(),
                        ..Default::default()
                    },
                ));
            }
        });
}

pub(super) fn update_text(
    q_player: Query<
        (&Player, &Velocity, &Substance, &Transform),
        (With<Player>, Without<PlayerTextMarker>),
    >,
    mut q_text: Query<(&mut Text, &mut Transform), With<PlayerTextMarker>>,
) {
    let (player, velocity, substance, transform) = q_player.single();
    q_text
        .iter_mut()
        .for_each(|(mut text, mut text_transform)| {
            text.sections[1].value = player.health.to_string();
            text.sections[3].value = format!("({:.4}, {:.4})", velocity.x, velocity.y);
            text.sections[5].value = format!("{:.4}", substance.mass);
            text.sections[7].value = format!(
                "({:.4}, {:.4})",
                transform.translation.x, transform.translation.y
            );
            text_transform.translation = transform.translation;
            text_transform.translation.y += 100.0;
        });
}
