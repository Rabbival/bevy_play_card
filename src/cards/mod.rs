use crate::prelude::*;

pub mod card;
pub mod card_bundle;
pub mod card_consts;
pub mod card_dragging_manager;
pub mod card_hovering_manager;
pub mod card_lines;
pub mod card_namer;
pub mod card_origin_set_listener;
pub mod card_picking_manager;
pub mod event;
pub mod tags;

pub struct CardsPlugin {
    pub print_debug_logs: bool,
}

#[derive(Resource)]
pub struct CardsPluginShouldPrintLogs(pub bool);

impl Plugin for CardsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CardsPluginShouldPrintLogs(self.print_debug_logs))
            .add_plugins((
                CardsEventsPlugin,
                CardOriginSetListenerPlugin,
                CardDraggingPlugin,
                CardHoveringPlugin,
                CardLinesPlugin,
                CardNamerPlugin,
                CardPickingPlugin,
            ));
    }
}
