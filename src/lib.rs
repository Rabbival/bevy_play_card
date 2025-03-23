#![allow(clippy::type_complexity)]

pub mod cards;
mod generic_plugins;
pub mod trait_unions;
pub mod tweening;

#[macro_use]
pub mod macros;
pub mod bevy_card_plugin;
pub mod utilities;

pub mod prelude {
    pub use crate::bevy_card_plugin::*;
    pub use crate::cards::{
        card::*,
        card_bundle::*,
        card_dragging_manager::*,
        card_hovering_manager::*,
        card_lines::{
            card_line::*, card_line_bundle::*, card_lines_content_manager::*, card_lines_mover::*,
            event::*, CardLinesPlugin,
        },
        card_mover::*,
        card_namer::*,
        event::*,
        tags::*,
        CardsPlugin,
    };
    pub use crate::trait_unions::*;
    pub use crate::tweening::{
        animation_parent_destoryer::*, custom_combinators::*, tween_destoryer::*,
        tween_priority::*, tween_request::*, TweeningPlugin,
    };
    pub use bevy::{prelude::*, utils::HashMap};
    pub use bevy_tween::*;
    pub use std::marker::PhantomData;
    pub use tween_event::*;
}
