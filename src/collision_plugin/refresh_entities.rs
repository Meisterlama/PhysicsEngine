use bevy::prelude::*;

use crate::aabb::AABB;
use crate::collision_plugin::broad_phase::{BroadPhaseData, BroadPhaseQuery};
use crate::collision_plugin::config::CollisionConfig;
use crate::collision_plugin::narrow_phase::NarrowPhaseData;
use crate::collision_plugin::PhysicsAwake;
use crate::transform2d::Transform2d;

pub fn refresh_entities(
    mut broad_phase_data: ResMut<BroadPhaseData>,
    mut narrow_phase_data: ResMut<NarrowPhaseData>,
    mut config: Res<CollisionConfig>,
    entity_added: Query<(Entity), (Added<AABB>, With<Transform2d>)>,
    entity_changed: Query<(Entity, Option<&PhysicsAwake>), (Or<(Changed<AABB>, Changed<Transform2d>)>)>,
    entity_removed: RemovedComponents<AABB>,
    query: BroadPhaseQuery,
    mut commands: Commands,
)
{
    let span = info_span!("broad_phase", name = "refresh_entities").entered();

    // Filter existing entities
    let entity_removed = entity_removed.iter().collect::<Vec<_>>();
    broad_phase_data.sorted_entities.retain(|x| !entity_removed.contains(x));
    broad_phase_data.sorted_entities.retain(|&x| query.get(x).is_ok());


    // FIXME: Waking up isn't done properly, nearby object should be woken up too
    // Handle asleep/awake of entities
    if config.put_non_colliding_asleep
    {
        for entity in &broad_phase_data.sorted_entities {
            if !narrow_phase_data.collided_entities.contains(entity)
            {
                commands.entity(*entity).remove::<PhysicsAwake>();
            }
        }
        broad_phase_data.sorted_entities.retain(|x| narrow_phase_data.collided_entities.contains(x));
    }

    // Wake up changed entities
    for (entity, awake) in &entity_changed {
        if awake.is_none()
        {
            commands.entity(entity).insert(PhysicsAwake);
            broad_phase_data.sorted_entities.push(entity);
        }
    }

    if !entity_changed.is_empty() || !entity_added.is_empty() || !entity_removed.is_empty()
    {
        broad_phase_data.sorted_entities.sort_by(|&e1, &e2| {
            let (_, p1, t1, a1) = query.get(e1).unwrap();
            let (_, p2, t2, a2) = query.get(e2).unwrap();
            return (t1.translate(&a1.min).x).partial_cmp(&(t2.translate(&a2.min).x)).unwrap();
        });
    }
}