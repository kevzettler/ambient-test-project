use ambient_api::{
    components::core::{
        game_objects::player_camera,
        prefab::prefab_from_url,
        primitives::quad,
        rendering::color,
        transform::{lookat_center, scale, translation},
    },
    concepts::{make_perspective_infinite_reverse_camera, make_transformable},
    entity::{AnimationAction, AnimationController},
    prelude::*,
};

#[main]
pub async fn main() -> EventResult {
    // camera
    Entity::new()
        .with_merge(make_perspective_infinite_reverse_camera())
        .with_default(player_camera())
        .with(translation(), Vec3::ONE * 10.)
        .with(lookat_center(), vec3(0., 0., 0.))
        .spawn();

    // ground
    Entity::new()
        .with_merge(make_transformable())
        .with_default(quad())
        .with(scale(), Vec3::ONE * 10.)
        .with(color(), vec4(0.5, 0.5, 0.5, 1.))
        .spawn();

    // mecha
    let unit_id = Entity::new()
        .with_merge(make_transformable())
        .with(prefab_from_url(), asset_url("assets/mecha.glb").unwrap())
        .spawn();

    entity::set_animation_controller(
        unit_id,
        AnimationController {
            actions: &[AnimationAction {
                clip_url: &asset_url("assets/mecha.glb/animations/walk_1.anim").unwrap(),
                looping: true,
                weight: 1.,
            }],
            apply_base_pose: false,
        },
    );

    println!("Hello, Ambient!");

    EventOk
}
