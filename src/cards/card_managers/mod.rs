use crate::prelude::*;

pub mod card_dragging_manager;
pub mod card_hovering_manager;
pub mod card_origin_set_listener;
pub mod card_picking_manager;
pub mod card_tag_insertion_listener;

pub struct CardManagersPlugin;

impl Plugin for CardManagersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            CardOriginSetListenerPlugin,
            CardDraggingPlugin,
            CardHoveringPlugin,
            CardPickingPlugin,
            CardTagInsertionListenerPlugin,
        ));
    }
}

pub fn theres_an_actively_dragged_card_from_that_line(
    card_in_question: &Card,
    dragged_cards: &Query<(&Card, &Dragged)>,
) -> bool {
    if let Some(owner_line) = card_in_question.owner_line {
        for (card, dragged_component) in dragged_cards {
            if let Dragged::Actively = dragged_component {
                if let Some(dragged_owner_line) = card.owner_line {
                    if owner_line == dragged_owner_line {
                        return true;
                    }
                }
            }
        }
    }
    false
}
