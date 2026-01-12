use crate::cards::card_consts::CardConsts;
use crate::prelude::*;

pub(crate) fn on_hover(
    trigger: On<Pointer<Over>>,
    cards: Query<&Card>,
    dragged_cards: Query<(&Card, &Dragged)>,
    card_consts: Res<CardConsts>,
    mut commands: Commands,
) {
    if let Ok(card) = cards.get(trigger.entity) {
        if card_consts.allow_hover_while_dragging {
            if let Some(owner_line) = card.owner_line
                && theres_an_actively_dragged_card_from_that_line(owner_line, dragged_cards.iter())
            {
                return;
            }
        } else {
            for (_, dragged) in &dragged_cards {
                if let Dragged::Actively = dragged {
                    return;
                }
            }
        }
        commands.entity(trigger.entity).try_insert(Hovered);
    }
}

pub(crate) fn on_hover_cancel(
    trigger: On<Pointer<Out>>,
    mut animation_requester: MessageWriter<CardAnimationRequest>,
    cards: Query<(Entity, &Card, Option<&Dragged>, Option<&Picked>)>,
    dragged_cards: Query<(&Card, &Dragged)>,
    mut commands: Commands,
) {
    if let Ok((entity, card, maybe_dragged, maybe_picked)) = cards.get(trigger.entity) {
        commands.entity(entity).try_remove::<Hovered>();

        if maybe_dragged.is_some() | maybe_picked.is_some() {
            return;
        }
        if let Some(card_line) = card.owner_line
            && theres_an_actively_dragged_card_from_that_line(card_line, dragged_cards.iter())
        {
            return;
        }
        animation_requester.write(CardAnimationRequest {
            entity,
            request_type: CardAnimationRequestType::FloatBackDown,
        });
    }
}
