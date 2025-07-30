use crate::cards::card_consts::CardConsts;
use crate::prelude::*;
use bevy_tween::combinator::{event, parallel, sequence};
use bevy_tween::prelude::*;
use bevy_tween_helpers::prelude::{TweenPriorityToOthersOfType, named_tween};

pub(crate) fn on_drag_start(
    trigger: Trigger<Pointer<DragStart>>,
    mut card_transforms: Query<&Card>,
    dragged_cards: Query<(&Card, &Dragged)>,
    mut commands: Commands,
) {
    if let Ok(card) = card_transforms.get_mut(trigger.target) {
        if theres_an_actively_dragged_card_from_that_line(card, &dragged_cards) {
            return;
        }
        if let Ok(mut entity_commands) = commands.get_entity(trigger.target) {
            entity_commands.insert(Dragged::Actively);
            if card.owner_line.is_some() {
                entity_commands.remove_parent_in_place();
            }
        }
    }
}

pub(crate) fn on_drag(
    trigger: Trigger<Pointer<Drag>>,
    mut card_transforms: Query<&mut Transform, With<Card>>,
    card_consts: Res<CardConsts>,
) {
    if let Ok(mut card_transform) = card_transforms.get_mut(trigger.target) {
        card_transform.translation.x += trigger.delta.x * card_consts.card_drag_delta_scaler.x;
        card_transform.translation.y -= trigger.delta.y * card_consts.card_drag_delta_scaler.y;
    }
}

pub(crate) fn back_to_origin_when_unused(
    trigger: Trigger<Pointer<DragEnd>>,
    mut dragged_cards: Query<
        (&mut Transform, Entity, &Card, &mut Dragged, &Name),
        Without<CardLine>,
    >,
    card_lines_query: Query<&Transform, Without<Card>>,
    card_consts: Res<CardConsts>,
    mut commands: Commands,
) {
    if let Ok((mut card_transform, card_entity, card, mut card_dragged_component, card_name)) =
        dragged_cards.get_mut(trigger.target)
    {
        *card_dragged_component = Dragged::GoingBackToPlace;

        if let Some(owner_card_line) = card.owner_line {
            if let (Ok(card_line_transform), Ok(mut card_line_commands)) = (
                card_lines_query.get(owner_card_line),
                commands.get_entity(owner_card_line),
            ) {
                card_line_commands.add_child(card_entity);

                let inverse = card_line_transform.compute_matrix().inverse();
                card_transform.translation = inverse.transform_point3(card_transform.translation);
                card_transform.rotation =
                    inverse.to_scale_rotation_translation().1 * card_transform.rotation;
                card_transform.scale /= card_line_transform.scale;
            }
        }
        play_card_going_back_to_place_animation(
            card_entity,
            card,
            &card_transform,
            card_name,
            &card_consts,
            &mut commands,
        );
    }
}

fn play_card_going_back_to_place_animation(
    card_entity: Entity,
    card: &Card,
    card_transform: &Transform,
    card_name: &Name,
    card_consts: &CardConsts,
    commands: &mut Commands,
) {
    let animation_target = card_entity.into_target();
    let mut transform_state = animation_target.transform_state(*card_transform);
    commands
        .spawn((
            Name::new(format!(
                "Go-back-to-origin-after-dragging animation parent for {}",
                card_name
            )),
            TweenPriorityToOthersOfType(30),
        ))
        .animation()
        .insert(sequence((
            parallel((
                named_tween(
                    Duration::from_secs_f32(card_consts.go_back_to_place_tween_duration),
                    EaseKind::Linear,
                    transform_state.translation_to(card.origin.translation),
                    format!(
                        "{} go-back-to-origin-after-dragging translation tween",
                        card_name
                    ),
                ),
                named_tween(
                    Duration::from_secs_f32(card_consts.go_back_to_place_tween_duration),
                    EaseKind::Linear,
                    transform_state.scale_to(card.origin.scale),
                    format!("{} go-back-to-origin-after-dragging scale tween", card_name),
                ),
            )),
            event(DeclareDraggingDoneForCard {
                card_entity: Some(card_entity),
            }),
        )));
}

pub(crate) fn listen_to_dragging_done_for_card(
    trigger: Trigger<TweenEvent<DeclareDraggingDoneForCard>>,
    cards: Query<(), With<Card>>,
    mut commands: Commands,
) {
    if let Some(entity) = trigger.data.card_entity {
        if let Ok(_card) = cards.get(entity) {
            if let Ok(mut entity_commands) = commands.get_entity(entity) {
                entity_commands.remove::<Dragged>();
            }
        }
    }
}
