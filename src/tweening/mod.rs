use crate::prelude::*;

pub mod animation_parent_destoryer;
pub mod custom_combinators;
pub mod tween_destoryer;
pub mod tween_priority;
pub mod tween_request;

pub struct TweeningPlugin;

impl Plugin for TweeningPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TweenRequestPlugin);
    }
}
