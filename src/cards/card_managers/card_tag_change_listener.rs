use crate::prelude::*;
use bevy_tween_helpers::prelude::TweenRequest;

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
    mut animation_requester: MessageWriter<CardAnimationRequest>,
    dragged_cards: Query<(), (With<Card>, With<Dragged>)>,
) {
    if dragged_cards.contains(trigger.entity) {
        return;
    }
    animation_requester.write(CardAnimationRequest {
        entity: trigger.entity,
        request_type: CardAnimationRequestType::FloatUp {
            tween_name: "on-hover",
        },
    });
}

fn on_picked_insertion(
    trigger: On<Add, Picked>,
    mut animation_requester: MessageWriter<CardAnimationRequest>,
    dragged_cards: Query<(), (With<Card>, With<Dragged>)>,
) {
    if dragged_cards.contains(trigger.entity) {
        return;
    }
    animation_requester.write(CardAnimationRequest {
        entity: trigger.entity,
        request_type: CardAnimationRequestType::FloatUp {
            tween_name: "on-picked",
        },
    });
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
