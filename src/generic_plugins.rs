use bevy_tween::interpolate::{Rotation, Scale, Translation};
use bevy_tween_helpers::prelude::{AnimationParentDestroyerGenericPlugin, TweenPriorityHandler, TweenTargetRemover};

use crate::{plugin_for_implementors_of_trait, prelude::*};

plugin_for_implementors_of_trait!(SendableGenericPlugins, Sendable);

pub struct GenericPlugins;

impl Plugin for GenericPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            SendableGenericPlugins::<Translation>::default(),
            SendableGenericPlugins::<Scale>::default(),
            SendableGenericPlugins::<Rotation>::default(),
        ));
    }
}

impl<T: Sendable> Plugin for SendableGenericPlugins<T> {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            TweenTargetRemover::<T>::default(),
            TweenPriorityHandler::<T>::default(),
            AnimationParentDestroyerGenericPlugin::<T>::default(),
        ));
    }
}
