use bevy::prelude::*;
use bevy::time::FixedTimestep;
#[allow(unused_imports)]
use bevy_inspector_egui::Inspectable;
#[allow(unused_imports)]
use bevy_inspector_egui::InspectorPlugin;

use config::CollisionConfig;
use refresh_entities::refresh_entities;

mod broad_phase;
mod collision_structs;
mod narrow_phase;
mod config;
mod draw_debug;
mod debug_system;
mod refresh_entities;
mod collision_response;
pub mod rigidbody;

pub struct CollisionPlugin;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum CollisionStage
{
    PreUpdate,
    BroadPhase,
    NarrowPhase,
    CollisionResponse,
    PostUpdate,
}

#[derive(Default, Component)]
pub struct PhysicsAwake;

const TIMESTEP: f64 = 1.0 /120.0;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        // Resources
        app
            .init_resource::<CollisionConfig>();

        // Stages
        app
            .add_stage(CollisionStage::PreUpdate, SystemStage::parallel())
            .add_stage_after(CollisionStage::PreUpdate, CollisionStage::BroadPhase, SystemStage::parallel())
            .add_stage_after(CollisionStage::BroadPhase, CollisionStage::NarrowPhase, SystemStage::parallel())
            .add_stage_after(CollisionStage::NarrowPhase, CollisionStage::CollisionResponse, SystemStage::parallel())
            .add_stage_after(CollisionStage::CollisionResponse, CollisionStage::PostUpdate, SystemStage::parallel());

        app.add_system_to_stage(CollisionStage::PreUpdate, refresh_entities);

        // CollisionPlugins
        app
            .add_plugin(broad_phase::BroadPhasePlugin)
            .add_plugin(narrow_phase::NarrowPhasePlugin)
            .add_plugin(collision_response::CollisionResponsePlugin);

        // Debug Plugins
        app
            .add_plugin(InspectorPlugin::<CollisionConfig>::new())
            .add_system_to_stage(CollisionStage::PostUpdate, draw_debug::draw_debug)
            .add_system_to_stage(CollisionStage::PostUpdate, debug_system::update_debug_info);
    }
}