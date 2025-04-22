use tracing::info;
use tween::{AnimationTarget, ComponentTween, TargetComponent};

use crate::{plugin_for_implementors_of_trait, prelude::*, read_single_field_variant};

plugin_for_implementors_of_trait!(TweenDestroyerPlugin, Sendable);

impl<T: Sendable> Plugin for TweenDestroyerPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_observer(remove_entity_and_clear_tween_if_has_none::<T>)
            .add_observer(remove_targets_from_all_tweens_targeting_them::<T>)
            .add_systems(
                Update,
                ((
                    handle_tween_priority_on_spawn::<T>,
                    listen_to_remove_entity_from_tween_targets_requests::<T>,
                )
                    .chain(),),
            );
    }
}

fn remove_targets_from_all_tweens_targeting_them<T: Sendable>(
    mut trigger: Trigger<TweenRequest>,
    mut tweens_of_type: Query<(&mut ComponentTween<T>, Entity, Option<&Name>)>,
    debug_logs_enabled: Res<TweeningPluginShouldPrintLogs>,
    mut commands: Commands,
) {
    trigger.propagate(false);
    if let TweenRequest::RemoveTargetsFromAllTweensTargetingThem(entities) = trigger.event() {
        for (mut tween, tween_entity, maybe_tween_name) in &mut tweens_of_type {
            remove_target_and_destroy_if_has_none(
                entities,
                tween_entity,
                &mut tween,
                maybe_tween_name,
                debug_logs_enabled.0,
                &mut commands,
            );
        }
    }
}

fn remove_entity_and_clear_tween_if_has_none<T: Sendable>(
    mut trigger: Trigger<OnRemove, AnimationTarget>,
    mut query: Query<(&mut ComponentTween<T>, Option<&Name>, Entity)>,
    debug_logs_enabled: Res<TweeningPluginShouldPrintLogs>,
    mut commands: Commands,
) {
    trigger.propagate(false);
    for (mut tween, maybe_tween_name, tween_entity) in &mut query {
        remove_target_and_destroy_if_has_none(
            &vec![trigger.target()],
            tween_entity,
            &mut tween,
            maybe_tween_name,
            debug_logs_enabled.0,
            &mut commands,
        );
    }
}

fn handle_tween_priority_on_spawn<T: Sendable>(
    mut tween_request_writer: EventWriter<TweenRequest>,
    tween_priorities_query: Query<&TweenPriorityToOthersOfType>,
    all_tweens_of_type: Query<(
        &ComponentTween<T>,
        &ChildOf,
        Option<&TweenPriorityToOthersOfType>,
        Entity,
    )>,
    newborn_tweens_query: Query<
        (
            &ComponentTween<T>,
            &ChildOf,
            Entity,
            Option<&TweenPriorityToOthersOfType>,
            Option<&Name>,
        ),
        Added<ComponentTween<T>>,
    >,
    debug_logs_enabled: Res<TweeningPluginShouldPrintLogs>,
) {
    for (newborn_tween, child_of, newborn_tween_entity, maybe_tween_priority, maybe_tween_name) in
        &newborn_tweens_query
    {
        if debug_logs_enabled.0 {
            info!(
                "{} spawned, looking for tweens to destroy by priority",
                maybe_tween_name.unwrap_or(&Name::new("A nameless tween with priority"))
            );
        }
        if let Some(priority) = maybe_tween_priority {
            handle_tween_priority_to_others_of_type(
                &mut tween_request_writer,
                priority,
                newborn_tween,
                newborn_tween_entity,
                child_of,
                &all_tweens_of_type,
                &tween_priorities_query,
            );
        } else if let Ok(parent_priority) = tween_priorities_query.get(child_of.parent()) {
            handle_tween_priority_to_others_of_type(
                &mut tween_request_writer,
                parent_priority,
                newborn_tween,
                newborn_tween_entity,
                child_of,
                &all_tweens_of_type,
                &tween_priorities_query,
            );
        }
    }
}

fn handle_tween_priority_to_others_of_type<T: Sendable>(
    tween_request_writer: &mut EventWriter<TweenRequest>,
    tween_priority: &TweenPriorityToOthersOfType,
    newborn_tween: &ComponentTween<T>,
    newborn_tween_entity: Entity,
    newborn_tween_child_of: &ChildOf,
    all_tweens_of_type: &Query<(
        &ComponentTween<T>,
        &ChildOf,
        Option<&TweenPriorityToOthersOfType>,
        Entity,
    )>,
    tween_priorities_query: &Query<&TweenPriorityToOthersOfType>,
) {
    for (other_tween, child_of, maybe_other_priority, other_tween_entity) in all_tweens_of_type {
        let sibling_tweens = newborn_tween_child_of.parent() == child_of.parent();
        if other_tween_entity != newborn_tween_entity && !sibling_tweens {
            if let Some(other_priority_level) = try_get_other_tween_priority(
                maybe_other_priority,
                child_of.parent(),
                tween_priorities_query,
            ) {
                if other_priority_level <= tween_priority.0 {
                    remove_intersecting_targets_for_weaker_tween(
                        tween_request_writer,
                        newborn_tween,
                        other_tween_entity,
                    );
                } else {
                    remove_intersecting_targets_for_weaker_tween(
                        tween_request_writer,
                        other_tween,
                        newborn_tween_entity,
                    );
                }
            }
        }
    }
}

fn try_get_other_tween_priority(
    maybe_other_tween_priority: Option<&TweenPriorityToOthersOfType>,
    other_tween_parent_entity: Entity,
    tween_policies_query: &Query<&TweenPriorityToOthersOfType>,
) -> Option<u32> {
    match maybe_other_tween_priority {
        Some(TweenPriorityToOthersOfType(other_priority_level)) => Some(*other_priority_level),
        _ => match tween_policies_query.get(other_tween_parent_entity) {
            Ok(TweenPriorityToOthersOfType(other_parent_priority_level)) => {
                Some(*other_parent_priority_level)
            }
            _ => None,
        },
    }
}

fn remove_intersecting_targets_for_weaker_tween<T: Sendable>(
    tween_request_writer: &mut EventWriter<TweenRequest>,
    dominant_tween: &ComponentTween<T>,
    weaker_tween_entity: Entity,
) {
    match &dominant_tween.target {
        TargetComponent::Entity(dominant_target) => {
            tween_request_writer.write(TweenRequest::RemoveEntity(RemoveTweenTargets {
                tween_entity: weaker_tween_entity,
                targets_to_remove: vec![*dominant_target],
            }));
        }
        TargetComponent::Entities(dominant_targets) => {
            tween_request_writer.write(TweenRequest::RemoveEntity(RemoveTweenTargets {
                tween_entity: weaker_tween_entity,
                targets_to_remove: dominant_targets.clone(),
            }));
        }
        _ => {}
    }
}

fn listen_to_remove_entity_from_tween_targets_requests<T: Sendable>(
    mut tween_request_reader: EventReader<TweenRequest>,
    mut tweens_of_type: Query<(&mut ComponentTween<T>, Option<&Name>)>,
    debug_logs_enabled: Res<TweeningPluginShouldPrintLogs>,
    mut commands: Commands,
) {
    for remove_request in
        read_single_field_variant!(tween_request_reader, TweenRequest::RemoveEntity)
    {
        if let Ok((mut tween, maybe_name)) = tweens_of_type.get_mut(remove_request.tween_entity) {
            remove_target_and_destroy_if_has_none(
                &remove_request.targets_to_remove,
                remove_request.tween_entity,
                &mut tween,
                maybe_name,
                debug_logs_enabled.0,
                &mut commands,
            );
        }
    }
}

fn remove_target_and_destroy_if_has_none<T: Sendable>(
    targets_to_match: &Vec<Entity>,
    tween_entity: Entity,
    tween: &mut ComponentTween<T>,
    maybe_tween_name: Option<&Name>,
    debug_logs_enabled: bool,
    commands: &mut Commands,
) {
    match &mut tween.target {
        TargetComponent::Entity(tween_target) => {
            if targets_to_match.contains(tween_target) {
                if let Ok(mut entity_commands) = commands.get_entity(tween_entity) {
                    entity_commands.try_despawn();
                    if debug_logs_enabled {
                        info!(
                            "destroying tween: {}",
                            maybe_tween_name.unwrap_or(&Name::new("(nameless)"))
                        );
                    }
                }
            }
        }
        TargetComponent::Entities(tween_targets) => {
            tween_targets.retain(|target| !targets_to_match.contains(target));
            if debug_logs_enabled {
                info!(
                    "removing targets {:?} from tween: {}",
                    targets_to_match,
                    maybe_tween_name.unwrap_or(&Name::new("(nameless)"))
                );
            }
            if tween_targets.is_empty() {
                if let Ok(mut entity_commands) = commands.get_entity(tween_entity) {
                    entity_commands.try_despawn();
                }
            }
        }
        _ => {}
    }
}
