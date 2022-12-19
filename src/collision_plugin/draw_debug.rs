use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;

use crate::aabb::AABB;
use crate::collision_plugin::broad_phase::BroadPhaseData;
use crate::collision_plugin::config::CollisionConfig;
use crate::collision_plugin::narrow_phase::{NarrowPhaseData};
use crate::collision_plugin::narrow_phase::helpers::find_furthest_point;
use crate::drawable::Drawable;
use crate::polygon_component::PolygonComponent;
use crate::transform2d::Transform2d;

pub fn draw_debug(
    mut query: Query<(Entity, &mut PolygonComponent, &Transform2d, &AABB)>,
    narrow_phase_data: ResMut<NarrowPhaseData>,
    broad_phase_data: ResMut<BroadPhaseData>,
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
        for pair in &broad_phase_data.collision_pairs {
            let (_, p1, t1, a1) = query.get(pair.entity_a).unwrap();
            let (_, p2, t2, a2) = query.get(pair.entity_b).unwrap();
            let vec = Vec2::ZERO;
            lines.line_colored(t1.position.extend(0f32), t2.position.extend(0f32), 0f32, Color::YELLOW);
        }
    }

    if config.draw_debug_narrow_phase {
        for collision_info in &narrow_phase_data.collision_infos {
            if let Some(pair) = collision_info.collision_pair
            {
                {
                    let (_, mut p1, t1, a1) = query.get_mut(pair.entity_a).unwrap();
                    p1.color = Color::RED;
                }

                {
                    let (_, mut p2, t2, a2) = query.get_mut(pair.entity_b).unwrap();
                    p2.color = Color::RED;
                }

                let start_location = collision_info.location.extend(0f32);
                let end_location = start_location - collision_info.normal.extend(0f32) * collision_info.distance;

                lines.line_colored(start_location, end_location, 0f32, Color::PURPLE);
            }
        }
    }
}