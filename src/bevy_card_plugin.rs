use crate::cards::card_consts::CardConsts;
use crate::generic_plugins::GenericPlugins;
use crate::prelude::*;

pub struct BevyCardPlugin {
    pub card_hover_height: f32,
    pub on_hover_scale_factor: f32,
    pub on_hover_scale_duration: f32,
    pub on_hover_cancel_scale_duration: f32,
    pub on_hover_position_tween_duration: f32,
    pub on_hover_cancel_position_tween_duration: f32,
    pub go_back_to_place_tween_duration: f32,
    pub card_slide_on_origin_set_duration: f32,
}

impl Plugin for BevyCardPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((GenericPlugins, TweeningPlugin, CardsPlugin))
            .insert_resource(CardConsts {
                card_hover_height: self.card_hover_height,
                on_hover_scale_factor: self.on_hover_scale_factor,
                on_hover_scale_duration: self.on_hover_scale_duration,
                on_hover_cancel_scale_duration: self.on_hover_cancel_scale_duration,
                on_hover_position_tween_duration: self.on_hover_position_tween_duration,
                on_hover_cancel_position_tween_duration: self
                    .on_hover_cancel_position_tween_duration,
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
        }
    }
}
