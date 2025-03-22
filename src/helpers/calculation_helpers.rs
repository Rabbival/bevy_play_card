use crate::prelude::*;
use bevy::math::NormedVectorSpace;
use std::f32::consts::PI;

pub fn projection_directed_distance(
    point_to_project: Vec2,
    line_projected_onto: Vec2,
    measure_distance_from: Vec2,
) -> f32 {
    let projection_on_line =
        (point_to_project - measure_distance_from).project_onto(line_projected_onto);
    let projection_direction = if projection_on_line.angle_to(line_projected_onto).abs() < PI / 2.0
    {
        1.0
    } else {
        -1.0
    };
    projection_on_line.norm() * projection_direction
}
