use ambient_api::{
    components::core::{
        game_objects::player_camera,
        physics::{
            character_controller_height, character_controller_radius, plane_collider, visualizing,
        },
        player::{player, user_id},
        prefab::prefab_from_url,
        primitives::quad,
        rendering::color,
        transform::{lookat_center, rotation, scale, translation},
    },
    concepts::{make_perspective_infinite_reverse_camera, make_transformable},
    entity::{AnimationAction, AnimationController},
    player::KeyCode,
    prelude::*,
};
use components::player_camera_ref;

#[main]
pub async fn main() -> EventResult {
    // ground
    Entity::new()
        .with_merge(make_transformable())
        .with_default(quad())
        .with(scale(), Vec3::ONE * 10.)
        .with(color(), vec4(0.5, 0.5, 0.5, 1.))
        .with_default(plane_collider())
        .spawn();

    spawn_query((player(), user_id())).bind(move |players| {
        for (id, (_, user)) in players {
            let camera = Entity::new()
                .with_merge(make_perspective_infinite_reverse_camera())
                .with_default(player_camera())
                .with(user_id(), user)
                .with(translation(), Vec3::ONE)
                .with(lookat_center(), vec3(0., 0., 0.))
                .spawn();

            // add mecha to player id
            entity::add_components(
                id,
                Entity::new()
                    .with_merge(make_transformable())
                    .with_default(visualizing())
                    .with(prefab_from_url(), asset_url("assets/mecha.glb").unwrap())
                    .with(player_camera_ref(), camera)
                    .with(character_controller_height(), 2.)
                    .with(character_controller_radius(), 0.5),
            );
        }
    });

    query((player(), player_camera_ref()))
        .build()
        .each_frame(move |players| {
            for (player_id, (_, camera_id)) in players {
                let Some((delta, pressed)) = player::get_raw_input_delta(player_id) else { continue; };

                let forward = entity::get_component(player_id, rotation()).unwrap() * Vec3::X;
                let right = entity::get_component(player_id, rotation()).unwrap() * Vec3::Y;
                let speed = 0.1;

                let mut direction: Vec3 = Vec3::ZERO;

                if pressed.keys.contains(&KeyCode::W) {
                    direction += forward;
                }
                if pressed.keys.contains(&KeyCode::S) {
                    direction += -forward;
                }
                if pressed.keys.contains(&KeyCode::A) {
                    direction += -right;
                }
                if pressed.keys.contains(&KeyCode::D) {
                    direction += right;
                }

                println!("wtf is player direction {}", &direction.length());
                entity::mutate_component(player_id, rotation(), |q: &mut Quat| {
                    *q *= Quat::from_rotation_z(delta.mouse_position.x * 0.01)
                });


                entity::mutate_component(player_id, translation(), |t| *t += direction * speed);

               if direction.length() != 0.0 { // play walk
                    entity::set_animation_controller(
                        player_id,
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
                        player_id,
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


                let pos = entity::get_component(player_id, translation()).unwrap();
                entity::set_component(camera_id, lookat_center(), pos);
                entity::set_component(camera_id, translation(), pos - forward * 4. + Vec3::Z * 7.);
            }
        });

    EventOk
}
