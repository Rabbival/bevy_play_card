use crate::prelude::*;
use bevy_tween::combinator::parallel;
use bevy_tween::prelude::*;

pub struct CardHoveringPlugin;

impl Plugin for CardHoveringPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_hover).add_observer(on_hover_cancel);
    }
}

fn on_hover(
    drag: Trigger<Pointer<Over>>,
    cards: Query<(&Transform, Entity, &Card, &Name)>,
    mut commands: Commands,
) {
    if let Ok((transform, entity, card, name)) = cards.get(drag.entity()) {
        let animation_target = entity.into_target();
        let mut transform_state = animation_target.transform_state(*transform);
        commands
            .spawn((
                Name::new(format!("On-hover animation parent for {}", name)),
                TweenPriorityToOthersOfType(0),
            ))
            .animation()
            .insert(parallel((
                named_tween(
                    Duration::from_secs_f32(ON_HOVER_SCALE_DURATION),
                    EaseKind::Linear,
                    transform_state.scale_to(ON_HOVER_SCALE_FACTOR * card.origin.scale),
                    format!("{} on-hover scaling tween", name),
                ),
                named_tween(
                    Duration::from_secs_f32(ON_HOVER_POSITION_TWEEN_DURATION),
                    EaseKind::CubicOut,
                    transform_state
                        .translation_to(card.origin.translation.with_y(CARD_HOVER_HEIGHT)),
                    format!("{} on-hover translation tween", name),
                ),
            )));
    }
}

fn on_hover_cancel(
    drag: Trigger<Pointer<Out>>,
    cards: Query<(&Transform, Entity, &Card, &Name), Without<Dragged>>,
    mut commands: Commands,
) {
    if let Ok((transform, entity, card, name)) = cards.get(drag.entity()) {
        let animation_target = entity.into_target();
        let mut transform_state = animation_target.transform_state(*transform);
        commands
            .spawn((
                Name::new(format!("On-hover-cancel animation parent for {}", name)),
                TweenPriorityToOthersOfType(1),
            ))
            .animation()
            .insert(parallel((
                named_tween(
                    Duration::from_secs_f32(ON_HOVER_CANCEL_SCALE_DURATION),
                    EaseKind::Linear,
                    transform_state.scale_to(card.origin.scale),
                    format!("{} on-hover-cancel scale tween", name),
                ),
                named_tween(
                    Duration::from_secs_f32(ON_HOVER_CANCEL_POSITION_TWEEN_DURATION),
                    EaseKind::CubicOut,
                    transform_state.translation_to(card.origin.translation),
                    format!("{} on-hover-cancel translation tween", name),
                ),
            )));
    }
}
