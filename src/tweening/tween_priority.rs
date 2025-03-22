use bevy::prelude::Component;

/// Can be attached to either the parent animation
///
/// (applies to all tween children) or the specific tween.
///
/// If the tween has a specified priority,
///
/// it overrides that of its parent
///
/// IMPORTANT: It wouldn't work unless the tween type
///
/// is registered in the generic plugins.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub struct TweenPriorityToOthersOfType(pub u8);
