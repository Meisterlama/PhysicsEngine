use bevy::prelude::*;

use crate::{
    collision_plugin::{
        aabb::AABB,
        config::{CollisionConfig},
        PhysicsAwake,
        polygon_component::PolygonComponent,
        rigidbody::RigidBody2d,
    }
};
use crate::collision_plugin::data_structs::{BroadPhaseData, NarrowPhaseData};
use crate::collision_plugin::TIMESTEP;
use crate::transform2d::Transform2d;

pub(crate) fn startup_refresh_entities(
    mut broad_phase_data: ResMut<BroadPhaseData>,
    entities: Query<Entity, With<PhysicsAwake>>,
)
{
    broad_phase_data.sorted_entities.append(&mut entities.iter().collect::<Vec<_>>());
}

pub(crate) fn refresh_entities(
    mut broad_phase_data: ResMut<BroadPhaseData>,
    narrow_phase_data: ResMut<NarrowPhaseData>,
    config: Res<CollisionConfig>,
    entity_added: Query<Entity, Added<PhysicsAwake>>,
    entity_removed: RemovedComponents<PhysicsAwake>,
    entity_changed: Query<Entity, Changed<Transform2d>>,
    query: Query<(&Transform2d, &AABB), With<PolygonComponent>>,
    mut commands: Commands,
)
{
    let _span = info_span!("broad_phase", name = "refresh_entities").entered();

    // Filter existing entities
    let entity_removed = entity_removed.iter().collect::<Vec<_>>();
    broad_phase_data.sorted_entities.retain(|x| !entity_removed.contains(&x));
    broad_phase_data.sorted_entities.retain(|&x| query.get(x).is_ok());

    // FIXME: Waking up isn't done properly, nearby object should be woken up too
    // Handle asleep/awake of entities
    // if config.system_params.put_non_colliding_asleep
    // {
    //     for entity in &broad_phase_data.sorted_entities {
    //         if !narrow_phase_data.collided_entities.contains(&entity)
    //         {
    //             commands.entity(*entity).remove::<PhysicsAwake>();
    //         }
    //     }
    //     broad_phase_data.sorted_entities.retain(|x| narrow_phase_data.collided_entities.contains(&x));
    // }

    let mut new_entities = entity_added.iter().collect::<Vec<_>>();
    broad_phase_data.sorted_entities.append(&mut new_entities);

    if !entity_added.is_empty() || !entity_removed.is_empty() || !entity_changed.is_empty()
    {
        broad_phase_data.sorted_entities.sort_by(|&e1, &e2| {
            let (t1, a1) = query.get(e1).unwrap();
            let (t2, a2) = query.get(e2).unwrap();
            let a1_min_x = t1.translate(a1.min).x;
            let a2_min_x = t2.translate(a2.min).x;

            let cmp = (a2_min_x).total_cmp(&a1_min_x);

            if cmp.is_eq() {
                return e1.cmp(&e2);
            }
            return cmp;
        });
        broad_phase_data.sorted_entities.dedup();
        // warn!("{:?}", broad_phase_data.sorted_entities);
    }
}

pub(crate) fn aabb_update_system(mut query: Query<(&PolygonComponent, &mut AABB, &Transform2d), Changed<Transform2d>>)
{
    for (p, mut a, t) in query.iter_mut()
    {
        *a = AABB::from(&p.get_rotated_points(&t));
    }
}

pub fn update_rigidbodies(
    mut query: Query<(&mut RigidBody2d, &mut Transform2d)>,
    config: Res<CollisionConfig>,
    time: Res<Time>,
)
{
    for (mut rigidbody, mut transform) in query.iter_mut() {
        if !rigidbody.is_kinematic {
            let mut acceleration = rigidbody.linear_acceleration;
            if config.system_params.gravity_enabled {
                acceleration += rigidbody.get_mass() * Vec2::new(0f32, -9.81f32);
            }
            rigidbody.linear_speed += acceleration * TIMESTEP as f32;
            transform.translation += rigidbody.linear_speed * TIMESTEP as f32;
            transform.rotation += rigidbody.angular_speed * TIMESTEP as f32;
        } else {
            rigidbody.linear_speed = Vec2::ZERO;
            rigidbody.linear_acceleration = Vec2::ZERO;

            rigidbody.angular_speed = 0f32;
            rigidbody.angular_acceleration = 0f32;
        }
    }
}