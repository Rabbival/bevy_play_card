use crate::prelude::*;

#[derive(Resource, Debug)]
pub struct CardConsts {
    pub card_hover_height: f32,
    pub on_hover_scale_factor: f32,
    pub on_hover_scale_duration: f32,
    pub on_hover_cancel_scale_duration: f32,
    pub on_hover_position_tween_duration: f32,
    pub on_hover_cancel_position_tween_duration: f32,
    pub go_back_to_place_tween_duration: f32,
    pub card_slide_on_origin_set_duration: f32,
    pub allow_hover_while_dragging: bool,
}
