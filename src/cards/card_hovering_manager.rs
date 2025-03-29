use crate::cards::card_consts::CardConsts;
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
    mut trigger: Trigger<Pointer<Over>>,
    cards: Query<(&Transform, Entity, &Card, &Name)>,
    card_consts: Res<CardConsts>,
    mut commands: Commands,
) {
    trigger.propagate(false);
    if let Ok((transform, entity, card, name)) = cards.get(trigger.target) {
        commands.entity(entity).insert(Hovered);
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
                    Duration::from_secs_f32(card_consts.on_hover_scale_duration),
                    EaseKind::Linear,
                    transform_state.scale_to(card_consts.on_hover_scale_factor * card.origin.scale),
                    format!("{} on-hover scaling tween", name),
                ),
                named_tween(
                    Duration::from_secs_f32(card_consts.on_hover_position_tween_duration),
                    EaseKind::CubicOut,
                    transform_state.translation_to(
                        card.origin
                            .translation
                            .with_y(card_consts.card_hover_height),
                    ),
                    format!("{} on-hover translation tween", name),
                ),
            )));
    }
}

fn on_hover_cancel(
    mut trigger: Trigger<Pointer<Out>>,
    cards: Query<(&Transform, Entity, &Card, &Name), (Without<Dragged>, Without<Picked>)>,
    card_consts: Res<CardConsts>,
    mut commands: Commands,
) {
    trigger.propagate(false);
    if let Ok((transform, entity, card, name)) = cards.get(trigger.target) {
        commands.entity(entity).remove::<Hovered>();
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
                    Duration::from_secs_f32(card_consts.on_hover_cancel_scale_duration),
                    EaseKind::Linear,
                    transform_state.scale_to(card.origin.scale),
                    format!("{} on-hover-cancel scale tween", name),
                ),
                named_tween(
                    Duration::from_secs_f32(card_consts.on_hover_cancel_position_tween_duration),
                    EaseKind::CubicOut,
                    transform_state.translation_to(card.origin.translation),
                    format!("{} on-hover-cancel translation tween", name),
                ),
            )));
    }
}
