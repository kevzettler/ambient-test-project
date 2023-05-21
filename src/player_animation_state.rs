use ambient_api::{
    animation::{AnimationPlayer, BlendNode, PlayClipFromUrlNode},
    components::core::animation::apply_animation_player,
    entity::get_component,
    prelude::*,
};
use num_derive::FromPrimitive;

// state machine from:
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2015&gist=ee3e4df093c136ced7b394dc7ffb78e1
#[derive(Debug, PartialEq, Clone, Copy, FromPrimitive)]
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

impl PlayerAnimationState {
    pub fn transition(
        self,
        entity_id: EntityId,
        event: PlayerAnimationEvent,
    ) -> PlayerAnimationState {
        return match (self, event) {
            (PlayerAnimationState::Walking, PlayerAnimationEvent::Dash) => {
                println!("State machine Idle -> Walk!");
                entity::set_animation_controller(
                    entity_id,
                    AnimationController {
                        actions: &[AnimationAction {
                            clip_url: &asset::url("assets/mecha.glb/animations/dash_0.anim")
                                .unwrap(),
                            looping: true,
                            weight: 1.,
                        }],
                        apply_base_pose: false,
                    },
                );

                PlayerAnimationState::Dashing
            }
            (PlayerAnimationState::Dashing, PlayerAnimationEvent::Dash) => {
                PlayerAnimationState::Dashing
            }
            (PlayerAnimationState::Idle, PlayerAnimationEvent::Dash) => {
                println!("State machine Idle -> Walk!");
                entity::set_animation_controller(
                    entity_id,
                    AnimationController {
                        actions: &[AnimationAction {
                            clip_url: &asset::url("assets/mecha.glb/animations/dash_0.anim")
                                .unwrap(),
                            looping: true,
                            weight: 1.,
                        }],
                        apply_base_pose: false,
                    },
                );

                PlayerAnimationState::Dashing
            }
            (PlayerAnimationState::Dashing, PlayerAnimationEvent::Walk) => {
                println!("State machine Idle -> Walk!");
                entity::set_animation_controller(
                    entity_id,
                    AnimationController {
                        actions: &[AnimationAction {
                            clip_url: &asset::url("assets/mecha.glb/animations/walk_2.anim")
                                .unwrap(),
                            looping: true,
                            weight: 1.,
                        }],
                        apply_base_pose: false,
                    },
                );

                PlayerAnimationState::Walking
            }

            (PlayerAnimationState::Idle, PlayerAnimationEvent::Walk) => {
                let anim_player_id =
                    entity::get_component(entity_id, apply_animation_player()).unwrap();
                let anim_player = AnimationPlayer(anim_player_id);
                let walk = PlayClipFromUrlNode::new(
                    asset::url("assets/mecha.glb/animations/walk_4.anim").unwrap(),
                );
                walk.apply_base_pose(true);
                println!("State machine Idle -> Walk!");
                anim_player.play(walk);
                PlayerAnimationState::Walking
            }
            (PlayerAnimationState::Walking, PlayerAnimationEvent::Walk) => {
                PlayerAnimationState::Walking
            }
            (PlayerAnimationState::Walking, PlayerAnimationEvent::Stop) => {
                let anim_player_id =
                    entity::get_component(entity_id, apply_animation_player()).unwrap();
                let anim_player = AnimationPlayer(anim_player_id);
                let idle = PlayClipFromUrlNode::new(
                    asset::url("assets/mecha.glb/animations/idle_1.anim").unwrap(),
                );
                idle.apply_base_pose(true);
                println!("State machine Walking -> Stop!");
                anim_player.play(idle);
                PlayerAnimationState::Idle
            }
            (PlayerAnimationState::Dashing, PlayerAnimationEvent::Stop) => {
                println!("State machine Walking -> Stop!");
                entity::set_animation_controller(
                    entity_id,
                    AnimationController {
                        actions: &[AnimationAction {
                            clip_url: &asset::url("assets/mecha.glb/animations/idle_1.anim")
                                .unwrap(),
                            looping: true,
                            weight: 1.,
                        }],
                        apply_base_pose: false,
                    },
                );

                PlayerAnimationState::Idle
            }
            (PlayerAnimationState::Idle, PlayerAnimationEvent::Stop) => PlayerAnimationState::Idle,
        };
    }
}
