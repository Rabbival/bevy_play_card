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
/// Requests that add or remove cards take care of hierarchy and transforms automatically
#[derive(Debug)]
pub enum CardLineRequestType {
    /// Raise the card-line's position, depending on the line Transform up direction
    RaiseLine,
    /// Lower a raised card-line back to place
    LowerLine,
    /// Add a card to the card-line if possible.
    AddToLine { card_entity: Entity },
    /// Remove a card from the card-line if found.
    RemoveFromLine { card_entity: Entity },
    /// Add multiple cards to the card-line if possible (starting from the first).
    BatchAddToLine { card_entities: Vec<Entity> },
    /// Remove multiple cards from the card-line if found.
    BatchRemoveFromLine { card_entities: Vec<Entity> },
    /// Removes all the cards from the card-line
    RemoveAllCardsFromLine,
}

pub struct CardLineEventsPlugin;

impl Plugin for CardLineEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CardLineRequest>();
    }
}
