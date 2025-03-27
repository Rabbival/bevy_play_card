use crate::cards::card_consts::CardConsts;
use crate::generic_plugins::GenericPlugins;
use crate::prelude::*;
use crate::utilities::system_sets::CardsSystemSetsPlugin;

///A plugin to add all bevy_play_card types and automations
pub struct BevyCardPlugin {
    /// How far up cards float when hovered in pixels
    pub card_hover_height: f32,
    /// How big cards get when hovered compared to its origin scale
    pub on_hover_scale_factor: f32,
    /// How many seconds it takes for cards to grow in size when hovered
    pub on_hover_scale_duration: f32,
    /// How many seconds it takes for cards to shrink back to their origin size when no longer hovered
    pub on_hover_cancel_scale_duration: f32,
    /// How many seconds it takes for cards to float up when hovered
    pub on_hover_position_tween_duration: f32,
    /// How many seconds it takes for cards to float back to their origin position when no longer hovered
    pub on_hover_cancel_position_tween_duration: f32,
    /// How many seconds it takes for dragged cards to get back into their origin position
    pub go_back_to_place_tween_duration: f32,
    /// How many seconds it takes for cards to get to their new origin once set
    pub card_slide_on_origin_set_duration: f32,
    /// Whether debug logs should be printed for TweeningPlugin
    pub enable_tweening_debug_logs: bool,
    /// Whether debug logs should be printed for CardsPlugin
    pub enable_cards_debug_logs: bool,
}

impl Plugin for BevyCardPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            GenericPlugins,
            TweeningPlugin {
                print_debug_logs: self.enable_tweening_debug_logs,
            },
            CardsPlugin {
                print_debug_logs: self.enable_cards_debug_logs,
            },
            CardsSystemSetsPlugin,
        ))
        .insert_resource(CardConsts {
            card_hover_height: self.card_hover_height,
            on_hover_scale_factor: self.on_hover_scale_factor,
            on_hover_scale_duration: self.on_hover_scale_duration,
            on_hover_cancel_scale_duration: self.on_hover_cancel_scale_duration,
            on_hover_position_tween_duration: self.on_hover_position_tween_duration,
            on_hover_cancel_position_tween_duration: self.on_hover_cancel_position_tween_duration,
            go_back_to_place_tween_duration: self.go_back_to_place_tween_duration,
            card_slide_on_origin_set_duration: self.card_slide_on_origin_set_duration,
        });
    }
}

impl Default for BevyCardPlugin {
    fn default() -> Self {
        Self {
            card_hover_height: 80.0,
            on_hover_scale_factor: 1.4,
            on_hover_scale_duration: 0.04,
            on_hover_cancel_scale_duration: 0.03,
            on_hover_position_tween_duration: 0.2,
            on_hover_cancel_position_tween_duration: 0.1,
            go_back_to_place_tween_duration: 0.04,
            card_slide_on_origin_set_duration: 0.2,
            enable_tweening_debug_logs: false,
            enable_cards_debug_logs: false,
        }
    }
}
