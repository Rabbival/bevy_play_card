use crate::prelude::*;
use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, Hash, PartialOrd, Default)]
pub struct DespawnRequest {
    pub entity: Option<Entity>,
    pub request_type: DespawnRequestType,
}

#[derive(
    Sequence, Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, Hash, PartialOrd, Default,
)]
pub enum DespawnRequestType {
    #[default]
    DespawnSelf,
    DespawnSelfAndClearTweens,
}

pub struct DespawnRequestPlugin;

impl Plugin for DespawnRequestPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TweenEventPlugin::<DespawnRequest>::default());
    }
}
