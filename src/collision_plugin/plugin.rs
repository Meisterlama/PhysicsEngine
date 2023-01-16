use std::time::Duration;

use bevy::prelude::*;
use bevy::time::FixedTimestep;
use bevy_inspector_egui::InspectorPlugin;

use crate::{
    collision_plugin::{
        config::CollisionConfig,
        data_structs::{BroadPhaseData, CollisionResponseData, NarrowPhaseData},
        systems,
    }
};
use crate::collision_plugin::{broad_phase, collision_response, debug, narrow_phase, rendering, TIMESTEP};
use crate::collision_plugin::rendering::LineBatches;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum CollisionStage
{
    PreSyncData,
    SyncData,
    BroadPhase,
    NarrowPhase,
    CollisionResponse,
    PostUpdate,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum StartupCollisionStage
{
    SyncData,
}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        // Resources

        app.init_resource::<CollisionConfig>();
        app.init_resource::<BroadPhaseData>();
        app.init_resource::<NarrowPhaseData>();
        app.init_resource::<CollisionResponseData>();
        app.init_resource::<LineBatches>();
        // Stages
        app.add_stage_after(
            CoreStage::PreUpdate,
            CollisionStage::PreSyncData,
            SystemStage::single_threaded()
                .with_system_set(Self::get_systems(CollisionStage::PreSyncData)),
        );
        app.add_stage_after(
            CoreStage::PreUpdate,
            CollisionStage::SyncData,
            SystemStage::single_threaded()
                .with_system_set(Self::get_systems(CollisionStage::SyncData)),
        );
        app.add_stage_after(
            CollisionStage::SyncData,
            CollisionStage::BroadPhase,
            SystemStage::single_threaded()
                .with_system_set(Self::get_systems(CollisionStage::BroadPhase))
                .with_run_criteria(FixedTimestep::step(TIMESTEP)),
        );
        app.add_stage_after(
            CollisionStage::BroadPhase,
            CollisionStage::NarrowPhase,
            SystemStage::single_threaded()
                .with_system_set(Self::get_systems(CollisionStage::NarrowPhase))
                .with_run_criteria(FixedTimestep::step(TIMESTEP)),
        );
        app.add_stage_after(
            CollisionStage::NarrowPhase,
            CollisionStage::CollisionResponse,
            SystemStage::single_threaded()
                .with_system_set(Self::get_systems(CollisionStage::CollisionResponse))
                .with_run_criteria(FixedTimestep::step(TIMESTEP)),
        );
        app.add_stage_after(
            CollisionStage::CollisionResponse,
            CollisionStage::PostUpdate,
            SystemStage::single_threaded()
                .with_system_set(Self::get_systems(CollisionStage::PostUpdate))
                .with_run_criteria(FixedTimestep::step(TIMESTEP)),
        );

        // Debug Plugins
        app.add_plugin(InspectorPlugin::<CollisionConfig>::new());
        app.add_startup_stage_after(
            StartupStage::PostStartup,
            StartupCollisionStage::SyncData,
            SystemStage::single_threaded()
                .with_system_set(Self::get_startup_systems(StartupCollisionStage::SyncData)),
        );
    }
}

impl CollisionPlugin {
    fn get_systems(stage: CollisionStage) -> SystemSet {
        match stage {
            CollisionStage::PreSyncData => {
                SystemSet::new()
            }
            CollisionStage::SyncData => {
                SystemSet::new()
                    .with_system(systems::refresh_entities)
                    .with_system(systems::aabb_update_system)
                    .with_system(clear_data)
            }
            CollisionStage::BroadPhase => {
                SystemSet::new()
                    .with_system(broad_phase::systems::broad_phase)
            }
            CollisionStage::NarrowPhase => {
                SystemSet::new()
                    .with_system(narrow_phase::systems::narrow_phase)
            }
            CollisionStage::CollisionResponse => {
                SystemSet::new()
                    .with_system(collision_response::systems::collision_response)
            }
            CollisionStage::PostUpdate => {
                SystemSet::new()
                    .with_system(debug::systems::draw_debug)
                    .with_system(debug::systems::update_debug_info)
                    .with_system(rendering::render_lines)
                    .with_system(debug::systems::refresh_polygon_lines)
                    .with_system(systems::update_rigidbodies)
                // .with_system(print_debug)
            }
        }
    }

    fn get_startup_systems(stage: StartupCollisionStage) -> SystemSet {
        match stage {
            StartupCollisionStage::SyncData => {
                SystemSet::new().with_system(systems::startup_refresh_entities)
            }
        }
    }
}

fn clear_data(
    mut narrow_phase_data: ResMut<NarrowPhaseData>,
    mut broad_phase_data: ResMut<BroadPhaseData>,
    mut collision_response_data: ResMut<CollisionResponseData>,
)
{
    narrow_phase_data.collision_infos.clear();

    narrow_phase_data.collided_entities.clear();
    broad_phase_data.collision_pairs.clear();

    narrow_phase_data.time = Duration::default();
    broad_phase_data.time = Duration::default();
    collision_response_data.time = Duration::default();
}

fn print_debug()
{
    warn!("COLLISION RESPONSE");
}