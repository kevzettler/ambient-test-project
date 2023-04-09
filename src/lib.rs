use std::f32::consts::FRAC_PI_2;

use ambient_api::{
    components::core::{
        game_objects::player_camera,
        physics::{
            character_controller_height, character_controller_radius, plane_collider, visualizing
        },
        player::{player, user_id},
        prefab::prefab_from_url,
        primitives::{cube, quad },
        rendering::{color, transparency_group},
        transform::{lookat_center, rotation, scale, translation, local_to_parent},
        ecs::{children, parent}
    },
    concepts::{make_perspective_infinite_reverse_camera, make_transformable, make_sphere},
    player::KeyCode,
    prelude::*,
};
use components::{player_camera_ref, player_mesh_ref, player_target_rotation};


#[main]
pub async fn main() -> EventResult {
    let world_front: Vec3 = Vec3::X;
    let world_right: Vec3 = Vec3::Y;

    let view_sphere_offset = Vec3::Z * 7. + Vec3::Y * 2.;
    let target_offset:Vec3 = view_sphere_offset + world_front * 7.;
    let eye_offset:Vec3 = view_sphere_offset - world_front * 7.;

    // camera target mesh for debugging
    let target_mesh_id = Entity::new()
        .with_merge(make_sphere())
        .with_default(translation())
        .with(transparency_group(), 100)
        .with(color(), vec4(0., 1.0, 1.0, 1.))
        .spawn();

    // camera target mesh for debugging
    let view_center_mesh_id = Entity::new()
        .with_merge(make_sphere())
        .with_default(translation())
        .with(transparency_group(), -10)
        .with(color(), vec4(0.4, 1.0, 0.0, 0.5))
        .spawn();


    // ground entity
    Entity::new()
        .with_merge(make_transformable())
        .with_default(quad())
        .with(scale(), Vec3::ONE * 10.)
        .with(color(), vec4(0.5, 1.0, 0.5, 1.))
        .with_default(plane_collider())
        .spawn();

    spawn_query((player(), user_id())).bind(move |players| {
        for (id, (_, user)) in players {
            // add mecha to player id
            let player_mesh_id = Entity::new()
                .with_merge(make_transformable())
                .with(prefab_from_url(), asset_url("assets/mecha.glb").unwrap())
                .with_default(local_to_parent())
                .with(parent(), id)
                .with(rotation(), Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2)) // rotate blender mesh to fit world coordinates
                .spawn();

            let camera = Entity::new()
                .with_merge(make_perspective_infinite_reverse_camera())
                .with_default(player_camera())
                .with(user_id(), user)
                .with(translation(), eye_offset)
                .with(lookat_center(), target_offset)
                .spawn();

            // create root player entity
            entity::add_components(
                id,
                // root entity and character controller
                Entity::new()
                    .with_merge(make_transformable())
                    .with_default(cube())
                    .with(children(), vec![player_mesh_id])
                    .with(player_camera_ref(), camera)
                    .with(player_target_rotation(), Quat::IDENTITY)
                    .with(player_mesh_ref(), player_mesh_id)
                    .with(color(), vec4(0.5, 0.0, 1.0, 1.0))
                    .with(character_controller_height(), 2.)
                    .with(character_controller_radius(), 0.5)
                    .with_default(visualizing()),
            );
        }
    });

    query((player(), player_camera_ref(), player_mesh_ref()))
        .build()
        .each_frame(move |players| {
            for (player_id, (_, camera_id, player_mesh_id)) in players {
                let Some((delta, pressed)) = player::get_raw_input_delta(player_id) else { continue; };

                //update player rotation
                entity::mutate_component(player_id, rotation(), |q: &mut Quat| {
                    *q *= Quat::from_rotation_z(delta.mouse_position.x * 0.01);
                });

                // update camera rotation on vertical access
                entity::mutate_component(player_id, player_target_rotation(), |q: &mut Quat| {
                   *q *= Quat::from_rotation_y(delta.mouse_position.y * 0.01);
                });

                let player_rotation = entity::get_component(player_id, rotation()).unwrap();
                let player_forward = player_rotation * world_front;
                let player_right = player_rotation * world_right;
                let speed = 0.1;

                let mut player_direction: Vec3 = Vec3::ZERO;

                if pressed.keys.contains(&KeyCode::W) {
                    player_direction += player_forward;
                }
                if pressed.keys.contains(&KeyCode::S) {
                    player_direction += -player_forward;
                }
                if pressed.keys.contains(&KeyCode::A) {
                    player_direction += -player_right;
                }
                if pressed.keys.contains(&KeyCode::D) {
                    player_direction += player_right;
                }

                // update player translation
                entity::mutate_component(player_id, translation(), |t| *t += player_direction * speed);


                // this is like an arc ball camera. There is a point offset from the players position that is the center of a sphere
                // There is eye on one point of the sphere and a target on the other
                // There is a quaternion that is updated with horizontal and vertical rotations from mouse input
                // there is a vector that is calculated from the world front vector and the rotation quaternion
                //
                let player_rotation = entity::get_component(player_id, rotation()).unwrap();
                let view_vert_rotation = entity::get_component(player_id, player_target_rotation()).unwrap();
                let camera_rotation_quat = player_rotation * view_vert_rotation;

                let player_position = entity::get_component(player_id, translation()).unwrap();

                let camera_front = camera_rotation_quat * Vec3::X;
                let target_projection = camera_front * 30.;
                let view_sphere_offset = Vec3::Z * 7. + player_right * 2.;
                let target_position = player_position + view_sphere_offset + target_projection;


                // move the target debug mesh to new target location
                entity::set_component(target_mesh_id, translation(), target_position);
                entity::set_component(view_center_mesh_id, translation(), player_position+view_sphere_offset);

                // update camera lookat
                entity::set_component(camera_id, lookat_center(), target_position);

                let camera_projection = camera_front * Vec3::NEG_ONE * 10.;
                let eye_position: Vec3 = player_position + view_sphere_offset + camera_projection;
                entity::set_component(camera_id, translation(), eye_position);
            }
        });

    EventOk
}
