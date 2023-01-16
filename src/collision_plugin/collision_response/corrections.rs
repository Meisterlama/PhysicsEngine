use bevy::prelude::*;

use crate::collision_plugin::collision_response::ImpulseResult;
use crate::collision_plugin::config::CollisionConfig;
use crate::collision_plugin::data_structs::CollisionInfo;
use crate::collision_plugin::helpers::Cross;
use crate::collision_plugin::polygon_component::PolygonComponent;
use crate::collision_plugin::rigidbody::RigidBody2d;
use crate::transform2d::Transform2d;

pub fn apply_position_correction(
    collision_info: &CollisionInfo,
    query: &mut Query<(&mut Transform2d, &mut RigidBody2d), With<PolygonComponent>>,
    config: &Res<CollisionConfig>,
)
{
    if let Some(collision_pair) = collision_info.collision_pair {
        let [(mut t1, rb1), (mut t2, rb2)]
            = query.get_many_mut([collision_pair.entity_a, collision_pair.entity_b]).unwrap();


        let damping = config.system_params.damping;

        let inv_mass_rb1 = rb1.get_inv_mass();
        let inv_mass_rb2 = rb2.get_inv_mass();

        let correction = (collision_info.distance * damping) / (inv_mass_rb1 + inv_mass_rb2);

        if !rb1.is_kinematic {
            t1.translation += collision_info.normal * correction * inv_mass_rb1;
        }

        if !rb2.is_kinematic {
            t2.translation -= collision_info.normal * correction * inv_mass_rb2;
        }
    }
}

pub fn compute_collision_impulse(
    collision_info: &CollisionInfo,
    query: &Query<(&mut Transform2d, &mut RigidBody2d), With<PolygonComponent>>,
    config: &Res<CollisionConfig>,
) -> Option<(ImpulseResult, ImpulseResult)>
{
    let collision_pair = &collision_info.collision_pair.unwrap();

    let [
    (t1, rb1),
    (t2, rb2)
    ] = query.get_many([collision_pair.entity_a, collision_pair.entity_b]).unwrap();

    let restitution = config.system_params.restitution;

    let inv_mass_rb1 = rb1.get_inv_mass();
    let inv_mass_rb2 = rb2.get_inv_mass();

    let mut impulse_a = ImpulseResult::default();
    impulse_a.entity = Some(collision_pair.entity_a);

    let mut impulse_b = ImpulseResult::default();
    impulse_b.entity = Some(collision_pair.entity_b);

    let mut one_impulse = false;

    for collision_location in &collision_info.location {
        let r_a = *collision_location - t1.translation;
        let r_b = *collision_location - t2.translation;

        let v_a = rb1.linear_speed + impulse_a.linear_impulse - r_a.cross_float(rb1.angular_speed + impulse_a.angular_impulse);
        let v_b = rb2.linear_speed + impulse_b.linear_impulse- r_b.cross_float(rb2.angular_speed + impulse_b.angular_impulse);

        let momentum_a = r_a.cross_vec(collision_info.normal) * inv_mass_rb1;
        let momentum_b = r_b.cross_vec(collision_info.normal) * inv_mass_rb2;

        let weight_rot_a = r_a.cross_float(momentum_a).dot(-collision_info.normal);
        let weight_rot_b = r_b.cross_float(momentum_b).dot(-collision_info.normal);

        let v_rel = (v_a - v_b).dot(collision_info.normal);

        if v_rel < 0f32 {
            one_impulse = true;
            let j = (-(1f32 + restitution) * v_rel) / (inv_mass_rb1 + inv_mass_rb2 + weight_rot_a + weight_rot_b);

            impulse_a.linear_impulse += j * inv_mass_rb1 * collision_info.normal;
            impulse_a.angular_impulse += j * momentum_a;

            impulse_b.linear_impulse -= j * inv_mass_rb2 * collision_info.normal;
            impulse_b.angular_impulse -= j * momentum_b;
        }
    }
    if one_impulse {
        return Some((impulse_a, impulse_b));
    }
    return None;
}

pub fn apply_friction(
    collision_info: &CollisionInfo,
    query: &mut Query<(&mut Transform2d, &mut RigidBody2d), With<PolygonComponent>>,
    config: &Res<CollisionConfig>,
)
{
    let normal = &collision_info.normal;
    let damping = config.system_params.damping;
    let friction = config.system_params.friction;
    let collision_pair = &collision_info.collision_pair.unwrap();
    let [(t1, mut rb1), (t2, mut rb2)] = query.get_many_mut([collision_pair.entity_a, collision_pair.entity_b]).unwrap();


    let tangent = Vec2::new(-normal.y, normal.x);
    let v_tangent = (rb1.linear_speed - rb2.linear_speed).dot(tangent);

    let inv_mass_rb1 = rb1.get_inv_mass();
    let inv_mass_rb2 = rb2.get_inv_mass();

    let collision_impulse = (collision_info.distance * damping) / (inv_mass_rb1 + inv_mass_rb2);


    let mut j = -v_tangent / (inv_mass_rb1 + inv_mass_rb2);
    //TODO: Way to update polygons friction
    // let friction = (rb1.friction + rb2.friction) / 2f32;

    j = f32::clamp(j, -collision_impulse.abs() * (friction - f32::EPSILON), collision_impulse.abs() * (friction + f32::EPSILON));

    if !rb1.is_kinematic {
        rb1.linear_speed += j * inv_mass_rb1 * tangent;
    }

    if !rb2.is_kinematic {
        rb2.linear_speed -= j * inv_mass_rb2 * tangent;
    }
}