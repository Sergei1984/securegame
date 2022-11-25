use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::random::rand_range;

use super::{swarm::Wasp, target::Target, GameParameters};

pub fn direct_wasps(
    time: Res<Time>,
    game_params: Res<GameParameters>,
    mut wasps_query: Query<(&mut Wasp, &Transform, &mut ExternalImpulse), With<Wasp>>,
    target_transform_query: Query<&Transform, With<Target>>,
) {
    let target_transform = target_transform_query.single();
    // let defence = defence_query.single();

    for (mut wasp, wasp_transform, mut external_impulse) in wasps_query.iter_mut() {
        if wasp.timer.tick(time.delta()).just_finished() {
            let force_point = target_transform.translation; // defence.0.local_center_of_mass;

            let impulse = Vec2::new(
                force_point.x - wasp_transform.translation.x,
                force_point.y - wasp_transform.translation.y,
            )
            .normalize();

            let rotation = Quat::from_rotation_z(rand_range(-0.2, 0.2));

            let rotated = rotation.mul_vec3([impulse.x, impulse.y, 0.0].into());

            external_impulse.impulse =
                Vec2::new(rotated.x, rotated.y) * game_params.wasp.mass * 30.0;
        }
    }
}
