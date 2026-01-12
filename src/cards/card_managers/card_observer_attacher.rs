use crate::cards::card_managers::card_dragging_manager::{on_drag, on_drag_end, on_drag_start};
use crate::cards::card_managers::card_hovering_manager::{on_hover, on_hover_cancel};
use crate::cards::card_managers::card_picking_manager::on_card_click;
use crate::prelude::*;

pub struct CardObserverAttacherPlugin;

impl Plugin for CardObserverAttacherPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(add_click_listener_to_newborn_cards);
    }
}

fn add_click_listener_to_newborn_cards(newborn_card: On<Add, Card>, mut commands: Commands) {
    if let Ok(mut card_commands) = commands.get_entity(newborn_card.entity) {
        card_commands
            .observe(on_card_click)
            .observe(on_drag_start)
            .observe(on_drag)
            .observe(on_drag_end)
            .observe(on_hover)
            .observe(on_hover_cancel)
            .observe(remove_card_from_line_on_card_despawn);
    }
}
