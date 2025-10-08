use crate::prelude::*;
use bevy_tween::tween_event::TweenEventPlugin;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, Hash, PartialOrd, Default)]
pub struct DeclareDraggingDoneForCard {
    pub card_entity: Option<Entity>,
}

#[derive(Debug, Clone, Copy, Message)]
pub struct TogglePickingForCard(pub Entity);

#[derive(Debug, Clone, Copy, Message)]
pub struct CardAnimationRequest {
    pub card_entity: Entity,
    pub request_type: CardAnimationRequestType,
}

#[derive(Debug, Clone, Copy)]
pub enum CardAnimationRequestType {
    FloatBackDown,
}

pub struct CardsEventsPlugin;

impl Plugin for CardsEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TweenEventPlugin::<DeclareDraggingDoneForCard>::default())
            .add_message::<TogglePickingForCard>()
            .add_message::<CardAnimationRequest>();
    }
}
