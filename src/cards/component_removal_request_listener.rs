use bevy_tween::prelude::TweenEvent;

use crate::prelude::*;

pub struct ComponentRemovalRequestListenerPlugin;

impl Plugin for ComponentRemovalRequestListenerPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_card_component_removal_request::<Dragged>)
            .add_observer(on_card_component_removal_request::<MovingToNewOrigin>);
    }
}

fn on_card_component_removal_request<C: Component>(
    trigger: On<TweenEvent<RemoveComponentFromCardTweenRequest<C>>>,
    cards: Query<(), With<Card>>,
    mut commands: Commands,
) {
    if let Some(entity) = trigger.data.card_entity
        && cards.contains(entity)
        && let Ok(mut entity_commands) = commands.get_entity(entity)
    {
        entity_commands.remove::<C>();
    }
}
