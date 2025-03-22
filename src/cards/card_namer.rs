use crate::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct CardNamer {
    cards_named: u32,
}

pub struct CardNamerPlugin;

impl Plugin for CardNamerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CardNamer>();
    }
}

impl CardNamer {
    pub fn make_name(&mut self) -> Name {
        self.cards_named += 1;
        Name::new(format!("Card {}", self.cards_named))
    }
}
