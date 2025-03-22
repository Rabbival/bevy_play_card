use crate::prelude::*;

pub mod despawn_request;
pub mod despawn_request_handler;

pub struct DespawningPlugin;

impl Plugin for DespawningPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((DespawnRequestHandlerPlugin, DespawnRequestPlugin));
    }
}
