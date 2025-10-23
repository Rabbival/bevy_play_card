use crate::cards::card_consts::CardConsts;
use crate::prelude::*;
use bevy_tween::combinator::{AnimationBuilderExt, TransformTargetStateExt, parallel};
use bevy_tween::interpolation::EaseKind;
use bevy_tween::prelude::IntoTarget;
use bevy_tween_helpers::prelude::{TweenPriorityToOthersOfType, TweenRequest, named_tween};
use std::time::Duration;

pub struct CardTagChangeListenerPlugin;

impl Plugin for CardTagChangeListenerPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_dragged_insertion)
            .add_observer(on_hovered_insertion)
            .add_observer(on_picked_insertion)
            .add_observer(on_picked_removal);
    }
}

fn on_dragged_insertion(
    trigger: On<Add, Dragged>,
    cards: Query<&Card>,
    picked_cards: Query<Entity, (With<Card>, With<Picked>)>,
    mut commands: Commands,
) {
    if let Ok(card) = cards.get(trigger.entity)
        && card.owner_line.is_some()
        && let Ok(mut entity_commands) = commands.get_entity(trigger.entity)
    {
        entity_commands.remove_parent_in_place();
        commands.trigger(TweenRequest::RemoveTargetsFromAllTweensTargetingThem(vec![
            trigger.entity,
        ]));
    }
    for picked_card_entity in &picked_cards {
        if let Ok(mut entity_commands) = commands.get_entity(picked_card_entity) {
            entity_commands.try_remove::<Picked>();
        }
    }
}

fn on_hovered_insertion(
    trigger: On<Add, Hovered>,
    cards: Query<(&Transform, &Card, &Name)>,
    dragged_cards: Query<(), (With<Card>, With<Dragged>)>,
    card_consts: Res<CardConsts>,
    mut commands: Commands,
) {
    if dragged_cards.contains(trigger.entity) {
        return;
    }
    play_card_float_up_animation(
        trigger.entity,
        10,
        "on-hover",
        &cards,
        &card_consts,
        &mut commands,
    );
}

fn on_picked_insertion(
    trigger: On<Add, Picked>,
    cards: Query<(&Transform, &Card, &Name)>,
    dragged_cards: Query<(), (With<Card>, With<Dragged>)>,
    card_consts: Res<CardConsts>,
    mut commands: Commands,
) {
    if dragged_cards.contains(trigger.entity) {
        return;
    }
    play_card_float_up_animation(
        trigger.entity,
        50,
        "on-picked",
        &cards,
        &card_consts,
        &mut commands,
    );
}

fn play_card_float_up_animation(
    card_to_animate: Entity,
    animation_priority: u32,
    animation_name: &str,
    cards: &Query<(&Transform, &Card, &Name)>,
    card_consts: &CardConsts,
    commands: &mut Commands,
) {
    if let Ok((transform, card, name)) = cards.get(card_to_animate) {
        let animation_target = card_to_animate.into_target();
        let mut transform_state = animation_target.transform_state(*transform);
        commands
            .spawn((
                Name::new(format!("{} animation parent for {}", animation_name, name)),
                TweenPriorityToOthersOfType(animation_priority),
                PlayCardTweenAnimationParent,
            ))
            .animation()
            .insert(parallel((
                named_tween(
                    Duration::from_secs_f32(card_consts.on_hover_scale_duration),
                    EaseKind::Linear,
                    transform_state.scale_to(card_consts.on_hover_scale_factor * card.origin.scale),
                    format!("{} {} scaling tween", name, animation_name),
                ),
                named_tween(
                    Duration::from_secs_f32(card_consts.on_hover_position_tween_duration),
                    EaseKind::CubicOut,
                    transform_state.translation_to(
                        card.origin
                            .translation
                            .with_y(card_consts.card_hover_height)
                            + Vec3::Z,
                    ),
                    format!("{} {} translation tween", name, animation_name),
                ),
            )));
    }
}

fn on_picked_removal(
    trigger: On<Remove, Picked>,
    mut animation_requester: MessageWriter<CardAnimationRequest>,
    cards: Query<Option<&Dragged>, With<Card>>,
) {
    if let Ok(maybe_dragged) = cards.get(trigger.entity) {
        if maybe_dragged.is_none() {
            animation_requester.write(CardAnimationRequest {
                entity: trigger.entity,
                request_type: CardAnimationRequestType::FloatBackDown,
            });
        }
    }
}
