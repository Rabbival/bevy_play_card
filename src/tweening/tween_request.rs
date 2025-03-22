use crate::prelude::*;

#[derive(Debug, Clone, Event)]
pub enum TweenRequest {
    RemoveEntity(RemoveTweenTargets),
    RemoveTargetsFromAllTweensTargetingThem(Vec<Entity>),
}

#[derive(Debug, Clone)]
pub struct RemoveTweenTargets {
    pub tween_entity: Entity,
    pub targets_to_remove: Vec<Entity>,
}

pub struct TweenRequestPlugin;

impl Plugin for TweenRequestPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TweenRequest>();
    }
}
