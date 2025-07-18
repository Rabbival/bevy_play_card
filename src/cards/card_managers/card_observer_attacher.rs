use crate::cards::card_managers::card_dragging_manager::{back_to_origin_when_unused, listen_to_dragging_done_for_card, on_drag, on_drag_start};
use crate::cards::card_managers::card_hovering_manager::{on_hover, on_hover_cancel};
use crate::cards::card_managers::card_picking_manager::on_card_click;
use crate::prelude::*;

pub struct CardObserverAttacherPlugin;

impl Plugin for CardObserverAttacherPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(add_click_listener_to_newborn_cards);
    }
}

fn add_click_listener_to_newborn_cards(newborn_card: Trigger<OnAdd, Card>, mut commands: Commands) {
    if let Ok(mut card_commands) = commands.get_entity(newborn_card.target()) {
        card_commands
            .observe(on_card_click)
            .observe(on_drag_start)
            .observe(on_drag)
            .observe(back_to_origin_when_unused)
            .observe(listen_to_dragging_done_for_card)
            .observe(on_hover)
            .observe(on_hover_cancel);
    }
}
