use crate::prelude::*;

/// A request to be handled automatically by bevy_play_card_crate
#[derive(Debug, Event)]
pub struct CardLineRequest {
    /// The line entity to which the request should apply
    pub line: Entity,
    /// The request type, including additional data if there's any
    pub request_type: CardLineRequestType,
}

/// The type of request to be applied to the line, including additional data if there's any
#[derive(Debug)]
pub enum CardLineRequestType {
    /// Raise the card-line's position, depending on the line Transform up direction
    RaiseCardLine,
    /// Lower a raised card-line back to place
    LowerCardLine,
    /// Add a card to the card-line if possible. This will take care of transform and hierarchy changes as well.
    AddToCardLine { card_entity: Entity },
    /// Remove a card from the card-line if found. This will take care of transform and hierarchy changes as well.
    RemoveCardFromLine { card_entity: Entity },
}

pub struct CardLineEventsPlugin;

impl Plugin for CardLineEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CardLineRequest>();
    }
}
