use super::*;

#[derive(Component)]
pub struct PlayerImage {
    pub index: usize,
}

#[derive(Resource)]
pub struct PlayerImageRes {
    start: Handle<Image>,
    a: Handle<Image>,
    b: Handle<Image>,
}

#[derive(Resource)]
pub struct MilkyWay(pub Handle<Image>);

#[derive(Resource)]
pub struct PlasmaImage(pub Handle<Image>);

#[derive(Resource)]
pub struct SupplyImage(pub Handle<Image>);

#[derive(Resource)]
pub struct RockImage(pub Vec<Handle<Image>>);

#[derive(Resource)]
pub struct PlaneImage(pub Handle<Image>);

pub(super) fn setup_image(mut commands: Commands, asset_server: Res<AssetServer>) {
    let start = asset_server.load("player_s.png");
    let a = asset_server.load("player_a.png");
    let b = asset_server.load("player_b.png");

    commands.insert_resource(PlayerImageRes { start, a, b });
    commands.insert_resource(MilkyWay(asset_server.load("milky_way.png")));
    commands.insert_resource(PlasmaImage(asset_server.load("plasma.png")));
    commands.insert_resource(SupplyImage(asset_server.load("supply.png")));
    commands.insert_resource(RockImage(vec![
        asset_server.load("rock.png"),
        asset_server.load("rock2.png"),
    ]));
    commands.insert_resource(PlaneImage(asset_server.load("plane.png")));
}

pub(super) fn player_image(
    mut q_image: Query<
        (
            &Velocity,
            &Acceleration,
            &mut PlayerImage,
            &mut Handle<Image>,
        ),
        With<Player>,
    >,
    r_player_image: Res<PlayerImageRes>,
    mut timer: Local<Timer>,
    time: Res<Time>,
) {
    q_image
        .iter_mut()
        .for_each(|(vel, acc, mut player_image, mut image)| {
            if acc.modify_flag {
                *timer = Timer::from_seconds(1.5, TimerMode::Once);
            }
            if vel.is_zero() || timer.finished() {
                player_image.index = 0;
                *image = r_player_image.start.clone();
            } else {
                player_image.index += 1;
                player_image.index = player_image.index % 10;
                *image = match player_image.index {
                    0..=4 => r_player_image.a.clone(),
                    _ => r_player_image.b.clone(),
                };
            }
            timer.tick(time.delta());
        });
}
