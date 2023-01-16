use std::f32::consts::PI;

use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

use crate::collision_plugin::aabb::AABB;
use crate::collision_plugin::plugin::CollisionStage;
use crate::collision_plugin::polygon_component::PolygonComponent;
use crate::collision_plugin::rigidbody::RigidBody2d;
use crate::MainCamera;
use crate::transform2d::Transform2d;

pub struct DrawPolygonPlugin;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct EntityToMove;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct EntityToRotate;

fn select_polygons(
    mut commands: Commands,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut q_polygons: Query<(Entity, &PolygonComponent, &AABB, &Transform2d, &mut RigidBody2d, Option<&EntityToRotate>, Option<&EntityToMove>)>,
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
)
{
    let (camera, camera_transform) = q_camera.single();
    let wnd = windows.get_primary().unwrap();

    if let Some(_position) = wnd.cursor_position() {
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
        let ndc = (_position / window_size) * 2.0 - Vec2::ONE;
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        let world_pos: Vec2 = world_pos.truncate();

        for (entity, _polygon, aabb, transform, rigidbody, entity_to_rotate, entity_to_move) in q_polygons.iter_mut() {
            // let is_point_inside = aabb.is_point_inside(world_pos, transform);
            let is_point_inside = _polygon.is_point_inside(transform, &world_pos);
            if buttons.just_pressed(MouseButton::Left) && entity_to_move.is_none() && is_point_inside {
                commands.entity(entity).insert(EntityToMove);
            } else if buttons.just_released(MouseButton::Left) && entity_to_move.is_some() {
                commands.entity(entity).remove::<EntityToMove>();
            }

            if buttons.just_pressed(MouseButton::Right) && entity_to_rotate.is_none() && is_point_inside {
                commands.entity(entity).insert(EntityToRotate);
            } else if buttons.just_released(MouseButton::Right) && entity_to_rotate.is_some() {
                commands.entity(entity).remove::<EntityToRotate>();
            }
        }
    }
}

fn move_polygon(
    mut q_polygons: Query<(Entity, &mut RigidBody2d, &mut Transform2d, Option<&EntityToRotate>, Option<&EntityToMove>), (Or<(With<EntityToRotate>, With<EntityToMove>)>, With<PolygonComponent>)>,
    q_camera: Query<&Projection, With<MainCamera>>,
    mut motion_evr: EventReader<MouseMotion>,
    kb_buttons: Res<Input<KeyCode>>,
)
{
    let proj = q_camera.single();
    let Projection::Orthographic(ref proj) = *proj else { unimplemented!() };

    let mut delta = {
        let mut tmp_delta = Vec2::new(0f32, 0f32);
        for ev in motion_evr.iter() {
            tmp_delta += ev.delta;
        }
        tmp_delta.y *= -1f32;
        tmp_delta *= proj.scale;
        tmp_delta
    };

    if kb_buttons.pressed(KeyCode::LShift)
    {
        if delta.x.abs() > delta.y.abs() {
            delta.y = 0f32
        } else {
            delta.x = 0f32;
        }
    }

    for (_entity, mut rigibody, mut transform, entity_to_rotate, entity_to_move) in q_polygons.iter_mut()
    {
        if entity_to_move.is_some()
        {
            if rigibody.is_kinematic {
                transform.translation += delta;
            } else {
                rigibody.linear_speed += delta;
            }
        }
        if entity_to_rotate.is_some()
        {
            if rigibody.is_kinematic
            {
                transform.rotation += delta.x.to_radians() % (PI * 2.0);
            } else {
                rigibody.angular_speed += delta.x.to_radians() % (PI * 2.0);
            }
        }
    }
}

fn auto_move_polygon(
    mut q_polygons: Query<&mut Transform2d, (Without<EntityToMove>, With<PolygonComponent>)>,
    kb_buttons: Res<Input<KeyCode>>,
)
{
    if kb_buttons.pressed(KeyCode::E) {
        q_polygons.par_for_each_mut(20, |mut transform| {
            transform.translation += Vec2::new(
                fastrand::f32() - 0.5f32,
                fastrand::f32() - 0.5f32,
            );
        })
    }
}

impl Plugin for DrawPolygonPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_stage_before(CollisionStage::SyncData, "Salut", SystemStage::parallel())
            .add_system_to_stage("Salut", select_polygons)
            .add_system_to_stage("Salut", move_polygon)
            .add_system_to_stage("Salut", auto_move_polygon);
    }
}