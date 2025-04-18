use crate::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct CardNamer {
    cards_named: Vec<u32>,
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
    named_cards: Query<(), (With<Card>, With<Name>)>,
    mut commands: Commands,
) {
    if named_cards.get(trigger.target()).is_ok() {
        return;
    }
    if let Ok(mut card_entity_commands) = commands.get_entity(trigger.target()) {
        card_entity_commands.insert_if_new(card_namer.make_name());
    }
}

impl CardNamer {
    pub fn make_name(&mut self) -> Name {
        let mut number_string = String::new();
        match self.cards_named.last_mut() {
            None => {
                self.cards_named.push(0);
            }
            Some(last_count) => {
                *last_count += 1;
                if *last_count == u32::MAX {
                    self.cards_named.push(0);
                }
            }
        }
        for count in self.cards_named.iter().rev() {
            number_string += &count.to_string();
        }
        Name::new(format!("Card {}", number_string))
    }
}
