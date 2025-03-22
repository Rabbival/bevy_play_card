use crate::prelude::*;

pub struct DespawnRequestHandlerPlugin;

impl Plugin for DespawnRequestHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(despawn_by_request_type_on_tween_event);
    }
}

fn despawn_by_request_type_on_tween_event(
    trigger: Trigger<TweenEvent<DespawnRequest>>,
    mut commands: Commands,
) {
    if let Some(entity) = trigger.data.entity {
        match trigger.data.request_type {
            DespawnRequestType::DespawnSelf => {
                if let Some(entity_commands) = commands.get_entity(entity) {
                    entity_commands.try_despawn_recursive();
                }
                commands.trigger(TweenRequest::RemoveTargetsFromAllTweensTargetingThem(vec![entity]));
            }
            request_type => print_error(
                format!("Woah there! There's no despawn implementation for TweenEvent with request type: {:?}",request_type),
                vec![LogCategory::RequestNotFulfilled],
            ),
        }
    }
}
