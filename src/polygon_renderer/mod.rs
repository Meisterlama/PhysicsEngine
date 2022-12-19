use bevy::prelude::*;
use bevy::render::mesh::PrimitiveTopology;
use bevy::sprite::MaterialMesh2dBundle;

use crate::polygon_component::PolygonComponent;
use crate::transform2d::Transform2d;

pub struct PolygonRendererPlugin;

fn refresh_polygon_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<(Entity, &PolygonComponent, &Transform2d), Changed<PolygonComponent>>
)
{
    for (entity, poly, t2d) in query.iter()
    {
        let mut poly_mesh = Mesh::new(PrimitiveTopology::LineList);

        let mut poly_3d_triangles : Vec<Vec3> = vec!{};
        for pair in poly.points.windows(2) {
            poly_3d_triangles.extend_from_slice(&[pair[0].extend(0f32), pair[1].extend(0f32)])
        }
        poly_3d_triangles.extend_from_slice(&[poly.points.last().unwrap().extend(0f32), poly.points[0].extend(0f32)]);


        poly_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, poly_3d_triangles);

        commands.entity(entity).insert(MaterialMesh2dBundle {
            mesh: meshes.add(poly_mesh).into(),
            transform: Transform::default().with_translation(t2d.position.extend(0f32)).with_rotation(Quat::from_euler(EulerRot::XYZ, 0f32, 0f32, t2d.rotation)),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            ..default()
        });
    }
}

fn sync_polygon_transform(
    mut query: Query<(Entity, &Transform2d, &mut Transform)>
)
{
    for (entity, transform2d, mut transform) in query.iter_mut()
    {
        transform.translation = transform2d.position.extend(0f32);
        transform.rotation = Quat::from_euler(EulerRot::XYZ,0f32, 0f32, transform2d.rotation);
    }
}


impl Plugin for PolygonRendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(refresh_polygon_mesh);
        app.add_system(sync_polygon_transform);
    }
}