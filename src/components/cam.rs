use super::*;

#[derive(Component)]
pub struct CameraMarker;

pub(super) fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
            ..Default::default()
        },
        CameraMarker,
    ));
}

pub(super) fn remove_outbound(
    mut commands: Commands,
    q_camera: Query<&Transform, With<CameraMarker>>,
    q_entity: Query<
        (Entity, &Transform),
        (Without<CameraMarker>, Without<Player>, With<Substance>),
    >,
) {
    let camera = q_camera.single();

    q_entity.iter().for_each(|(entity, transform)| {
        let diff = camera.translation - transform.translation;
        if diff.x.abs() > WIDTH / 2.0 || diff.y.abs() > HEIGHT / 2.0 {
            commands.entity(entity).despawn_recursive();
        }
    });
}
