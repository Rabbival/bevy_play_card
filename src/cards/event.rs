use crate::prelude::*;
use bevy_tween_alt::tween_event::TweenEventPlugin;
use bevy_tween_helpers::prelude::ScheduleLabel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, Hash, PartialOrd, Default)]
pub struct RemoveComponentFromCardTweenRequest<C: Component> {
    pub card_entity: Option<Entity>,
    phantom: PhantomData<C>,
}

impl<C: Component> RemoveComponentFromCardTweenRequest<C> {
    pub fn new(entity: Entity) -> Self {
        Self {
            card_entity: Some(entity),
            phantom: PhantomData::<C>::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, Message, EntityEvent)]
pub struct TogglePickingForCard(pub Entity);

#[derive(Debug, Clone, Copy, Message, EntityEvent)]
pub struct CardAnimationRequest {
    pub entity: Entity,
    pub request_type: CardAnimationRequestType,
}

#[derive(Debug, Clone, Copy)]
pub enum CardAnimationRequestType {
    FloatBackDown,
    FloatUp { tween_name: &'static str },
}

pub struct CardsEventsPlugin;

impl Plugin for CardsEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TweenEventPlugin::<
            RemoveComponentFromCardTweenRequest<Dragged>,
        >::in_schedule(PostUpdate.intern()))
            .add_plugins(TweenEventPlugin::<
                RemoveComponentFromCardTweenRequest<MovingToNewOrigin>,
            >::in_schedule(PostUpdate.intern()))
            .add_message::<TogglePickingForCard>()
            .add_message::<CardAnimationRequest>();
    }
}
