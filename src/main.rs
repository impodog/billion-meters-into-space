use ::billion_meters_into_space::prelude::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: bevy::window::WindowResolution::new(WIDTH, HEIGHT)
                        .with_scale_factor_override(1.0),
                    title: "Billion Meters Into Space".to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ComponentsPlugin,
            StatusPlugin,
            ResourcesPlugin,
        ))
        .run();
}
