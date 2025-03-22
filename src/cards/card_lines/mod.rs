use crate::prelude::*;

pub mod card_line;
pub mod card_line_bundle;
pub mod card_line_entities;
pub mod card_lines_content_manager;
pub mod card_lines_mover;
pub mod card_lines_spawner;
pub mod consts;
pub mod event;

pub struct CardLinesPlugin;

impl Plugin for CardLinesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            CardLinesContentManagerPlugin,
            CardLineEventsPlugin,
            CardLinesSpawnerPlugin,
            CardLinesMoverPlugin,
            CardLineEntitiesPlugin,
        ));
    }
}
