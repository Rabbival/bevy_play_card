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
    mut request_listener: EventReader<CardAnimationRequest>,
    cards: Query<(&Transform, &Card, &Name)>,
    dragged_or_picked_cards: Query<(), (With<Card>, Or<(With<Picked>, With<Dragged>)>)>,
    card_consts: Res<CardConsts>,
    mut commands: Commands,
) {
    let mut request_type_by_entity: HashMap<Entity, CardAnimationRequestType> = HashMap::new();
    for request in request_listener.read() {
        request_type_by_entity.insert(request.card_entity, request.request_type);
    }
    for (entity, request_type) in request_type_by_entity {
        match request_type {
            CardAnimationRequestType::FloatBackDown
                if !dragged_or_picked_cards.contains(entity) =>
            {
                play_float_back_down_request(entity, &cards, &card_consts, &mut commands);
            }
            _ => {}
        }
    }
}

fn play_float_back_down_request(
    entity: Entity,
    cards: &Query<(&Transform, &Card, &Name)>,
    card_consts: &CardConsts,
    commands: &mut Commands,
) {
    if let Ok((transform, card, name)) = cards.get(entity) {
        let animation_target = entity.into_target();
        let mut transform_state = animation_target.transform_state(*transform);
        commands
            .spawn((
                Name::new(format!("float-back-down animation parent for {}", name)),
                TweenPriorityToOthersOfType(20),
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
