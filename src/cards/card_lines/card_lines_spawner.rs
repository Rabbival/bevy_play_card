use crate::prelude::*;
use std::f32::consts::PI;

pub struct CardLinesSpawnerPlugin;

impl Plugin for CardLinesSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, spawn_card_line);
    }
}

fn spawn_card_line(mut commands: Commands, mut card_line_entities: ResMut<CardLineEntities>) {
    let player_card_line = commands
        .spawn(CardLineBundle::new(Transform::from_xyz(
            0.0,
            CARD_LINE_Y,
            CARD_LINE_Z,
        )))
        .id();
    if FeatureToggle::SpawnDebugCardLines.enabled() {
        for (rotation, location) in [
            (PI * 0.5, Vec3::new(400.0, 0.0, CARD_LINE_Z)),
            (PI, Vec3::new(0.0, 400.0, CARD_LINE_Z)),
            (PI * 1.5, Vec3::new(-400.0, 0.0, CARD_LINE_Z)),
        ] {
            card_line_entities.debug_card_lines.push(
                commands
                    .spawn(CardLineBundle::new(
                        Transform::from_translation(location.into())
                            .with_rotation(Quat::from_rotation_z(rotation)),
                    ))
                    .id(),
            );
        }
    }
    card_line_entities.player_card_line = Some(player_card_line);
}
