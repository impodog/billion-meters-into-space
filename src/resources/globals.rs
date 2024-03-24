use super::*;

#[derive(Debug, Resource)]
pub struct GlobalStat {
    pub distance: f32,
}

impl Default for GlobalStat {
    fn default() -> Self {
        Self { distance: 0.0 }
    }
}

pub(super) fn setup_global_stat(mut commands: Commands) {
    commands.insert_resource(GlobalStat::default());
}
