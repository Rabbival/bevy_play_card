use crate::cards::card_consts::CardConsts;
use crate::prelude::*;
use bevy_tween::combinator::{AnimationBuilderExt, TransformTargetStateExt, parallel};
use bevy_tween::interpolation::EaseKind;
use bevy_tween::prelude::IntoTarget;
use bevy_tween_helpers::custom_combinators::named_tween;
use bevy_tween_helpers::prelude::TweenPriorityToOthersOfType;
use std::time::Duration;

pub struct CardAnimationRequestHandlerPlugin;

impl Plugin for CardAnimationRequestHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_animation_requests);
    }
}

fn handle_animation_requests(
    mut request_listener: MessageReader<CardAnimationRequest>,
    cards: Query<(&Transform, &Card, &Name, Has<MovingToNewOrigin>)>,
    dragged_or_picked_cards: Query<(), (With<Card>, Or<(With<Picked>, With<Dragged>)>)>,
    card_lines: Query<&CardLine>,
    card_consts: Res<CardConsts>,
    mut commands: Commands,
) {
    let mut request_type_by_entity: HashMap<Entity, CardAnimationRequestType> = HashMap::new();
    for request in request_listener.read() {
        request_type_by_entity.insert(request.entity, request.request_type);
    }
    for (entity, request_type) in request_type_by_entity {
        if dragged_or_picked_cards.contains(entity) {
            continue;
        }
        match request_type {
            CardAnimationRequestType::FloatBackDown => {
                play_float_back_down_request(entity, &cards, &card_consts, &mut commands);
            }
            CardAnimationRequestType::FloatUp { tween_name } => {
                play_card_float_up_animation(
                    entity,
                    tween_name,
                    &cards,
                    &card_lines,
                    &card_consts,
                    &mut commands,
                );
            }
        }
    }
}

fn play_float_back_down_request(
    entity: Entity,
    cards: &Query<(&Transform, &Card, &Name, Has<MovingToNewOrigin>)>,
    card_consts: &CardConsts,
    commands: &mut Commands,
) {
    if let Ok((transform, card, name, moving_to_new_origin)) = cards.get(entity) {
        let tween_priority = if moving_to_new_origin {
            10 + TWEEN_PRIORITY_ADDITION_ON_ORIGIN_SET
        } else {
            10
        };
        let animation_target = entity.into_target();
        let mut transform_state = animation_target.transform_state(*transform);
        commands
            .spawn((
                Name::new(format!("float-back-down animation parent for {}", name)),
                TweenPriorityToOthersOfType(tween_priority),
                PlayCardTweenAnimationParent,
            ))
            .animation()
            .insert(parallel((
                named_tween(
                    Duration::from_secs_f32(card_consts.on_float_back_down_scale_duration),
                    EaseKind::Linear,
                    transform_state.scale_to(card.origin.scale),
                    format!("{} float-back-down scale tween", name),
                ),
                named_tween(
                    Duration::from_secs_f32(card_consts.on_float_back_down_position_tween_duration),
                    EaseKind::CubicOut,
                    transform_state.translation_to(card.origin.translation),
                    format!("{} float-back-down translation tween", name),
                ),
            )));
    }
}

fn play_card_float_up_animation(
    card_to_animate: Entity,
    animation_name: &str,
    cards: &Query<(&Transform, &Card, &Name, Has<MovingToNewOrigin>)>,
    card_lines: &Query<&CardLine>,
    card_consts: &CardConsts,
    commands: &mut Commands,
) {
    if let Ok((transform, card, name, moving_to_new_origin)) = cards.get(card_to_animate) {
        let tween_priority = if moving_to_new_origin {
            10 + TWEEN_PRIORITY_ADDITION_ON_ORIGIN_SET
        } else {
            10
        };
        let animation_target = card_to_animate.into_target();
        let mut transform_state = animation_target.transform_state(*transform);
        let scale_tween = named_tween(
            Duration::from_secs_f32(card_consts.on_hover_scale_duration),
            EaseKind::Linear,
            transform_state.scale_to(card_consts.on_hover_scale_factor * card.origin.scale),
            format!("{} {} scaling tween", name, animation_name),
        );
        let mut animation_parent_commands = commands.spawn((
            Name::new(format!("{} animation parent for {}", animation_name, name)),
            TweenPriorityToOthersOfType(tween_priority),
            PlayCardTweenAnimationParent,
        ));
        if let Some(card_line_entity) = card.owner_line
            && let Ok(card_line) = card_lines.get(card_line_entity)
        {
            animation_parent_commands.animation().insert(parallel((
                scale_tween,
                named_tween(
                    Duration::from_secs_f32(card_consts.on_hover_position_tween_duration),
                    EaseKind::CubicOut,
                    transform_state.translation_to(
                        card.origin.translation.with_y(card_line.card_hover_height) + Vec3::Z,
                    ),
                    format!("{} {} translation tween", name, animation_name),
                ),
            )));
        } else {
            animation_parent_commands.animation().insert(scale_tween);
        }
    }
}
