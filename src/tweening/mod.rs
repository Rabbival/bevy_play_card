use crate::prelude::*;

pub mod animation_parent_destoryer;
pub mod custom_combinators;
pub mod tween_destoryer;
pub mod tween_priority;
pub mod tween_request;

pub struct TweeningPlugin {
    pub logging_function: Option<fn(String)>,
}

#[derive(Resource)]
pub struct TweeningLoggingFunction(pub Option<fn(String)>);

impl Plugin for TweeningPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TweeningLoggingFunction(self.logging_function.clone()))
            .add_plugins((
                TweenRequestPlugin,
                DefaultTweenPlugins,
                AnimationParentDestroyerPlugin,
            ));
    }
}
