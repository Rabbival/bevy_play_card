use crate::prelude::*;

pub mod card;
pub mod card_bundle;
pub mod card_consts;
pub mod card_lines;
pub mod card_managers;
pub mod card_namer;
pub mod event;
pub mod tags;

pub struct CardsPlugin {
    pub logging_function: Option<fn(String)>,
}

#[derive(Resource)]
pub struct CardsPluginLoggingFunction(pub Option<fn(String)>);

impl Plugin for CardsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CardsPluginLoggingFunction(self.logging_function.clone()))
            .add_plugins((
                CardsEventsPlugin,
                CardManagersPlugin,
                CardLinesPlugin,
                CardNamerPlugin,
            ));
    }
}
