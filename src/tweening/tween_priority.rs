use bevy::prelude::Component;

/// When there's a conflict between two existing tweens of the same type
/// (say, two position tweens on the same entity)
/// one of them is destroyed (either the one with the lesser priority and if equal- the older one).
///
/// This component can be attached to either the parent animation
/// (applies to all tween children) or the specific tween.
/// If the tween has a specified priority, it overrides that of its parent
///
/// IMPORTANT: For tween type not added here, it wouldn't work unless the tween type
/// is registered in the generic plugins.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub struct TweenPriorityToOthersOfType(pub u32);
