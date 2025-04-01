use crate::prelude::*;
pub struct CardPickingPlugin;

impl Plugin for CardPickingPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_card_click);
    }
}

fn on_card_click(
    mut trigger: Trigger<Pointer<Click>>,
    picked_cards: Query<&Card, With<Picked>>,
    dragged_cards: Query<(&Card, &Dragged)>,
    cards: Query<&Card>,
    card_lines: Query<&CardLine>,
    mut commands: Commands,
) {
    trigger.propagate(false);
    if let Ok(card) = cards.get(trigger.target) {
        let card_is_picked = picked_cards.get(trigger.target).is_ok();
        if let Ok(mut card_entity_commands) = commands.get_entity(trigger.target) {
            if card_is_picked {
                card_entity_commands.remove::<Picked>();
            } else if dragged_cards.get(trigger.target).is_ok() {
                return;
            } else if let Some(owner_line_entity) = card.owner_line {
                if line_is_below_picked_cards_capacity(
                    owner_line_entity,
                    &picked_cards,
                    &card_lines,
                ) {
                    card_entity_commands.insert(Picked);
                }
            } else {
                card_entity_commands.insert(Picked);
            }
        }
    }
}

fn line_is_below_picked_cards_capacity(
    owner_line_entity: Entity,
    picked_cards: &Query<&Card, With<Picked>>,
    card_lines: &Query<&CardLine>,
) -> bool {
    if let Ok(card_line) = card_lines.get(owner_line_entity) {
        if let Some(picked_cards_capacity) = card_line.picked_cards_capacity {
            let picked_cards_in_line = picked_cards.iter().filter(|card| match card.owner_line {
                Some(owner_line_entity) => owner_line_entity == owner_line_entity,
                None => false,
            });
            if picked_cards_in_line.count() >= picked_cards_capacity {
                return false;
            }
        }
    }
    true
}
