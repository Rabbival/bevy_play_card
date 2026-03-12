use crate::cards::card_consts::CardConsts;
use crate::prelude::*;
use bevy_tween::combinator::{event, parallel, sequence};
use bevy_tween::prelude::*;
use bevy_tween_helpers::prelude::{TweenPriorityToOthersOfType, TweenRequest, named_tween};

#[derive(Debug, Clone, Copy)]
pub(crate) enum CardDraggingRequest {
    StartDragging,
    DragMove { delta: Vec2 },
    EndDragging,
}

#[derive(Debug, Resource, Default, Deref, DerefMut)]
pub(crate) struct AwaitingCardDraggingRequests(pub(crate) HashMap<Entity, CardDraggingRequest>);

pub struct CardDraggingPlugin;

impl Plugin for CardDraggingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AwaitingCardDraggingRequests>()
            .add_systems(
                Update,
                execute_card_dragging_requests.in_set(CardsOrderingSystemSet::CardAnimation),
            );
    }
}

pub(crate) fn on_drag_start(
    trigger: On<Pointer<DragStart>>,
    undragged_cards: Query<(), (With<Card>, Without<Dragged>)>,
    mut requests: ResMut<AwaitingCardDraggingRequests>,
) {
    if undragged_cards.contains(trigger.entity) {
        requests.insert(trigger.entity, CardDraggingRequest::StartDragging);
    }
}

pub(crate) fn on_drag(
    trigger: On<Pointer<Drag>>,
    dragged_cards: Query<(), (With<Card>, With<Dragged>)>,
    mut requests: ResMut<AwaitingCardDraggingRequests>,
) {
    if dragged_cards.contains(trigger.entity) {
        requests.insert(
            trigger.entity,
            CardDraggingRequest::DragMove {
                delta: trigger.delta,
            },
        );
    }
}

pub(crate) fn on_drag_end(
    trigger: On<Pointer<DragEnd>>,
    dragged_cards: Query<(), (With<Card>, With<Dragged>)>,
    mut requests: ResMut<AwaitingCardDraggingRequests>,
) {
    if dragged_cards.contains(trigger.entity) {
        requests.insert(trigger.entity, CardDraggingRequest::EndDragging);
    }
}

fn execute_card_dragging_requests(
    mut undragged_cards: Query<(&mut Pickable, &Card), Without<Dragged>>,
    mut dragged_cards: Query<
        (
            &mut Transform,
            Entity,
            &Card,
            &mut Dragged,
            &mut Pickable,
            &Name,
            Has<ChildOf>,
            Has<MovingToNewOrigin>,
        ),
        Without<CardLine>,
    >,
    card_lines_query: Query<&Transform, Without<Card>>,
    mut requests: ResMut<AwaitingCardDraggingRequests>,
    card_consts: Res<CardConsts>,
    mut commands: Commands,
) {
    let mut card_to_start_dragging_by_owner_line = HashMap::new();
    let dragged_cards_as_lens_vec: Vec<(Card, Dragged)> = dragged_cards
        .iter()
        .map(|(_, _, card, dragged, _, _, _, _)| (*card, *dragged))
        .collect();
    for (card_entity, dragging_reqeust) in requests.drain() {
        match dragging_reqeust {
            CardDraggingRequest::StartDragging => {
                if let Ok((_, card)) = undragged_cards.get(card_entity)
                    && let Some(owner_line) = card.owner_line
                    && !theres_an_actively_dragged_card_from_that_line(
                        owner_line,
                        dragged_cards_as_lens_vec
                            .iter()
                            .map(|(card, dragged)| (card, dragged)),
                    )
                {
                    card_to_start_dragging_by_owner_line.insert(owner_line, card_entity);
                }
            }
            CardDraggingRequest::DragMove { delta } => {
                drag_card(card_entity, delta, &mut dragged_cards, &card_consts)
            }
            CardDraggingRequest::EndDragging => send_card_back_to_origin(
                card_entity,
                &mut dragged_cards,
                &card_lines_query,
                &card_consts,
                &mut commands,
            ),
        }
    }
    for card_entity in card_to_start_dragging_by_owner_line.values() {
        start_card_drag(*card_entity, &mut undragged_cards, &mut commands);
    }
}

fn start_card_drag(
    card_entity: Entity,
    cards: &mut Query<(&mut Pickable, &Card), Without<Dragged>>,
    commands: &mut Commands,
) {
    if let Ok((mut card_pickable, _)) = cards.get_mut(card_entity) {
        commands.trigger(TweenRequest::RemoveTargetsFromAllTweensTargetingThem(vec![
            card_entity,
        ]));
        commands
            .entity(card_entity)
            .try_remove::<MovingToNewOrigin>()
            .try_insert(Dragged::Actively);
        card_pickable.should_block_lower = false;
    }
}

fn drag_card(
    card_entity: Entity,
    delta: Vec2,
    dragged_cards: &mut Query<
        (
            &mut Transform,
            Entity,
            &Card,
            &mut Dragged,
            &mut Pickable,
            &Name,
            Has<ChildOf>,
            Has<MovingToNewOrigin>,
        ),
        Without<CardLine>,
    >,
    card_consts: &CardConsts,
) {
    if let Ok((mut card_transform, _, _, _, _, _, _, _)) = dragged_cards.get_mut(card_entity) {
        card_transform.translation.x += delta.x * card_consts.card_drag_delta_scaler.x;
        card_transform.translation.y -= delta.y * card_consts.card_drag_delta_scaler.y;
    }
}

fn send_card_back_to_origin(
    card_entity: Entity,
    dragged_cards: &mut Query<
        (
            &mut Transform,
            Entity,
            &Card,
            &mut Dragged,
            &mut Pickable,
            &Name,
            Has<ChildOf>,
            Has<MovingToNewOrigin>,
        ),
        Without<CardLine>,
    >,
    card_lines_query: &Query<&Transform, Without<Card>>,
    card_consts: &CardConsts,
    commands: &mut Commands,
) {
    if let Ok((
        mut card_transform,
        card_entity,
        card,
        mut card_dragged_component,
        mut card_pickable,
        card_name,
        card_has_parent,
        is_moving_to_new_origin,
    )) = dragged_cards.get_mut(card_entity)
    {
        *card_dragged_component = Dragged::GoingBackToPlace;
        card_pickable.should_block_lower = true;

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
            is_moving_to_new_origin,
            card,
            &card_transform,
            card_name,
            &card_consts,
            commands,
        );
    }
}

fn play_card_going_back_to_place_animation(
    card_entity: Entity,
    card_currently_going_to_new_origin: bool,
    card: &Card,
    card_transform: &Transform,
    card_name: &Name,
    card_consts: &CardConsts,
    commands: &mut Commands,
) {
    let tween_priority = if card_currently_going_to_new_origin {
        30 + TWEEN_PRIORITY_ADDITION_ON_ORIGIN_SET
    } else {
        30
    };
    let animation_target = card_entity.into_target();
    let mut transform_state = animation_target.transform_state(*card_transform);
    commands
        .spawn((
            Name::new(format!(
                "Go-back-to-origin-after-dragging animation parent for {}",
                card_name
            )),
            TweenPriorityToOthersOfType(tween_priority),
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
