use crate::prelude::*;

pub mod card;
pub mod card_bundle;
pub mod card_dragging_manager;
pub mod card_hovering_manager;
pub mod card_lines;
pub mod card_mover;
pub mod card_namer;
pub mod cards_spawner;
pub mod consts;
pub mod event;
pub mod tags;

pub struct CardsPlugin;

impl Plugin for CardsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            CardsSpawnerPlugin,
            CardsEventsPlugin,
            CardMoverPlugin,
            CardDraggingPlugin,
            CardHoveringPlugin,
            CardLinesPlugin,
            CardNamerPlugin,
        ));
    }
}
