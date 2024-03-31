use super::*;

#[derive(Component)]
pub struct CameraMarker;

// This is for components that should not be killed too soon by camera
#[derive(Component)]
pub struct DontJustKillMe;

#[derive(Component)]
pub struct BackgroundMarker;

pub(super) fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
            ..Default::default()
        },
        CameraMarker,
    ));
}

pub(super) fn setup_background(mut commands: Commands, backg: Res<Background>) {
    for x in -1..=1 {
        for y in -1..=1 {
            commands.spawn((
                BackgroundMarker,
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(WIDTH, HEIGHT)),
                        ..Default::default()
                    },
                    texture: backg.0.clone(),
                    transform: Transform::from_translation(Vec3::new(
                        0.0 + x as f32 * WIDTH,
                        0.0 + y as f32 * HEIGHT,
                        -10.0,
                    )),
                    ..Default::default()
                },
            ));
        }
    }
}

pub(super) fn recycle_background(
    q_camera: Query<&Transform, (With<CameraMarker>, Without<BackgroundMarker>)>,
    mut q_entity: Query<&mut Transform, With<BackgroundMarker>>,
) {
    let camera = q_camera.single();
    q_entity.iter_mut().for_each(|mut transform| {
        let diff = camera.translation - transform.translation;
        if diff.x > WIDTH {
            transform.translation.x += WIDTH * 3.0;
        }
        if diff.x < -WIDTH {
            transform.translation.x -= WIDTH * 3.0;
        }
        if diff.y > HEIGHT {
            transform.translation.y += HEIGHT * 3.0;
        }
        if diff.y < -HEIGHT {
            transform.translation.y -= HEIGHT * 3.0;
        }
    });
}

pub(super) fn remove_outbound(
    mut commands: Commands,
    q_camera: Query<&Transform, With<CameraMarker>>,
    q_entity: Query<
        (Entity, &Transform),
        (Without<CameraMarker>, Without<Player>, With<Substance>),
    >,
    q_dont_kill: Query<(), With<DontJustKillMe>>,
) {
    let camera = q_camera.single();

    q_entity.iter().for_each(|(entity, transform)| {
        let diff = camera.translation - transform.translation;
        if q_dont_kill.get(entity).is_ok() {
            if diff.x.abs() > WIDTH || diff.y.abs() > HEIGHT {
                commands.entity(entity).despawn_recursive();
            }
        } else {
            if diff.x.abs() > WIDTH / 2.0 || diff.y.abs() > HEIGHT / 2.0 {
                commands.entity(entity).despawn_recursive();
            }
        }
    });
}

pub(super) fn restore_camera_position(mut q_camera: Query<&mut Transform, With<CameraMarker>>) {
    q_camera.single_mut().translation.x = 0.0;
    q_camera.single_mut().translation.y = 0.0;
}
