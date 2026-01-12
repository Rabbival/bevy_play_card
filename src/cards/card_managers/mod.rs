use crate::prelude::*;

pub mod card_dragging_manager;
pub mod card_hovering_manager;
pub mod card_observer_attacher;
pub mod card_origin_set_listener;
pub mod card_picking_manager;
pub mod card_tag_change_listener;

pub struct CardManagersPlugin;

impl Plugin for CardManagersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            CardPickingPlugin,
            CardOriginSetListenerPlugin,
            CardObserverAttacherPlugin,
            CardTagChangeListenerPlugin,
            CardDraggingPlugin,
        ));
    }
}

type CardLineEntity = Entity;

pub fn theres_an_actively_dragged_card_from_that_line<'a, 'b>(
    card_owner_line: CardLineEntity,
    dragged_cards: impl Iterator<Item = (&'a Card, &'b Dragged)>,
) -> bool {
    for (card, dragged_component) in dragged_cards {
        if let Dragged::Actively = dragged_component {
            if let Some(dragged_owner_line) = card.owner_line {
                if card_owner_line == dragged_owner_line {
                    return true;
                }
            }
        }
    }
    false
}
