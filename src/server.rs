use core::f32::consts::FRAC_PI_2;

use ambient_api::{
    components::core::{
        ecs::{children, parent},
        physics::{character_controller_height, character_controller_radius, plane_collider},
        player::player,
        prefab::prefab_from_url,
        primitives::{cube, quad},
        rendering::color,
        text::{font_size, text},
        transform::{local_to_parent, rotation, scale, spherical_billboard, translation},
    },
    concepts::make_transformable,
    prelude::*,
};

use components::{
    player_animation_controller_ref, player_input_direction, player_mesh_ref, player_mouse_delta_x,
    player_mouse_delta_y, player_text_container_ref, player_vertical_rotation_angle,
    view_vertical_rotation,
};

mod player_animation_controller;
use player_animation_controller::{PlayerAnimationController, PlayerAnimationEvent};

fn make_text() -> Entity {
    Entity::new()
        .with(
            local_to_parent(),
            Mat4::from_scale(Vec3::ONE * 0.02) * Mat4::from_rotation_x(-180_f32.to_radians()),
        )
        .with(color(), vec4(1., 0., 0., 1.))
        .with(font_size(), 36.)
        .with_default(main_scene())
        .with_default(local_to_world())
        .with_default(mesh_to_local())
        .with_default(mesh_to_world())
}

#[main]
pub fn main() {
    let world_front: Vec3 = Vec3::X;
    let world_right: Vec3 = Vec3::Y;

    // ground entity
    Entity::new()
        .with_merge(make_transformable())
        .with_default(quad())
        .with(scale(), Vec3::ONE * 20.)
        .with(color(), vec4(0.5, 1.0, 0.5, 1.0))
        .with_default(plane_collider())
        .spawn();

    spawn_query(player()).bind(move |players| {
        for (id, _) in players {
            // add mecha to player id
            let player_mesh_id = Entity::new()
                .with_merge(make_transformable())
                .with(prefab_from_url(), asset::url("assets/mecha.glb").unwrap())
                .with_default(local_to_parent())
                .with(parent(), id)
                .with(
                    rotation(),
                    Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2),
                ) // rotate blender mesh to fit world coordinates
                .with(
                    player_animation_controller_ref(),
                    PlayerAnimationController::new().0,
                )
                .spawn();

            let text = make_text()
                .with(color(), vec4(1.0, 1.0, 1.0, 1.0))
                .with(user_id(), id.to_string())
                .with(text(), "player".to_string())
                .with(parent(), id)
                .spawn();

            let text_container = make_transformable()
                .with_default(main_scene())
                .with_default(local_to_world())
                .with_default(spherical_billboard())
                .with(translation(), vec3(-5., 0., 5.))
                .with(children(), vec![text])
                .spawn();
            entity::add_component(id, player_text_container_ref(), text_container);

            // create root player entity
            entity::add_components(
                id,
                // root entity and character controller
                Entity::new()
                    .with_merge(make_transformable())
                    .with_default(cube())
                    .with(children(), vec![player_mesh_id, text_container])
                    .with(view_vertical_rotation(), Quat::IDENTITY)
                    .with(player_vertical_rotation_angle(), 0.0)
                    .with(player_mesh_ref(), player_mesh_id)
                    .with(color(), vec4(0.5, 0.0, 1.0, 1.0))
                    .with(character_controller_height(), 2.)
                    .with(character_controller_radius(), 0.5)
                    .with_default(player_input_direction())
                    .with_default(player_mouse_delta_x())
                    .with_default(player_mouse_delta_y()),
            );
        }
    });

    // capture input messages from client and update state
    messages::Input::subscribe(move |source, msg| {
        let Some(player_id) = source.client_entity_id() else { return; };
        entity::set_component(player_id, player_input_direction(), msg.input_direction);
        entity::set_component(player_id, player_mouse_delta_x(), msg.mouse_delta_x);
        entity::set_component(player_id, player_mouse_delta_y(), msg.mouse_delta_y);
    });

    query((
        player(),
        player_input_direction(),
        player_mouse_delta_x(),
        player_mouse_delta_y(),
    ))
    .each_frame(move |players| {
        for (player_id, (_, input_direction, mouse_delta_x, mouse_delta_y)) in players {
            // apply input messages and update player rotation and position

            //update player rotation
            entity::mutate_component(player_id, rotation(), |q: &mut Quat| {
                *q *= Quat::from_rotation_z(mouse_delta_x * 0.01);
            });

            // update camera rotation on vertical access
            // entity::mutate_component(player_id, view_vertical_rotation(), |q: &mut Quat| {
            //     *q *= Quat::from_rotation_y(mouse_delta_y * 0.01);
            // });

            entity::mutate_component(
                player_id,
                player_vertical_rotation_angle(),
                |angle: &mut f32| {
                    *angle += mouse_delta_y * 0.01;
                },
            );

            // Clamp the vertical rotation angle between a min and max value
            let min_angle = -FRAC_PI_2 + 0.1; // Adjust the 0.1 value to avoid gimbal lock
            let max_angle = FRAC_PI_2 - 0.1; // Adjust the 0.1 value to avoid gimbal lock
            let clamped_angle = entity::get_component(player_id, player_vertical_rotation_angle())
                .unwrap()
                .clamp(min_angle, max_angle);

            // Update the player_lookat_rotation quaternion
            let clamped_vertical_rotation = Quat::from_rotation_y(clamped_angle);
            entity::set_component(
                player_id,
                view_vertical_rotation(),
                clamped_vertical_rotation,
            );

            let player_rotation = entity::get_component(player_id, rotation()).unwrap();
            let player_forward = player_rotation * world_front;
            let player_right = player_rotation * world_right;
            let speed = 0.1;
            let mut player_direction: Vec3 = Vec3::ZERO;
            let player_mesh_id = entity::get_component(player_id, player_mesh_ref()).unwrap();

            let animation_controller_id =
                entity::get_component(player_mesh_id, player_animation_controller_ref()).unwrap();
            let mut animation_controller = PlayerAnimationController(animation_controller_id);

            if input_direction.x == 0.0 && input_direction.y == 0.0 {
                animation_controller.transition(player_mesh_id, PlayerAnimationEvent::Stop);
            } else {
                animation_controller.transition(player_mesh_id, PlayerAnimationEvent::Walk);
            }

            if input_direction.x == 1.0 {
                player_direction += player_forward;
            }
            if input_direction.x == -1.0 {
                player_direction += -player_forward;
            }
            if input_direction.y == -1.0 {
                player_direction += -player_right;
            }
            if input_direction.y == 1.0 {
                player_direction += player_right;
            }

            // update player translation
            entity::mutate_component(player_id, translation(), |t| *t += player_direction * speed);

            // update player text
            let player_position = entity::get_component(player_id, translation()).unwrap();
            let player_text_container =
                entity::get_component(player_id, player_text_container_ref()).unwrap();
            entity::set_component(
                player_text_container,
                translation(),
                player_position + Vec3::Z * 9.,
            );

            //TODO how to update physics here?
            //physics::move_character(player_id, displace, 0.01, frametime());
        }
    });
}
