use crate::prelude::*;
use bevy_tween::combinator::AnimationBuilderExt;
use bevy_tween::interpolation::EaseKind;
use bevy_tween::prelude::*;
use std::collections::HashMap;
use std::time::Duration;

pub struct CardLinesMoverPlugin;

impl Plugin for CardLinesMoverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, listen_to_card_line_move_requests);
    }
}

fn listen_to_card_line_move_requests(
    mut card_line_request_listener: EventReader<CardLineRequest>,
    card_lines: Query<(&CardLine, &Transform, Entity), With<CardLine>>,
    mut commands: Commands,
) {
    // Using a hashmap to allow both requests from other entities
    // and automatic value overriding for getting the newest destination only
    let mut destination_by_card_line: HashMap<Entity, Vec3> = HashMap::new();
    for move_request in card_line_request_listener.read() {
        if let Ok((card_line, card_line_transform, card_line_entity)) =
            card_lines.get(move_request.line)
        {
            let destination = match move_request.request_type {
                CardLineRequestType::RaiseLine => {
                    card_line.origin.translation
                        + card_line_transform.up() * card_line.raised_card_line_delta
                }
                CardLineRequestType::LowerLine => card_line.origin.translation,
                _ => continue,
            };
            destination_by_card_line.insert(card_line_entity, destination);
        }
    }
    for (card_line_entity, destination) in destination_by_card_line {
        if let Ok((card_line, card_line_transform, _)) = card_lines.get(card_line_entity) {
            let animation_target = card_line_entity.into_target();
            let mut transform_state = animation_target.transform_state(*card_line_transform);
            commands
                .spawn((
                    Name::new("Card line slide animation parent"),
                    TweenPriorityToOthersOfType(20),
                ))
                .animation()
                .insert(named_tween(
                    Duration::from_secs_f32(card_line.slide_duration),
                    EaseKind::CubicOut,
                    transform_state.translation_to(destination),
                    String::from("card line slide (transation) tween"),
                ));
        }
    }
}
