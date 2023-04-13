use ambient_api::{
    components::core::{
        app::main_scene,
        camera::aspect_ratio_from_window,
        player::{local_user_id, player, user_id},
        transform::{lookat_center, rotation, translation},
    },
    concepts::make_perspective_infinite_reverse_camera,
    player::KeyCode,
    prelude::*,
};
use components::{player_camera_ref, view_vertical_rotation};

#[main]
fn main() {
    let world_front: Vec3 = Vec3::X;
    let world_right: Vec3 = Vec3::Y;
    let world_up: Vec3 = Vec3::Z;

    // offset view sphere up and to the right
    let view_sphere_offset = world_up * 7. + world_right * 2.;
    let target_offset:Vec3 = view_sphere_offset + world_front * 7.;
    let eye_offset:Vec3 = view_sphere_offset - world_front * 7.;

    spawn_query((player(), user_id())).bind(move |players| {
        for (id, (_, user)) in players {
            // First, we check if this player is the "local" player, and only then do we attach a camera
            if user == entity::get_component(entity::resources(), local_user_id()).unwrap() {
                let camera = Entity::new()
                    .with_merge(make_perspective_infinite_reverse_camera())
                    .with(aspect_ratio_from_window(), EntityId::resources())
                    .with_default(main_scene())
                    .with(user_id(), user)
                    .with(translation(), eye_offset)
                    .with(lookat_center(), target_offset)
                    .spawn();

                entity::add_components(id, Entity::new().with(player_camera_ref(), camera));
            }
        }
    });
    // Since we're only attaching player_camera_ref to the local player, this system will only
    // run for the local player
    query((player(), player_camera_ref())).each_frame(move |players| {
        for (player_id, (_, camera_id )) in players {
                // calclulate camera position
                // this is like an arc ball camera. There is a point offset from the players position that is the center of a sphere
                // There is eye on one point of the sphere and a target on the other
                // There is a quaternion that is updated with horizontal and vertical rotations from mouse input
                // there is a vector that is calculated from the world front vector and the rotation quaternion
                //
                let player_rotation = entity::get_component(player_id, rotation()).unwrap();
                let player_right = player_rotation * world_right;
                let view_vertical_rotation = entity::get_component(player_id, view_vertical_rotation()).unwrap();
                let camera_rotation_quat = player_rotation * view_vertical_rotation;

                let player_position = entity::get_component(player_id, translation()).unwrap();

                let camera_front = camera_rotation_quat * world_front;
                let target_projection = camera_front * 30.;
                let view_sphere_offset = world_up * 7. + player_right * 2.;
                let target_position = player_position + view_sphere_offset + target_projection;
                // update camera lookat
                entity::set_component(camera_id, lookat_center(), target_position);

                let camera_projection = camera_front * Vec3::NEG_ONE * 10.;
                let eye_position: Vec3 = player_position + view_sphere_offset + camera_projection;
                entity::set_component(camera_id, translation(), eye_position);
        }
    });

    ambient_api::messages::Frame::subscribe(move |_| {
        let (delta, pressed) = player::get_raw_input_delta();

        let mut input_direction = Vec2::ZERO;
        if pressed.keys.contains(&KeyCode::W) {
            input_direction.x += 1.0;
        }
        if pressed.keys.contains(&KeyCode::S) {
            input_direction.x -= 1.0;
        }
        if pressed.keys.contains(&KeyCode::A) {
            input_direction.y -= 1.0;
        }
        if pressed.keys.contains(&KeyCode::D) {
            input_direction.y += 1.0;
        }

        messages::Input::new(
            input_direction,
            delta.mouse_position.x,
            delta.mouse_position.y
        ).send_server_reliable();
    });
}