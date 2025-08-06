use crate::prelude::*;
use bevy_tween::prelude::EaseKind;

#[derive(Resource, Debug, Clone, Copy)]
pub struct CardConsts {
    /// How far up cards float when hovered in pixels
    pub card_hover_height: f32,
    /// How big cards get when hovered compared to its origin scale
    pub on_hover_scale_factor: f32,
    /// How many seconds it takes for cards to grow in size when hovered
    pub on_hover_scale_duration: f32,
    /// How many seconds it takes for cards to float up when hovered
    pub on_hover_position_tween_duration: f32,
    /// How many seconds it takes for cards to shrink back to their origin size when no longer hovered or picked
    pub on_float_back_down_scale_duration: f32,
    /// How many seconds it takes for cards to float back to their origin position when no longer hovered or picked
    pub on_float_back_down_position_tween_duration: f32,
    /// How many seconds it takes for dragged cards to get back into their origin position
    pub go_back_to_place_tween_duration: f32,
    /// How many seconds it takes for cards to get to their new origin once set
    pub card_slide_on_origin_set_duration: f32,
    /// What should pointer delta be multiplied by when dragging
    pub card_drag_delta_scaler: Vec2,
    /// Whether cards can be hovered over while actively dragging a card
    pub allow_hover_while_dragging: bool,
    /// Ease kind for card movement when their origin changes
    pub card_origin_set_ease_kind: EaseKind,
}

impl Default for CardConsts {
    fn default() -> Self {
        Self {
            card_hover_height: 80.0,
            on_hover_scale_factor: 1.4,
            on_hover_scale_duration: 0.04,
            on_float_back_down_scale_duration: 0.03,
            on_hover_position_tween_duration: 0.2,
            on_float_back_down_position_tween_duration: 0.1,
            go_back_to_place_tween_duration: 0.04,
            card_slide_on_origin_set_duration: 0.2,
            card_drag_delta_scaler: Vec2::ONE,
            allow_hover_while_dragging: false,
            card_origin_set_ease_kind: EaseKind::CubicOut,
        }
    }
}
