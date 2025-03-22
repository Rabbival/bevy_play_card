use crate::prelude::*;

#[derive(Debug, Event)]
pub struct CardLineRequest {
    pub line: Entity,
    pub request_type: CardLineRequestType,
}

#[derive(Debug)]
pub enum CardLineRequestType {
    RaiseCardLine,
    LowerCardLine,
    AddToCardLine { card_entity: Entity },
    RemoveCardFromLine { card_entity: Entity },
}

pub struct CardLineEventsPlugin;

impl Plugin for CardLineEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CardLineRequest>();
    }
}
