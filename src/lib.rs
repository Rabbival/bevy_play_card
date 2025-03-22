#![allow(clippy::type_complexity)]

#[macro_use]
extern crate lazy_static;
mod animation;
mod app;
mod audio;
mod common_logic;
mod debug;
mod ecs;
mod game;
mod input;
mod os_access;
mod physics;
mod time;
mod trait_unions;
mod tweening;
mod ui;

#[macro_use]
mod macros;

pub mod prelude {
    pub use crate::animation::{
        consts::*,
        default_color::*,
        events::*,
        frame_animation::{
            frame_animation_spawner::*, frame_animator::*, frame_animator_by_folder::*,
            FrameAnimationPlugin,
        },
        particles::{blood_particles::*, consts::*, ParticlesPlugin},
        sprite_color_animation::*,
        MyAnimationPlugin,
    };
    pub use crate::app::{
        app_manager::*, consts::*, generic_plugins::*, main, main_camera::*, screen_setup::*,
        tags::*,
    };
    pub use crate::audio::{
        audio_channels::*,
        consts::*,
        loop_handle::*,
        music::{
            consts::*, music_layers::*, music_layers_by_folder::*, music_manager::*, MusicPlugin,
        },
        sound_randomizer_by_folder::*,
        sounds_randomizer::*,
        spatial_audio_emitter_bundle::*,
        MyAudioPlugin,
    };
    pub use crate::common_logic::{
        action_performed::*, argument_validation::*, calculation_helpers::*,
        enums::basic_direction::*, mismatch_error::*, vector_utilities::*,
    };
    pub use crate::debug::{
        consts::*,
        enums::{bevy_log_level::*, feature_toggle::*, log_category::*},
        events::*,
        game_session_log::*,
        gizmos::*,
        inspector::*,
        print_config_struct::*,
        print_log::*,
        print_vec::*,
        DebugPlugin,
    };
    pub use crate::ecs::{
        component_utilities::*,
        custom_run_conditions::*,
        despawning::{despawn_request::*, despawn_request_handler::*, DespawningPlugin},
        entity_error::*,
        system_sets::*,
        EcsPlugin,
    };
    pub use crate::game::{
        consts::*,
        damage::{
            current_hp::*, damage_causer::*, damage_layer::*, damage_manager::*, damage_taker::*,
            events::*, DamagePlugin,
        },
        enemies::{
            enemies_chasing_player::*,
            enemy_bundle::*,
            enemy_namer::*,
            stone_head::{
                consts::*, stone_head_animation_controller::*, stone_head_chasing::*,
                stone_head_sounds_manager::*, stone_head_spawner::*,
                stone_head_sprite_animators::*, stone_head_visual_state::*, StoneHeadEnemyPlugin,
            },
            tags::*,
            EnemiesPlugin,
        },
        game_manager::*,
        game_state::*,
        hostiles::{consts::*, events::*, hostile::*, hostiles_manager::*, HostilesPlugin},
        noise_circles::{noise_circle::*, noise_circles_manager::*, NoiseCirclesPlugin},
        orbs::{
            consts::*, events::*, orb::*, orb_collector::*, orb_count::*, orb_sounds_manager::*,
            orb_spawner::*, orb_sprite_animators::*, OrbsPlugin,
        },
        player::{
            consts::*, events::*, player::*, player_animation_controller::*, player_health::*,
            player_movement::*, player_sounds_manager::*, player_spawner::*,
            player_sprite_animators::*, player_visual_state::*, PlayerPlugin,
        },
        tags::*,
        unchangeable_velocity::*,
        y_sorting::*,
        GamePlugin,
    };
    pub use crate::input::{
        actions::{player_action::*, ui_action::*},
        cursor_world_position::*,
        ignore_requests_component::*,
        input_error::*,
        input_maps::{player_input_map::*, ui_input_map::*, InputMapsPlugin},
        player_input::*,
        ui_input::*,
        InputPlugin,
    };
    pub use crate::os_access::{
        asset_loaders::{
            music_loader::*,
            sound_effect_loaders::{
                consts::*,
                enemy_sounds_loaders::{
                    consts::*, stone_head_sounds_loader::*, EnemySoundsLoadersPlugin,
                },
                orb_sound_effect_loader::*,
                player_sound_effect_loader::*,
                SoundEffectLoaderPlugin,
            },
            sprite_loaders::{
                consts::*,
                enemy_sprite_loaders::{
                    consts::*, stone_head_sprite_loader::*, EnemySpriteLoadersPlugin,
                },
                orb_sprite_loader::*,
                player_sprite_loader::*,
                ui_sprite_loader::*,
                SpriteLoaderPlugin,
            },
            utility_functions::*,
            AssetLoaderPlugin,
        },
        folder_access::{
            file_path_to_asset_loader_path, file_path_traits::*, folder_access_functions::*,
            folder_to_access::*, music_folder::*, sound_effects_folder::*, sprite_folders::*,
        },
        os_access_error::*,
        os_access_log::*,
        system_file_name::*,
        system_file_type::*,
        text_file_access::*,
    };
    pub use crate::physics::{
        bound_to_world::*, bounds_keeper::*, consts::*, events::*, movement_acceleration::*,
        movement_damping_factor::*, physics_bundle::*, physics_layers::*, recoil::*, PhysicsPlugin,
    };
    pub use crate::time::{
        consts::*,
        time_scalers::{
            consts::*, errors::*, events::*, time_scaled_timer::*, time_scaler::*,
            time_scaler_id::*, time_scaler_interpolator::*, time_scaler_management::*,
            time_scaler_subscriber::*, time_scaler_utility_functions::*, TimeScalingPlugin,
        },
        TimePlugin,
    };
    pub use crate::trait_unions::*;
    pub use crate::tweening::{
        animation_parent_destoryer::*, custom_combinators::*, tween_destoryer::*,
        tween_priority::*, tween_request::*, TweeningPlugin,
    };
    pub use crate::ui::{
        app_margins_ui::{
            app_margins_spawner::*,
            app_margins_sprites::*,
            consts::*,
            player_health_bar::{health_bar::*, health_bar_manager::*, PlayerHealthBarPlugin},
            tags::*,
            ui_orbs::*,
            AppMarginsUiPlugin,
        },
        cards::{
            card::*,
            card_bundle::*,
            card_dragging_manager::*,
            card_hovering_manager::*,
            card_lines::{
                card_line::*, card_line_bundle::*, card_line_entities::*,
                card_lines_content_manager::*, card_lines_mover::*, card_lines_spawner::*,
                consts::*, event::*, CardLinesPlugin,
            },
            card_mover::*,
            card_namer::*,
            cards_spawner::*,
            consts::*,
            event::*,
            tags::*,
            CardsPlugin,
        },
        UiPlugin,
    };
    pub use avian2d::prelude::*;
    pub use bevy::{prelude::*, utils::HashMap};
    pub use bevy_tween::*;
    pub use std::marker::PhantomData;
    pub use tween_event::*;
}
