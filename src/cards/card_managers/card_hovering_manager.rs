use crate::cards::card_consts::CardConsts;
use crate::prelude::*;

pub(crate) fn on_hover(
    trigger: Trigger<Pointer<Over>>,
    cards: Query<&Card>,
    dragged_cards: Query<(&Card, &Dragged)>,
    card_consts: Res<CardConsts>,
    mut commands: Commands,
) {
    if let Ok(card) = cards.get(trigger.target) {
        if card_consts.allow_hover_while_dragging {
            if theres_an_actively_dragged_card_from_that_line(card, &dragged_cards) {
                return;
            }
        } else {
            for (_, dragged) in &dragged_cards {
                if let Dragged::Actively = dragged {
                    return;
                }
            }
        }
        commands.entity(trigger.target).try_insert(Hovered);
    }
}

pub(crate) fn on_hover_cancel(
    trigger: Trigger<Pointer<Out>>,
    mut animation_requester: EventWriter<CardAnimationRequest>,
    cards: Query<(Entity, &Card, Option<&Dragged>, Option<&Picked>)>,
    dragged_cards: Query<(&Card, &Dragged)>,
    mut commands: Commands,
) {
    if let Ok((entity, card, maybe_dragged, maybe_picked)) = cards.get(trigger.target) {
        commands.entity(entity).remove::<Hovered>();
        if theres_an_actively_dragged_card_from_that_line(card, &dragged_cards) {
            return;
        }
        if maybe_dragged.is_some() | maybe_picked.is_some() {
            return;
        }
        animation_requester.write(CardAnimationRequest {
            card_entity: entity,
            request_type: CardAnimationRequestType::FloatBackDown,
        });
    }
}
