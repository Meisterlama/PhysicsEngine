use bevy::prelude::*;
use crate::line_renderer::{LineBatch, LineBatches};

use crate::polygon_component::PolygonComponent;
use crate::transform2d::Transform2d;

pub struct PolygonRendererPlugin;

fn refresh_polygon_mesh(
    mut line_batches: ResMut<LineBatches>,
    polygon_query: Query<(&PolygonComponent, &Transform2d)>,
)
{
    let mut batches = Vec::<LineBatch>::new();


    let mut non_colliding_batch = LineBatch::new(Color::GREEN);
    let mut colliding_batch = LineBatch::new(Color::RED);

    for (p, t) in &polygon_query {
        let mut points = p.get_transformed_points(t);

        points.push(points[0].clone());

        let points = points.iter().map(|x| x.extend(0f32)).collect::<Vec<Vec3>>();

        if !p.collided {
            if !non_colliding_batch.try_push_vertices(&points)
            {
                batches.push(non_colliding_batch);
                non_colliding_batch = LineBatch::new(Color::GREEN);
                non_colliding_batch.try_push_vertices(&points);
            }
        } else {
            if !colliding_batch.try_push_vertices(&points)
            {
                batches.push(colliding_batch);
                colliding_batch = LineBatch::new(Color::RED);
                colliding_batch.try_push_vertices(&points);
            }
        }
    }

    if !colliding_batch.is_empty() {
        batches.push(colliding_batch);
    }
    if !non_colliding_batch.is_empty() {
        batches.push(non_colliding_batch);
    }

    line_batches.batches.append(&mut batches);
}

impl Plugin for PolygonRendererPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LineBatches>();
        app.add_system(refresh_polygon_mesh);
    }
}