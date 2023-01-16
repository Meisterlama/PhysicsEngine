use std::mem;

use bevy::prelude::*;
use bevy_polyline::prelude::*;

pub const POLYLINE_MAX_SIZE: usize = 4096 * 30;

#[derive(Default, Resource)]
pub struct LineBatches {
    pub batches: Vec<LineBatch>,
}

#[derive(Default, Resource)]
pub struct LineBatch {
    vertices: Vec<Vec3>,
    pub(crate) color: Color,
}

impl LineBatch {
    pub fn new(color: Color) -> Self {
        Self {
            vertices: Vec::<Vec3>::with_capacity(POLYLINE_MAX_SIZE),
            color,
        }
    }

    pub fn try_push_vertices(&mut self, vertices: &[Vec3]) -> bool {
        assert!(vertices.len() + 1 < POLYLINE_MAX_SIZE, "Polygon must be smaller than {} vertices", POLYLINE_MAX_SIZE);
        if self.vertices.len() + vertices.len() + 1 >= POLYLINE_MAX_SIZE {
            return false;
        }

        for vertex in vertices {
            self.vertices.push(*vertex);
        }
        self.vertices.push(Vec3::NAN);

        return true;
    }

    pub fn is_empty(&self) -> bool {
        return self.vertices.is_empty();
    }

    pub fn extract_vertices(&mut self) -> Vec<Vec3>
    {
        assert!(!self.is_empty());
        let mut tmp = vec!();
        mem::swap(&mut tmp, &mut self.vertices);
        return tmp;
    }
}

pub fn render_lines(
    mut line_batches: ResMut<LineBatches>,
    mut commands: Commands,
    mut polyline_materials: ResMut<Assets<PolylineMaterial>>,
    mut polylines: ResMut<Assets<Polyline>>,
    query: Query<(&Handle<Polyline>, &Handle<PolylineMaterial>)>,
) {
    for (polyline_handle, polyline_mat_handle) in query.iter()
    {
        if let Some(mut batch) = line_batches.batches.pop() {
            polylines.get_mut(polyline_handle).unwrap().vertices = batch.extract_vertices();
            polyline_materials.get_mut(polyline_mat_handle).unwrap().color = batch.color;
        }
        else {
            polylines.get_mut(polyline_handle).unwrap().vertices.clear();
        }
    }

    while let Some(mut batch) = line_batches.batches.pop() {
        warn!("New polyline! ");
        commands.spawn(PolylineBundle {
            polyline: polylines.add(Polyline {
                vertices: batch.extract_vertices(),
                ..Default::default()
            }),
            material: polyline_materials.add(PolylineMaterial {
                width: 1.0,
                color: batch.color,
                perspective: false,
                ..Default::default()
            }),
            ..Default::default()
        });
    }
}