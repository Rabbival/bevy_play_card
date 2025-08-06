use crate::prelude::*;

/// How a card line should act when picking cards when at picked card capacity
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CardPickingPolicy {
    #[default]
    /// Don't allow new cards to be picked until others are cancelled
    ForbidNewOnes,
    /// Unpick the least recently picked if there is any
    RemoveLeastRecentlyPicked,
    /// Unpick the latest card to be picked if there is any
    RemoveMostRecentlyPicked,
}

/// How a card line should act when picking cards when at picked card capacity
#[derive(Debug, Default, Clone)]
pub enum CardPickingPolicyWithContent {
    #[default]
    /// Don't allow new cards to be picked until others are cancelled
    ForbidNewOnes,
    /// Unpick the least recently picked if there is any
    RemoveLeastRecentlyPicked { picked_cards_in_order: Vec<Entity> },
    /// Unpick the latest card to be picked if there is any
    RemoveMostRecentlyPicked(Option<Entity>),
}

impl CardPickingPolicy {
    pub fn to_initial_with_content(&self) -> CardPickingPolicyWithContent {
        match self {
            Self::ForbidNewOnes => CardPickingPolicyWithContent::ForbidNewOnes,
            Self::RemoveLeastRecentlyPicked => {
                CardPickingPolicyWithContent::RemoveLeastRecentlyPicked {
                    picked_cards_in_order: Vec::new(),
                }
            }
            Self::RemoveMostRecentlyPicked => {
                CardPickingPolicyWithContent::RemoveMostRecentlyPicked(None)
            }
        }
    }
}
