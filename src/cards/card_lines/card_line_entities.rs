use crate::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct CardLineEntities {
    pub player_card_line: Option<Entity>,
    pub debug_card_lines: Vec<Entity>,
}

pub struct CardLineEntitiesPlugin;

impl Plugin for CardLineEntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CardLineEntities>();
    }
}
