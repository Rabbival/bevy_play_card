use crate::cards::card_consts::CardConsts;
use crate::prelude::*;
use bevy_tween::combinator::{event, parallel, sequence};
use bevy_tween::prelude::*;

pub struct CardDraggingPlugin;

impl Plugin for CardDraggingPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_drag_start)
            .add_observer(on_drag)
            .add_observer(back_to_origin_when_unused)
            .add_observer(listen_to_dragging_done_for_card);
    }
}

fn on_drag_start(
    drag_start: Trigger<Pointer<DragStart>>,
    mut card_transforms: Query<(&mut Transform, &Card), (With<Card>, Without<CardLine>)>,
    card_line_transforms: Query<&Transform, (With<CardLine>, Without<Card>)>,
    mut commands: Commands,
) {
    if let Ok((mut card_transform, card)) = card_transforms.get_mut(drag_start.entity()) {
        if let Some(mut entity_commands) = commands.get_entity(drag_start.entity()) {
            entity_commands.insert(Dragged::Actively);
        }

        if let Some(card_line) = card.owner_line {
            if let (Ok(card_line_transform), Some(mut card_line_commands)) = (
                card_line_transforms.get(card_line),
                commands.get_entity(card_line),
            ) {
                card_line_commands.remove_children(&[drag_start.entity()]);
                card_transform.translation =
                    card_line_transform.transform_point(card_transform.translation);
                card_transform.rotation = card_line_transform.rotation * card_transform.rotation;
                card_transform.scale *= card_line_transform.scale;
            }
        }
    }
}

fn on_drag(
    drag: Trigger<Pointer<Drag>>,
    mut card_transforms: Query<&mut Transform, With<Card>>,
) {
    if let Ok(mut card_transform) = card_transforms.get_mut(drag.entity()) {
        card_transform.translation.x += drag.delta.x;
        card_transform.translation.y -= drag.delta.y;
    }
}

fn back_to_origin_when_unused(
    drag_end: Trigger<Pointer<DragEnd>>,
    mut dragged_cards: Query<
        (&mut Transform, Entity, &Card, &mut Dragged, &Name),
        Without<CardLine>,
    >,
    card_lines_query: Query<&Transform, Without<Card>>,
    card_consts: Res<CardConsts>,
    mut commands: Commands,
) {
    if let Ok((mut card_transform, card_entity, card, mut card_dragged_component, card_name)) =
        dragged_cards.get_mut(drag_end.entity())
    {
        *card_dragged_component = Dragged::GoingBackToPlace;

        if let Some(owner_card_line) = card.owner_line {
            if let (Ok(card_line_transform), Some(mut card_line_commands)) = (
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
            TweenPriorityToOthersOfType(2),
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
                    transform_state.rotation_to(card.origin.rotation),
                    format!(
                        "{} go-back-to-origin-after-dragging rotation tween",
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

fn listen_to_dragging_done_for_card(
    trigger: Trigger<TweenEvent<DeclareDraggingDoneForCard>>,
    cards: Query<(), With<Card>>,
    mut commands: Commands,
) {
    if let Some(entity) = trigger.data.card_entity {
        if let Ok(_card) = cards.get(entity) {
            if let Some(mut entity_commands) = commands.get_entity(entity) {
                entity_commands.remove::<Dragged>();
            }
        }
    }
}
