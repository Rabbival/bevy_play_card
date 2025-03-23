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
    trigger: Trigger<TweenRequest>,
    mut tweens_of_type: Query<(&mut ComponentTween<T>, Entity)>,
    mut commands: Commands,
) {
    if let TweenRequest::RemoveTargetsFromAllTweensTargetingThem(entities) = trigger.event() {
        for (mut tween, tween_entity) in &mut tweens_of_type {
            remove_target_and_destroy_if_has_none(
                entities,
                tween_entity,
                &mut tween,
                &mut commands,
            );
        }
    }
}

fn remove_entity_and_clear_tween_if_has_none<T: Sendable>(
    trigger: Trigger<OnRemove, AnimationTarget>,
    mut query: Query<(&mut ComponentTween<T>, Entity)>,
    mut commands: Commands,
) {
    for (mut tween, tween_entity) in &mut query {
        remove_target_and_destroy_if_has_none(
            &vec![trigger.entity()],
            tween_entity,
            &mut tween,
            &mut commands,
        );
    }
}

fn handle_tween_priority_on_spawn<T: Sendable>(
    mut tween_request_writer: EventWriter<TweenRequest>,
    tween_priorities_query: Query<&TweenPriorityToOthersOfType>,
    all_tweens_of_type: Query<(
        &ComponentTween<T>,
        &Parent,
        Option<&TweenPriorityToOthersOfType>,
        Entity,
    )>,
    newborn_tweens_query: Query<
        (
            &ComponentTween<T>,
            &Parent,
            Entity,
            Option<&TweenPriorityToOthersOfType>,
        ),
        Added<ComponentTween<T>>,
    >,
) {
    for (newborn_tween, parent, newborn_tween_entity, maybe_tween_priority) in &newborn_tweens_query
    {
        if let Some(priority) = maybe_tween_priority {
            handle_tween_priority_to_others_of_type(
                &mut tween_request_writer,
                priority,
                newborn_tween,
                newborn_tween_entity,
                &all_tweens_of_type,
                &tween_priorities_query,
            );
        } else if let Ok(parent_priority) = tween_priorities_query.get(parent.get()) {
            handle_tween_priority_to_others_of_type(
                &mut tween_request_writer,
                parent_priority,
                newborn_tween,
                newborn_tween_entity,
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
    all_tweens_of_type: &Query<(
        &ComponentTween<T>,
        &Parent,
        Option<&TweenPriorityToOthersOfType>,
        Entity,
    )>,
    tween_priorities_query: &Query<&TweenPriorityToOthersOfType>,
) {
    for (other_tween, parent, maybe_other_priority, other_tween_entity) in all_tweens_of_type {
        if other_tween_entity != newborn_tween_entity {
            if let Some(other_priority_level) = try_get_other_tween_priority(
                maybe_other_priority,
                parent.get(),
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
) -> Option<u8> {
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
            tween_request_writer.send(TweenRequest::RemoveEntity(RemoveTweenTargets {
                tween_entity: weaker_tween_entity,
                targets_to_remove: vec![*dominant_target],
            }));
        }
        TargetComponent::Entities(dominant_targets) => {
            tween_request_writer.send(TweenRequest::RemoveEntity(RemoveTweenTargets {
                tween_entity: weaker_tween_entity,
                targets_to_remove: dominant_targets.clone(),
            }));
        }
        _ => {}
    }
}

fn listen_to_remove_entity_from_tween_targets_requests<T: Sendable>(
    mut tween_request_reader: EventReader<TweenRequest>,
    mut tweens_of_type: Query<&mut ComponentTween<T>>,
    mut commands: Commands,
) {
    for remove_request in
        read_single_field_variant!(tween_request_reader, TweenRequest::RemoveEntity)
    {
        if let Ok(mut tween) = tweens_of_type.get_mut(remove_request.tween_entity) {
            remove_target_and_destroy_if_has_none(
                &remove_request.targets_to_remove,
                remove_request.tween_entity,
                &mut tween,
                &mut commands,
            );
        }
    }
}

fn remove_target_and_destroy_if_has_none<T: Sendable>(
    targets_to_match: &Vec<Entity>,
    tween_entity: Entity,
    tween: &mut ComponentTween<T>,
    commands: &mut Commands,
) {
    match &mut tween.target {
        TargetComponent::Entity(tween_target) => {
            if targets_to_match.contains(tween_target) {
                commands.entity(tween_entity).try_despawn_recursive();
            }
        }
        TargetComponent::Entities(tween_targets) => {
            tween_targets.retain(|target| !targets_to_match.contains(target));
            if tween_targets.is_empty() {
                commands.entity(tween_entity).try_despawn_recursive();
            }
        }
        _ => {}
    }
}
