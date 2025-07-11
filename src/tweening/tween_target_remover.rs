use tween::{AnimationTarget, ComponentTween, TargetComponent};

use crate::{plugin_for_implementors_of_trait, prelude::*, read_single_field_variant};

plugin_for_implementors_of_trait!(TweenTargetRemover, Sendable);

impl<T: Sendable> Plugin for TweenTargetRemover<T> {
    fn build(&self, app: &mut App) {
        app.add_observer(remove_entity_and_clear_tween_if_has_none::<T>)
            .add_observer(remove_targets_from_all_tweens_targeting_them::<T>)
            .add_systems(
                Update,
                listen_to_remove_entity_from_tween_targets_requests::<T>
                    .in_set(TweenHelpersSystemSet::TargetRemoval),
            );
    }
}

fn remove_targets_from_all_tweens_targeting_them<T: Sendable>(
    trigger: Trigger<TweenRequest>,
    mut tweens_of_type: Query<(&mut ComponentTween<T>, Entity, Option<&Name>)>,
    logging_function: Res<TweeningLoggingFunction>,
    mut commands: Commands,
) {
    if let TweenRequest::RemoveTargetsFromAllTweensTargetingThem(entities) = trigger.event() {
        if entities.is_empty() {
            return;
        }
        for (mut tween, tween_entity, maybe_tween_name) in &mut tweens_of_type {
            remove_target_and_destroy_if_has_none(
                entities,
                tween_entity,
                &mut tween,
                maybe_tween_name,
                &logging_function.0,
                &mut commands,
            );
        }
    }
}

fn remove_entity_and_clear_tween_if_has_none<T: Sendable>(
    trigger: Trigger<OnRemove, AnimationTarget>,
    mut query: Query<(&mut ComponentTween<T>, Option<&Name>, Entity)>,
    logging_function: Res<TweeningLoggingFunction>,
    mut commands: Commands,
) {
    for (mut tween, maybe_tween_name, tween_entity) in &mut query {
        remove_target_and_destroy_if_has_none(
            &vec![trigger.target()],
            tween_entity,
            &mut tween,
            maybe_tween_name,
            &logging_function.0,
            &mut commands,
        );
    }
}

fn listen_to_remove_entity_from_tween_targets_requests<T: Sendable>(
    mut tween_request_reader: EventReader<TweenRequest>,
    mut tweens_of_type: Query<(&mut ComponentTween<T>, Option<&Name>)>,
    logging_function: Res<TweeningLoggingFunction>,
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
                &logging_function.0,
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
    logging_function: &Option<fn(String) -> ()>,
    commands: &mut Commands,
) {
    match &mut tween.target {
        TargetComponent::Entity(tween_target) => {
            if targets_to_match.contains(tween_target) {
                if let Ok(mut entity_commands) = commands.get_entity(tween_entity) {
                    entity_commands.try_despawn();
                    if let Some(logger) = logging_function {
                        logger(format!(
                            "destroying tween: {}",
                            maybe_tween_name.unwrap_or(&Name::new("(nameless)"))
                        ));
                    }
                }
            }
        }
        TargetComponent::Entities(tween_targets) => {
            tween_targets.retain(|target| !targets_to_match.contains(target));
            if let Some(logger) = logging_function {
                logger(format!(
                    "removing targets {:?} from tween: {}",
                    targets_to_match,
                    maybe_tween_name.unwrap_or(&Name::new("(nameless)"))
                ));
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
