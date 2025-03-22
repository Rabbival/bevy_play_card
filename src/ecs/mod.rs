use crate::prelude::*;

pub mod component_utilities;
pub mod custom_run_conditions;
pub mod despawning;
pub mod entity_error;
pub mod system_sets;

pub struct EcsPlugin;

impl Plugin for EcsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((SystemSetsPlugin, DespawningPlugin));
    }
}
