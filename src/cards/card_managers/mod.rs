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
