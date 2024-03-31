use bevy::input::touch::TouchPhase;

use super::*;

#[derive(Component)]
pub struct Player {
    pub health: f32,
    pub body_mass: f32,
}

#[derive(Component)]
pub struct PlayerTextMarker;

#[derive(Component)]
pub struct PlasmaMarker;

impl Default for Player {
    fn default() -> Self {
        Self {
            health: PLAYER_HEALTH,
            body_mass: PLAYER_BODY_MASS,
        }
    }
}

pub(super) fn spawn_player(
    mut commands: Commands,
    r_font: Res<DefaultFont>,
    r_mono: Res<MonoFont>,
) {
    let size = Vec2::new(50.0, 100.0);
    commands.spawn((
        Player::default(),
        PlayerImage { index: 0 },
        Velocity::default(),
        Acceleration::default(),
        Substance {
            mass: PLAYER_MASS,
            size,
        },
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(size),
                ..Default::default()
            },
            ..Default::default()
        },
    ));
    let style = TextStyle {
        font: r_font.0.clone(),
        font_size: 20.0,
        color: Color::WHITE,
    };
    let style_big = TextStyle {
        font: r_mono.0.clone(),
        font_size: 25.0,
        color: Color::YELLOW,
    };
    commands.spawn((
        Text2dBundle {
            text: Text::from_sections([
                TextSection::new("Health:  ", style.clone()),
                TextSection::new("100", style_big.clone()),
                TextSection::new("\nVelocity:  ", style.clone()),
                TextSection::new("(0.0, 0.0)", style_big.clone()),
                TextSection::new("\nMass:  ", style.clone()),
                TextSection::new("1000.0", style_big.clone()),
                TextSection::new("\nFuel:  ", style.clone()),
                TextSection::new("500.0", style_big.clone()),
                TextSection::new("\nCoordinates:  ", style.clone()),
                TextSection::new("(0.0, 0.0)", style_big.clone()),
            ]),
            text_anchor: bevy::sprite::Anchor::TopLeft,
            transform: Transform::from_translation(Vec3::new(
                -WIDTH / 2.0 + 10.0,
                HEIGHT / 2.0 - 10.0,
                100.0,
            )),
            ..Default::default()
        },
        PlayerTextMarker,
    ));
}

pub(super) fn shoot_player(
    mut commands: Commands,
    mut q_player: Query<(
        &Player,
        &Velocity,
        &mut Acceleration,
        &mut Substance,
        &Transform,
    )>,
    plasma: Res<PlasmaImage>,

    q_window: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform), With<CameraMarker>>,
    q_button: Res<ButtonInput<MouseButton>>,
    mut e_touch: EventReader<TouchInput>,
) {
    let mut position = if q_button.just_pressed(MouseButton::Left) {
        let (camera, camera_transform) = q_camera.single();

        let window = q_window.single();

        window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
    } else {
        None
    };

    e_touch.read().for_each(|input| match input.phase {
        TouchPhase::Ended => {
            position = {
                let (camera, camera_transform) = q_camera.single();
                camera
                    .viewport_to_world(camera_transform, input.position)
                    .map(|ray| ray.origin.truncate())
            };
        }
        _ => {}
    });

    if let Some(position) = position {
        let (player, vel, mut acc, mut subst, transform) = q_player.single_mut();
        let mut direction = -(position - transform.translation.xy()).normalize_or_zero();

        if direction != Vec2::ZERO && subst.mass - PLASMA_MASS >= player.body_mass {
            let vel: Vec2 = vel.into();
            direction = direction * (PLASMA_SHOOT_SPEED + vel.length());
            subst.mass -= PLASMA_MASS;

            let player_speed =
                (vel * (subst.mass + PLASMA_MASS) + direction * PLASMA_MASS) / subst.mass;
            acc.x += player_speed.x - vel.x;
            acc.y += player_speed.y - vel.y;
            direction = -direction;
            let vel = Velocity {
                x: direction.x,
                y: direction.y,
            };
            direction = direction.normalize();

            let size = Vec2::new(10.0, 10.0);
            commands.spawn((
                PlasmaMarker,
                vel,
                Acceleration::default(),
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
    }
}

pub(super) fn update_player(
    q_player: Query<
        (&Player, &Velocity, &Substance, &Transform),
        (With<Player>, Without<PlayerTextMarker>),
    >,
    mut q_text: Query<(&mut Text, &mut Transform), With<PlayerTextMarker>>,
    mut stat: ResMut<GlobalStat>,
) {
    let (player, velocity, substance, transform) = q_player.single();
    q_text
        .iter_mut()
        .for_each(|(mut text, mut text_transform)| {
            text.sections[1].value = format!("{:0<6.2}", player.health);
            text.sections[3].value = format!(
                "({:0<6.2}, {:0<6.2}) = {:0<6.2}",
                velocity.x,
                velocity.y,
                velocity.length()
            );
            text.sections[5].value = format!("{:0<6.2}", substance.mass);
            text.sections[7].value = format!("{:0<6.2}", substance.mass - player.body_mass);
            text.sections[9].value = format!(
                "({:0<9.2}, {:0<9.2}) = {:0<9.2}",
                transform.translation.x,
                transform.translation.y,
                transform.translation.length()
            );
            text_transform.translation.x = transform.translation.x - WIDTH / 2.0 + 10.0;
            text_transform.translation.y = transform.translation.y + HEIGHT / 2.0 - 10.0;
        });
    stat.distance = transform.translation.length();
}

pub(super) fn test_game_end(
    q_player: Query<(&Player, &Substance)>,
    mut state: ResMut<NextState<Status>>,
) {
    let (player, _substance) = q_player.single();
    if player.health <= 0.0 {
        state.set(Status::Over);
    }
}
