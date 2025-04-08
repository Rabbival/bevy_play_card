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
    mut trigger: Trigger<Pointer<DragStart>>,
    mut card_transforms: Query<&Card>,
    dragged_cards: Query<(&Card, &Dragged)>,
    mut commands: Commands,
) {
    trigger.propagate(false);
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

fn on_drag(
    mut trigger: Trigger<Pointer<Drag>>,
    mut card_transforms: Query<&mut Transform, With<Card>>,
) {
    trigger.propagate(false);
    if let Ok(mut card_transform) = card_transforms.get_mut(trigger.target) {
        card_transform.translation.x += trigger.delta.x;
        card_transform.translation.y -= trigger.delta.y;
    }
}

fn back_to_origin_when_unused(
    mut trigger: Trigger<Pointer<DragEnd>>,
    mut dragged_cards: Query<
        (&mut Transform, Entity, &Card, &mut Dragged, &Name),
        Without<CardLine>,
    >,
    card_lines_query: Query<&Transform, Without<Card>>,
    card_consts: Res<CardConsts>,
    mut commands: Commands,
) {
    trigger.propagate(false);
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

fn listen_to_dragging_done_for_card(
    mut trigger: Trigger<TweenEvent<DeclareDraggingDoneForCard>>,
    cards: Query<(), With<Card>>,
    mut commands: Commands,
) {
    trigger.propagate(false);
    if let Some(entity) = trigger.data.card_entity {
        if let Ok(_card) = cards.get(entity) {
            if let Ok(mut entity_commands) = commands.get_entity(entity) {
                entity_commands.remove::<Dragged>();
            }
        }
    }
}
