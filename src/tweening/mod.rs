use crate::prelude::*;

pub mod animation_parent_destoryer;
pub mod custom_combinators;
pub mod tween_destoryer;
pub mod tween_priority;
pub mod tween_request;

pub struct TweeningPlugin{
    pub print_debug_logs: bool,
}

#[derive(Resource)]
pub struct TweeningPluginShouldPrintLogs(pub bool);


impl Plugin for TweeningPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TweeningPluginShouldPrintLogs(self.print_debug_logs))
            .add_plugins((
            TweenRequestPlugin,
            DefaultTweenPlugins,
            AnimationParentDestroyerPlugin,
        ));
    }
}
