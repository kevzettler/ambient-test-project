use ambient_api::{
    entity::{AnimationAction, AnimationController},
    prelude::*,
};
use num_derive::FromPrimitive;

// state machine from:
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2015&gist=ee3e4df093c136ced7b394dc7ffb78e1
#[derive(Debug, PartialEq, Clone, Copy, FromPrimitive)]
pub enum PlayerAnimationState{
    Idle,
    Walking,
}

#[derive(Debug, Clone, Copy)]
pub enum PlayerAnimationEvent {
    Stop,
    Walk,
}

impl PlayerAnimationState {
    pub fn transition(self, entity_id:EntityId, event: PlayerAnimationEvent) -> PlayerAnimationState{
         return match (self, event) {
            (PlayerAnimationState::Idle, PlayerAnimationEvent::Walk) => {
                println!("State machine Idle -> Walk!");
                entity::set_animation_controller(
                    entity_id,
                    AnimationController {
                        actions: &[AnimationAction {
                            clip_url: &asset::url("assets/mecha.glb/animations/walk_2.anim").unwrap(),
                            looping: true,
                            weight: 1.,
                        }],
                        apply_base_pose: false,
                    },
                );

                PlayerAnimationState::Walking
            },
            (PlayerAnimationState::Walking, PlayerAnimationEvent::Walk) => PlayerAnimationState::Walking,
            (PlayerAnimationState::Walking, PlayerAnimationEvent::Stop) => {
                println!("State machine Walking -> Stop!");
                entity::set_animation_controller(
                    entity_id,
                    AnimationController {
                        actions: &[AnimationAction {
                            clip_url: &asset::url("assets/mecha.glb/animations/idle_1.anim").unwrap(),
                            looping: true,
                            weight: 1.,
                        }],
                        apply_base_pose: false,
                    },
                );

                PlayerAnimationState::Idle
            },
            (PlayerAnimationState::Idle, PlayerAnimationEvent::Stop) => PlayerAnimationState::Idle,
         };
    }
}
