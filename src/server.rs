use ambient_api::{
    components::core::{
        physics::{
            character_controller_height, character_controller_radius,
            plane_collider, visualizing,
        },
        player::player,
        primitives::{cube, quad},
        rendering::color,
        transform::{rotation, scale, translation, local_to_parent},
        prefab::prefab_from_url,
        ecs::{children, parent}
    },
    concepts::make_transformable,
    prelude::*,
};

use components::{
    view_vertical_rotation,
    player_mesh_ref,
    player_input_direction,
    player_mouse_delta_x,
    player_mouse_delta_y
};

#[main]
pub fn main() {
    let world_front: Vec3 = Vec3::X;
    let world_right: Vec3 = Vec3::Y;

    // ground entity
    Entity::new()
        .with_merge(make_transformable())
        .with_default(quad())
        .with(scale(), Vec3::ONE * 10.)
        .with(color(), vec4(0.5, 1.0, 0.5, 1.))
        .with_default(plane_collider())
        .spawn();

    spawn_query(player()).bind(move |players| {
        for (id, _) in players {
            // add mecha to player id
            let player_mesh_id = Entity::new()
                .with_merge(make_transformable())
                .with(
                    prefab_from_url(),
                    asset::url("assets/mecha.glb").unwrap(),
                )
                .with_default(local_to_parent())
                .with(parent(), id)
                .with(rotation(), Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2)) // rotate blender mesh to fit world coordinates
                .spawn();

            // create root player entity
            entity::add_components(
                id,
                // root entity and character controller
                Entity::new()
                    .with_merge(make_transformable())
                    .with_default(cube())
                    .with(children(), vec![player_mesh_id])
                    .with(view_vertical_rotation(), Quat::IDENTITY)
                    .with(player_mesh_ref(), player_mesh_id)
                    .with(color(), vec4(0.5, 0.0, 1.0, 1.0))
                    .with(character_controller_height(), 2.)
                    .with(character_controller_radius(), 0.5)
                    .with_default(visualizing()),
            );
        }
    });

    // capture input messages from client and update state
    messages::Input::subscribe(move |source, msg| {
        let Some(player_id) = source.client_entity_id() else { return; };

        entity::add_component(player_id, player_input_direction(), msg.input_direction);
        entity::add_component(player_id, player_mouse_delta_x(), msg.mouse_delta_x);
        entity::add_component(player_id, player_mouse_delta_y(), msg.mouse_delta_y);
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
            entity::mutate_component(player_id, view_vertical_rotation(), |q: &mut Quat| {
                *q *= Quat::from_rotation_y(mouse_delta_y * 0.01);
            });

            let player_rotation = entity::get_component(player_id, rotation()).unwrap();
            let player_forward = player_rotation * world_front;
            let player_right = player_rotation * world_right;
            let speed = 0.1;

            let mut player_direction: Vec3 = Vec3::ZERO;

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

            //TODO how to update physics here?
            //physics::move_character(player_id, displace, 0.01, frametime());
        }
    });
}
