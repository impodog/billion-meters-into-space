use super::*;

#[derive(Resource)]
pub struct DefaultFont(pub Handle<Font>);

#[derive(Resource)]
pub struct MonoFont(pub Handle<Font>);

pub(super) fn setup_fonts(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("Cantarell-VF.otf");
    commands.insert_resource(DefaultFont(font));
    let font = asset_server.load("CaskaydiaCoveNerdFont-Bold.ttf");
    commands.insert_resource(MonoFont(font));
}
