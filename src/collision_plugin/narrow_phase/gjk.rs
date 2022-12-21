use bevy::prelude::*;

use crate::collision_plugin::collision_structs::CollisionInfo;
use crate::collision_plugin::narrow_phase::helpers::{find_furthest_point, triple_product};
use crate::polygon_component::PolygonComponent;
use crate::transform2d::Transform2d;

pub fn get_support(p1: &PolygonComponent, t1: &Transform2d,
               p2: &PolygonComponent, t2: &Transform2d,
               direction: Vec2,
) -> Vec2 {
    return find_furthest_point(p1, t1, direction)
        - find_furthest_point(p2, t2, -direction);
}

pub fn check_collision(p1: &PolygonComponent, t1: &Transform2d,
                       p2: &PolygonComponent, t2: &Transform2d) -> (bool, [Vec2; 3])
{

    let mut simplex = [Vec2::ZERO; 3];
    let mut index = 0;

    let mut a = get_support(p1, t1, p2, t2, Vec2::X);
    simplex[0] = a;

    let mut direction = -a;

    for _ in 0..32 {
        a = get_support(p1, t1, p2, t2, direction);

        index += 1;
        simplex[index] = a;

        if a.dot(direction) <= 0f32 {
            break;
        }

        let ao = -a;

        // simplex is line
        if index < 2 {
            let b = simplex[0];

            let ab = b - a;
            direction = triple_product(&ab, &ao, &ab);

            if direction.length_squared() == 0f32 {
                direction = ab.perp();
            }
            continue;
        }

        let b = simplex[1];
        let c = simplex[0];

        let ab = b - a;
        let ac = c - a;

        let acp = triple_product(&ab, &ac, &ac);

        if acp.dot(ao) >= 0f32 {
            direction = acp;
        } else {
            let abp = triple_product(&ac, &ab, &ab);
            if abp.dot(ao) < 0f32
            {
                return (true, simplex.clone());
            }

            simplex[0] = simplex[1];

            direction = abp;
        }

        simplex[1] = simplex[2];
        index -= 1;
    }

    return (false, [Vec2::ZERO; 3]);
}

pub fn get_info_collisions(p1: &PolygonComponent, t1: &Transform2d,
                           p2: &PolygonComponent, t2: &Transform2d,
                           simplex: [Vec2; 3],
) -> CollisionInfo {
    const MAX_POLYTOPE_VERTICES: usize = 256;
    const TOLERANCE: f32 = 0.001f32;

    let mut polytope = Vec::<Vec2>::from(simplex);

    let mut min_index = 0;
    let mut min_dist = f32::INFINITY;
    let mut min_normal = Vec2::ZERO;

    while min_dist >= f32::INFINITY {
        for i in 0..polytope.len() {
            let j = (i + 1) % polytope.len();

            let vtx_i = polytope[i].clone();
            let vtx_j = polytope[j].clone();

            let vtx_edge = vtx_j - vtx_i;

            let mut normal = Vec2::new(vtx_edge.y, -vtx_edge.x).normalize();
            let mut dist = normal.dot(vtx_i);

            if dist < 0f32 {
                dist *= -1f32;
                normal *= -1f32;
            }
            if dist < min_dist {
                min_dist = dist;
                min_normal = normal;
                min_index = j;
            }
        }

        let support = get_support(p1, t1, p2, t2, min_normal);
        let dist = min_normal.dot(support);
        if polytope.len() > MAX_POLYTOPE_VERTICES {
            break;
        }

        if (dist - min_dist).abs() > TOLERANCE {
            min_dist = f32::INFINITY;
            polytope.insert(min_index, support);
        }
    }

    let location1 = find_furthest_point(p1, t1, min_normal);
    let location2 = find_furthest_point(p2, t2, -min_normal);

    let mut location = location1;
    if !p2.is_point_inside(t2, &location)
    {
        location = location2;
        // min_normal *= -1f32;
    }

    return CollisionInfo {
        collision_pair: None,
        location: location,
        normal: min_normal,
        distance: min_dist + TOLERANCE * 2f32,
    };
}