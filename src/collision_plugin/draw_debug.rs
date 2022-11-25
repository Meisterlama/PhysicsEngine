use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;

use crate::aabb::AABB;
use crate::collision_plugin::config::CollisionConfig;
use crate::collision_plugin::SystemData;
use crate::drawable::Drawable;
use crate::polygon_component::PolygonComponent;
use crate::transform2d::Transform2d;

pub fn draw_debug(
    mut query: Query<(Entity, &mut PolygonComponent, &Transform2d, &AABB)>,
    system_data: ResMut<SystemData>,
    mut lines: ResMut<DebugLines>,
    config: Res<CollisionConfig>,
)
{
    query.iter_mut().for_each(|(_e, mut p, t, a)| {
        if config.draw_debug_aabb {
            a.draw(&t, &mut lines);
        }
        p.color = Color::GREEN;
    });

    if config.draw_debug_broad_phase
    {
        if let Some(collision_pairs) = &system_data.broad_phase_collision_pairs {
            for pair in collision_pairs {
                lines.line_colored(pair.pos_a.extend(0f32), pair.pos_b.extend(0f32), 0f32, Color::YELLOW)
            }
        }
    }

    if let Some(collision_infos) = &system_data.narrow_phase_collision_infos
    {
        let collision_entities: Vec<Entity> = collision_infos.iter().flat_map(|collision_info| {
            [collision_info.pair.entity_a.unwrap(),
             collision_info.pair.entity_b.unwrap()]
        }).collect();


        let mut subquery_iter = query.iter_many(&collision_entities);

        while let Ok([(_e1, p1, t1, _a1), (_e2, p2, t2, _a2)]) = subquery_iter.next_chunk::<2>()
        {
            if config.draw_debug_narrow_phase {
                lines.line_colored(t1.position.extend(0f32), t2.position.extend(0f32), 0f32, Color::PURPLE);
            }

            unsafe {
                let mut p1 = p1 as *const PolygonComponent as *mut PolygonComponent;
                let mut p2 = p2 as *const PolygonComponent as *mut PolygonComponent;
                (*p1).color = Color::RED;
                (*p2).color = Color::RED;
            }
        }
    }
}