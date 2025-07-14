use bevy_tween::DefaultTweenPlugins;
use bevy_tween_helpers::prelude::BevyTweenHelpersPlugin;
use crate::cards::card_consts::CardConsts;
use crate::generic_plugins::GenericPlugins;
use crate::prelude::*;
use crate::utilities::system_sets::CardsSystemSetsPlugin;

///A plugin to add all bevy_play_card types and automations
pub struct BevyCardPlugin {
    /// Different factors that affect the way cards behave relative to one another, their line and the pointer
    pub card_consts: CardConsts,
    /// Here you can insert your own function for logging BevyTweenHelpersPlugin
    pub tweening_debug_logging_function: Option<fn(String)>,
    /// Here you can insert your own function for logging CardsPlugin
    pub card_debug_logging_function: Option<fn(String)>,
}

impl Plugin for BevyCardPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            GenericPlugins,
            DefaultTweenPlugins,
            BevyTweenHelpersPlugin {
                logging_function: self.tweening_debug_logging_function.clone(),
            },
            CardsPlugin {
                logging_function: self.card_debug_logging_function.clone(),
            },
            CardsSystemSetsPlugin,
        ))
        .insert_resource(self.card_consts);
    }
}

impl Default for BevyCardPlugin {
    fn default() -> Self {
        Self {
            card_consts: CardConsts::default(),
            tweening_debug_logging_function: None,
            card_debug_logging_function: None,
        }
    }
}
