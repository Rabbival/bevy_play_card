use crate::plugin_for_implementors_of_trait;
use crate::prelude::*;
use bevy_time_runner::TimeRunnerEnded;
use bevy_tween::bevy_time_runner::TimeRunner;
use bevy_tween::prelude::ComponentTween;

pub struct AnimationParentDestroyerPlugin;

impl Plugin for AnimationParentDestroyerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, despawn_done_time_runners);
    }
}

plugin_for_implementors_of_trait!(AnimationParentDestroyerGenericPlugin, Sendable);

impl<T: Sendable> Plugin for AnimationParentDestroyerGenericPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_observer(despawn_time_runners_with_no_children::<T>);
    }
}

pub fn despawn_done_time_runners(
    mut time_runner_ended_reader: EventReader<TimeRunnerEnded>,
    mut commands: Commands,
) {
    for event in time_runner_ended_reader.read() {
        if event.is_completed() {
            if let Ok(mut entity_commands) = commands.get_entity(event.time_runner) {
                entity_commands.try_despawn();
            }
        }
    }
}

pub fn despawn_time_runners_with_no_children<T: Sendable>(
    trigger: Trigger<OnRemove, ComponentTween<T>>,
    time_runners: Query<(&Children, Entity), With<TimeRunner>>,
    mut commands: Commands,
) {
    'time_runners_for: for (time_runner_children, time_runner_entity) in &time_runners {
        for child in time_runner_children.iter() {
            if child != trigger.target() {
                continue 'time_runners_for;
            }
        }
        if let Ok(mut entity_commands) = commands.get_entity(time_runner_entity) {
            entity_commands.try_despawn();
        }
    }
}
