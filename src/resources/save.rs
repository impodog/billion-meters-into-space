use super::*;

#[derive(Resource, serde::Serialize, serde::Deserialize)]
pub struct Save {
    pub high_distance: f32,
}

impl Default for Save {
    fn default() -> Self {
        Self { high_distance: 0.0 }
    }
}

pub(super) fn setup_save(mut commands: Commands) {
    commands.insert_resource(Save::default());
}

pub(super) fn save(_save: Res<Save>) {
    // Saving in web is not supported
}
