use std::f32::consts::FRAC_PI_2;

use ambient_api::{
    components::core::{
        game_objects::player_camera,
        physics::{
            character_controller_height, character_controller_radius, plane_collider, visualizing,
        },
        player::{player, user_id},
        prefab::prefab_from_url,
        primitives::{cube, quad},
        rendering::color,
        transform::{lookat_center, rotation, scale, translation, local_to_parent},
        ecs::{children, parent}
    },
    concepts::{make_perspective_infinite_reverse_camera, make_transformable},
    entity::{AnimationAction, AnimationController},
    player::KeyCode,
    prelude::*,
};
use components::{player_camera_ref, player_mesh_ref};

#[main]
pub async fn main() -> EventResult {
    // ground
    Entity::new()
        .with_merge(make_transformable())
        .with_default(quad())
        .with(scale(), Vec3::ONE * 10.)
        .with(color(), vec4(0.5, 1.0, 0.5, 1.))
        .with_default(plane_collider())
        .spawn();

    spawn_query((player(), user_id())).bind(move |players| {
        for (id, (_, user)) in players {
            let camera = Entity::new()
                .with_merge(make_perspective_infinite_reverse_camera())
                .with_default(player_camera())
                .with(user_id(), user)
                .with(translation(), Vec3::ONE)
                .with(rotation(), Quat::IDENTITY)
                .with(lookat_center(), vec3(0., 0., 0.))
                .spawn();

            // add mecha to player id
            let player_mesh_id = Entity::new()
                .with_merge(make_transformable())
                .with(prefab_from_url(), asset_url("assets/mecha.glb").unwrap())
                .with_default(local_to_parent())
                .with(parent(), id)
                .with(rotation(), Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2)) // rotate blender mesh to fit world coordinates
                .spawn();

            entity::add_components(
                id,
                // root entity and character controller
                Entity::new()
                    .with_merge(make_transformable())
                    .with_default(cube())
                    .with(children(), vec![player_mesh_id])
                    .with(player_camera_ref(), camera)
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

                let forward = entity::get_component(player_id, rotation()).unwrap() * Vec3::X;
                let right = entity::get_component(player_id, rotation()).unwrap() * Vec3::Y;
                let speed = 0.1;

                let mut player_direction: Vec3 = Vec3::ZERO;

                if pressed.keys.contains(&KeyCode::W) {
                    player_direction += forward;
                }
                if pressed.keys.contains(&KeyCode::S) {
                    player_direction += -forward;
                }
                if pressed.keys.contains(&KeyCode::A) {
                    player_direction += -right;
                }
                if pressed.keys.contains(&KeyCode::D) {
                    player_direction += right;
                }

                //update player rotation
                entity::mutate_component(player_id, rotation(), |q: &mut Quat| {
                    *q *= Quat::from_rotation_z(delta.mouse_position.x * 0.01)
                });

                // upate player translation
                entity::mutate_component(player_id, translation(), |t| *t += player_direction * speed);


                let player_position = entity::get_component(player_id, translation()).unwrap();
                let camera_player_distance: Vec3 = Vec3::NEG_ONE * forward * 5.;
                let camera_height_offset: Vec3 = Vec3::Z * 9.5;
                let camera_position: Vec3 = player_position + camera_player_distance + camera_height_offset;
                entity::set_component(camera_id, translation(), camera_position);
//                entity::set_component(camera_id, lookat_center(), player_position);

                // update camera vertical rotation
                entity::mutate_component(camera_id, rotation(), |q: &mut Quat|{
                    *q *= Quat::from_rotation_z(delta.mouse_position.x * 0.01);
                    *q *= Quat::from_rotation_y(delta.mouse_position.y * 0.01);
                });

                let camera_rotation = entity::get_component(camera_id, rotation()).unwrap();
                let camera_target_height_offset: Vec3 = Vec3::Z * 7.5;
                // WTF is this correct? tried to copy camera 'front' from the typescript codebase
                let camera_forward = camera_rotation.mul_vec3(Vec3::X);
                let camera_target_distance: f32 = 150.;
                let camera_target = player_position + camera_target_height_offset + ( camera_forward * camera_target_distance);

                entity::set_component(camera_id, lookat_center(), camera_target);


               // Animation controllers
               if player_direction.length() != 0.0 { // play walk
                    entity::set_animation_controller(
                        player_mesh_id,
                        AnimationController {
                            actions: &[AnimationAction {
                                clip_url: &asset_url("assets/mecha.glb/animations/walk_2.anim").unwrap(),
                                looping: true,
                                weight: 1.,
                            }],
                            apply_base_pose: false,
                        },
                    );
                }else{ // idle
                    entity::set_animation_controller(
                        player_mesh_id,
                        AnimationController {
                            actions: &[AnimationAction {
                                clip_url: &asset_url("assets/mecha.glb/animations/idle_1.anim").unwrap(),
                                looping: true,
                                weight: 1.,
                            }],
                            apply_base_pose: false,
                        },
                    );
                }

            }
        });

    EventOk
}
