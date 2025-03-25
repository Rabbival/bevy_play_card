use crate::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct CardNamer {
    cards_named: u32,
}

pub struct CardNamerPlugin;

impl Plugin for CardNamerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CardNamer>()
            .add_observer(name_newborn_card);
    }
}

fn name_newborn_card(
    trigger: Trigger<OnAdd, Card>,
    mut card_namer: ResMut<CardNamer>,
    mut commands: Commands,
) {
    if let Some(mut card_entity_commands) = commands.get_entity(trigger.target) {
        card_entity_commands.insert(card_namer.make_name());
    }
}

impl CardNamer {
    pub fn make_name(&mut self) -> Name {
        self.cards_named += 1;
        Name::new(format!("Card {}", self.cards_named))
    }
}
