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
    let str = std::fs::read_to_string("save.toml");
    let save: Save = match str {
        Ok(str) => toml::from_str(&str).unwrap(),
        Err(_) => Save::default(),
    };
    commands.insert_resource(save);
}

pub(super) fn save(save: Res<Save>) {
    let str = toml::to_string(&*save).unwrap();
    std::fs::write("save.toml", str).unwrap();
}
