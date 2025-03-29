use crate::prelude::*;
pub struct CardPickingPlugin;

impl Plugin for CardPickingPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_card_click);
    }
}

fn on_card_click(
    mut trigger: Trigger<Pointer<Click>>,
    picked_cards: Query<(), (With<Card>, With<Picked>)>,
    cards: Query<(), With<Card>>,
    mut commands: Commands,
) {
    if cards.get(trigger.target).is_err() {
        return;
    }
    trigger.propagate(false);
    let card_is_picked = picked_cards.get(trigger.target).is_ok();
    if let Ok(mut card_entity_commands) = commands.get_entity(trigger.target) {
        if card_is_picked {
            card_entity_commands.remove::<Picked>();
        } else {
            card_entity_commands.insert(Picked);
        }
    }
}
