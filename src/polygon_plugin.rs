use std::f32::consts::PI;

use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy_prototype_debug_lines::*;

use crate::aabb::AABB;
use crate::drawable::Drawable;
use crate::MainCamera;
use crate::polygon_component::PolygonComponent;
use crate::transform2d::Transform2d;

pub struct DrawPolygonPlugin;

#[derive(Resource, Default)]
pub struct DrawPoly(pub bool);

fn draw_polygons(
    draw_poly: Res<DrawPoly>,
    query: Query<(&PolygonComponent, &Transform2d)>,
    mut lines: ResMut<DebugLines>)
{
    if draw_poly.0 == true {
        for (poly, transform) in query.iter()
        {
            poly.draw(&transform, &mut lines);
        }
    }
}

#[derive(Component)]
pub struct EntityToMove;

#[derive(Component)]
pub struct EntityToRotate;

fn select_polygons(
    mut commands: Commands,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    q_polygons: Query<(Entity, &PolygonComponent, &Transform2d, &AABB, Option<&EntityToRotate>, Option<&EntityToMove>)>,
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut lines: ResMut<DebugLines>,
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

        for (entity, _polygon, transform, aabb, entity_to_rotate, entity_to_move) in q_polygons.iter() {
            // let is_point_inside = aabb.is_point_inside(transform.inv_translate(&world_pos));
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
    mut q_polygons: Query<(Entity, &mut PolygonComponent, &mut Transform2d, &AABB, Option<&EntityToRotate>, Option<&EntityToMove>), Or<(With<EntityToRotate>, With<EntityToMove>)>>,
    q_camera: Query<&OrthographicProjection, With<MainCamera>>,
    mut motion_evr: EventReader<MouseMotion>,
)
{
    let proj = q_camera.single();
    let delta = {
        let mut tmp_delta = Vec2::new(0f32, 0f32);
        for ev in motion_evr.iter() {
            tmp_delta += ev.delta;
        }
        tmp_delta.y *= -1f32;
        tmp_delta *= proj.scale;
        tmp_delta
    };

    for (_entity, polygon, mut transform, aabb, entity_to_rotate, entity_to_move) in q_polygons.iter_mut()
    {
        if entity_to_move.is_some()
        {
            transform.position += delta;
        }
        if entity_to_rotate.is_some()
        {
            transform.rotation += delta.x.to_radians() % (PI*2.0);
        }
    }
}

fn auto_move_polygon(
    mut q_polygons: Query<(Entity, &PolygonComponent, &mut Transform2d), Without<EntityToMove>>,
    kb_buttons: Res<Input<KeyCode>>,
    time: Res<Time>,
)
{
    if kb_buttons.pressed(KeyCode::E) {
        for (entity, polygon, mut transform) in q_polygons.iter_mut()
        {
            transform.position += Vec2::new(
                fastrand::f32() - 0.5f32,
                fastrand::f32() - 0.5f32,
            );
        }
    }
}

impl Plugin for DrawPolygonPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<DrawPoly>()
            .add_system(draw_polygons)
            .add_system(select_polygons)
            .add_system(move_polygon)
            .add_system(auto_move_polygon);
    }
}