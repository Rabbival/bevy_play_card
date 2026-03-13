use crate::prelude::*;

/// A request to be handled automatically by bevy_play_card_crate
#[derive(Debug, Message, EntityEvent, Clone)]
pub struct CardLineRequest {
    /// The line entity to which the request should apply
    pub entity: Entity,
    /// The request type, including additional data if there's any
    pub request_type: CardLineRequestType,
}

/// The type of request to be applied to the line, including additional data if there's any
/// Requests that add or remove cards take care of hierarchy and transforms automatically
#[derive(Debug, Clone)]
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
    /// Puts the cards of the line in their correct places, redundant if auto-sorted
    Sort
}

/// Event as a result to a request to signal success or failure of the request
/// The result when successful is the quantity of card in the line 
/// In case of failure it is the reason of the failure and the card that failed if possible
#[derive(Debug, Message, EntityEvent)]
pub struct CardLineRequestResult {
    /// The line entity which processed the request
    pub entity: Entity,
    /// The result of the request
    pub result: Result<usize,(CardLineRequestFailure, Option<Entity>)>,
    /// The request processed
    pub linked_request: CardLineRequest,
}


/// Reason why an action failed
#[derive(Debug)]
pub enum CardLineRequestFailure {
    MaximumLineCapacityReached
}


pub struct CardLineEventsPlugin;

impl Plugin for CardLineEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<CardLineRequest>()
           .add_message::<CardLineRequestResult>();
    }
}
