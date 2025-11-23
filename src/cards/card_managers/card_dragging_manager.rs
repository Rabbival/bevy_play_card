use crate::cards::card_consts::CardConsts;
use crate::prelude::*;
use bevy_tween::combinator::{event, parallel, sequence};
use bevy_tween::prelude::*;
use bevy_tween_helpers::prelude::{TweenPriorityToOthersOfType, TweenRequest, named_tween};

pub(crate) fn on_drag_start(
    trigger: On<Pointer<DragStart>>,
    mut tween_request_writer: MessageWriter<TweenRequest>,
    mut card_transforms: Query<&Card>,
    dragged_cards: Query<(&Card, &Dragged)>,
    mut commands: Commands,
) {
    if let Ok(card) = card_transforms.get_mut(trigger.entity) {
        if theres_an_actively_dragged_card_from_that_line(card, &dragged_cards) {
            return;
        }
        if let Ok(mut entity_commands) = commands.get_entity(trigger.entity) {
            tween_request_writer.write(TweenRequest::RemoveTargetsFromAllTweensTargetingThem(
                vec![trigger.entity],
            ));
            entity_commands
                .try_remove::<MovingToNewOrigin>()
                .try_insert(Dragged::Actively);
        }
    }
}

pub(crate) fn on_drag(
    trigger: On<Pointer<Drag>>,
    mut card_transforms: Query<&mut Transform, With<Card>>,
    card_consts: Res<CardConsts>,
) {
    if let Ok(mut card_transform) = card_transforms.get_mut(trigger.entity) {
        card_transform.translation.x += trigger.delta.x * card_consts.card_drag_delta_scaler.x;
        card_transform.translation.y -= trigger.delta.y * card_consts.card_drag_delta_scaler.y;
    }
}

pub(crate) fn back_to_origin_when_unused(
    trigger: On<Pointer<DragEnd>>,
    mut dragged_cards: Query<
        (
            &mut Transform,
            Entity,
            &Card,
            &mut Dragged,
            &Name,
            Has<ChildOf>,
        ),
        Without<CardLine>,
    >,
    card_lines_query: Query<&Transform, Without<Card>>,
    card_consts: Res<CardConsts>,
    mut commands: Commands,
) {
    if let Ok((
        mut card_transform,
        card_entity,
        card,
        mut card_dragged_component,
        card_name,
        card_has_parent,
    )) = dragged_cards.get_mut(trigger.entity)
    {
        *card_dragged_component = Dragged::GoingBackToPlace;

        if !card_has_parent
            && let Some(owner_card_line) = card.owner_line
            && let (Ok(card_line_transform), Ok(mut card_line_commands)) = (
                card_lines_query.get(owner_card_line),
                commands.get_entity(owner_card_line),
            )
        {
            card_line_commands.add_child(card_entity);
            let inverse_transform = card_line_transform.to_matrix().inverse();
            card_transform.translation =
                inverse_transform.transform_point3(card_transform.translation);
            card_transform.rotation =
                inverse_transform.to_scale_rotation_translation().1 * card_transform.rotation;
            card_transform.scale /= card_line_transform.scale;
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
            PlayCardTweenAnimationParent,
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
            event(RemoveComponentFromCardTweenRequest::<Dragged>::new(
                card_entity,
            )),
        )));
}
