use crate::components::{player_animation_controller_ref, player_animation_state};
use ambient_api::{
    animation::{AnimationPlayer, PlayClipFromUrlNode},
    components::core::animation::apply_animation_player,
    prelude::*,
};

use num_derive::FromPrimitive;
// state machine from:
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2015&gist=ee3e4df093c136ced7b394dc7ffb78e1
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, FromPrimitive)]
pub enum PlayerAnimationState {
    Idle,
    Walking,
    Dashing,
}

#[derive(Debug, Clone, Copy)]
pub enum PlayerAnimationEvent {
    Stop,
    Walk,
    Dash,
}

fn lookup_clip_path(animation_state: PlayerAnimationState) -> &'static str {
    return match animation_state {
        PlayerAnimationState::Idle => "assets/mecha.glb/animations/idle_1.anim",
        PlayerAnimationState::Walking => "assets/mecha.glb/animations/walk_4.anim",
        PlayerAnimationState::Dashing => "assets/mecha.glb/animations/dash_0.anim",
    };
}

pub struct PlayerAnimationController(pub EntityId);
impl PlayerAnimationController {
    pub fn new() -> Self {
        let idle_path = lookup_clip_path(PlayerAnimationState::Idle);
        let idle = PlayClipFromUrlNode::new(asset::url(idle_path).unwrap());
        let anim_player = AnimationPlayer::new(idle);

        let entity_id = Entity::new()
            .with(apply_animation_player(), anim_player.0)
            .with(player_animation_state(), PlayerAnimationState::Idle as u32)
            .with(name(), "Animation Controller".to_string())
            .spawn();

        Self(entity_id)
    }

    pub fn transition(
        &mut self,
        target_entity_id: EntityId,
        event: PlayerAnimationEvent,
    ) -> PlayerAnimationState {
        let player_animation_controller_id =
            entity::get_component(target_entity_id, player_animation_controller_ref()).unwrap();

        let player_animation_id =
            entity::get_component(player_animation_controller_id, player_animation_state())
                .unwrap();

        let current_state = match num::FromPrimitive::from_u32(player_animation_id) {
            Some(animation_state) => animation_state,
            None => {
                eprintln!("Unkown animation state!!!");
                PlayerAnimationState::Idle
            }
        };

        let next_state = match (current_state, event) {
            (PlayerAnimationState::Idle, PlayerAnimationEvent::Walk) => {
                PlayerAnimationState::Walking
            }
            (PlayerAnimationState::Idle, PlayerAnimationEvent::Stop) => PlayerAnimationState::Idle,

            (PlayerAnimationState::Idle, PlayerAnimationEvent::Dash) => {
                PlayerAnimationState::Dashing
            }

            (PlayerAnimationState::Walking, PlayerAnimationEvent::Walk) => {
                PlayerAnimationState::Walking
            }
            (PlayerAnimationState::Walking, PlayerAnimationEvent::Stop) => {
                PlayerAnimationState::Idle
            }
            (PlayerAnimationState::Walking, PlayerAnimationEvent::Dash) => {
                PlayerAnimationState::Dashing
            }

            (PlayerAnimationState::Dashing, PlayerAnimationEvent::Dash) => {
                PlayerAnimationState::Dashing
            }
            (PlayerAnimationState::Dashing, PlayerAnimationEvent::Stop) => {
                PlayerAnimationState::Idle
            }

            (PlayerAnimationState::Dashing, PlayerAnimationEvent::Walk) => {
                PlayerAnimationState::Walking
            }
        };

        if (current_state.eq(&next_state)) {
            // return early no state transition
            return next_state;
        }

        println!(
            "animation transition {:?} -> {:?} = {:?}",
            current_state, event, next_state
        );
        let clip_path = lookup_clip_path(next_state);
        let clip = PlayClipFromUrlNode::new(asset::url(clip_path).unwrap());

        let anim_player_id =
            entity::get_component(player_animation_controller_id, apply_animation_player())
                .unwrap();

        let anim_player = AnimationPlayer(anim_player_id);
        anim_player.play(clip);

        entity::set_component(
            player_animation_controller_id,
            player_animation_state(),
            next_state as u32,
        );
        next_state
    }
}
