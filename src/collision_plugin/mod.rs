use std::time::Duration;
use bevy::prelude::*;
use bevy::time::Stopwatch;

#[allow(unused_imports)]
use bevy_inspector_egui::Inspectable;
#[allow(unused_imports)]
use bevy_inspector_egui::InspectorPlugin;

use crate::collision_plugin::collision_structs::{CollisionInfo, CollisionPair};

use crate::collision_plugin::config::CollisionConfig;

mod broad_phase;
mod collision_structs;
mod narrow_phase;
mod config;
mod sat;
mod draw_debug;
mod debug_system;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CollisionConfig>()
            .init_resource::<SystemData>()
            .add_system(broad_phase::broad_phase)
            .add_system(narrow_phase::narrow_phase.after(broad_phase::broad_phase))
            .add_system(draw_debug::draw_debug.after(narrow_phase::narrow_phase));

            app.add_plugin(InspectorPlugin::<CollisionConfig>::new());
            app.add_system(debug_system::update_debug_info.after(narrow_phase::narrow_phase));
    }
}

#[derive(Default, Resource)]
pub struct SystemData
{
    pub broad_phase_collision_pairs: Option<Vec<CollisionPair>>,
    pub narrow_phase_collision_infos: Option<Vec<CollisionInfo>>,

    pub broad_time: Duration,
    pub narrow_time: Duration,
}