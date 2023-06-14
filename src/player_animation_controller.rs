use crate::components::player_animation_state;
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

fn lookup_animation_clip(animation_state: &PlayerAnimationState) -> PlayClipFromUrlNode {
    return match animation_state {
        PlayerAnimationState::Idle => {
            PlayClipFromUrlNode::new(asset::url("assets/mecha.glb/animations/idle_1.anim").unwrap())
        }
        PlayerAnimationState::Walking => {
            PlayClipFromUrlNode::new(asset::url("assets/mecha.glb/animations/walk_4.anim").unwrap())
        }
        PlayerAnimationState::Dashing => {
            PlayClipFromUrlNode::new(asset::url("assets/mecha.glb/animations/dash_0.anim").unwrap())
        }
    };
}

pub struct PlayerAnimationController(pub EntityId);
impl PlayerAnimationController {
    pub fn new() -> Self {
        let idle = lookup_animation_clip(&PlayerAnimationState::Idle);
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
        entity_id: EntityId,
        event: PlayerAnimationEvent,
    ) -> PlayerAnimationState {
        let player_animation_id =
            entity::get_component(entity_id, player_animation_state()).unwrap();
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

        println!(
            "animation transition {:?} -> {:?} = {:?}",
            current_state, event, next_state
        );
        let anim_player_id = entity::get_component(entity_id, apply_animation_player()).unwrap();
        let anim_player = AnimationPlayer(anim_player_id);
        let clip = lookup_animation_clip(&next_state);
        entity::set_component(entity_id, player_animation_state(), next_state as u32);
        anim_player.play(clip);
        next_state
    }
}
