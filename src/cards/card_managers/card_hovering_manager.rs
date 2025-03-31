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
    cards: Query<(), With<Card>>,
    mut commands: Commands,
) {
    trigger.propagate(false);
    if cards.get(trigger.target).is_ok() {
        commands.entity(trigger.target).insert(Hovered);
    }
}

fn on_hover_cancel(
    mut trigger: Trigger<Pointer<Out>>,
    cards: Query<(&Transform, Entity, &Card, &Name, Option<&Dragged>), Without<Picked>>,
    card_consts: Res<CardConsts>,
    mut commands: Commands,
) {
    trigger.propagate(false);
    if let Ok((transform, entity, card, name, maybe_dragged)) = cards.get(trigger.target) {
        commands.entity(entity).remove::<Hovered>();
        if maybe_dragged.is_some() {
            return;
        }
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
