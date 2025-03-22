use interpolate::*;

use crate::{plugin_for_implementors_of_trait, prelude::*};

plugin_for_implementors_of_trait!(SendableGenericPlugins, Sendable);

pub struct GenericPlugins;

impl Plugin for GenericPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            SendableGenericPlugins::<Translation>::default(),
            SendableGenericPlugins::<Scale>::default(),
            SendableGenericPlugins::<interpolate::Rotation>::default(),
            SendableGenericPlugins::<SpriteColor>::default(),
            SendableGenericPlugins::<DamageCauserInterpolator>::default(),
            SendableGenericPlugins::<FrameAnimator>::default(),
            SendableGenericPlugins::<TimeScalerInterpolator>::default(),
        ));
    }
}

impl<T: Sendable> Plugin for SendableGenericPlugins<T> {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            TweenDestroyerPlugin::<T>::default(),
            AnimationParentDestroyerGenericPlugin::<T>::default(),
        ));
    }
}
