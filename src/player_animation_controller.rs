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
    Punching,
    Jumping,
}

#[derive(Debug, Clone, Copy)]
pub enum PlayerAnimationEvent {
    Stop,
    Walk,
    Dash,
    Punch,
    Jump,
}

fn lookup_clip_path(animation_state: PlayerAnimationState) -> &'static str {
    return match animation_state {
        PlayerAnimationState::Idle => "assets/mecha.glb/animations/idle_2.anim",
        PlayerAnimationState::Walking => "assets/mecha.glb/animations/walk_5.anim",
        PlayerAnimationState::Dashing => "assets/mecha.glb/animations/dash_0.anim",
        PlayerAnimationState::Punching => "assets/mecha.glb/animations/punch_4.anim",
        PlayerAnimationState::Jumping => "assets/mecha.glb/animations/dash_0.anim",
    };
}

pub struct PlayerAnimationController(pub EntityId);
impl PlayerAnimationController {
    pub fn new(target_id: EntityId) -> Self {
        let idle_path = lookup_clip_path(PlayerAnimationState::Idle);
        let idle = PlayClipFromUrlNode::new(asset::url(idle_path).unwrap());
        let anim_player = AnimationPlayer::new(idle);

        entity::add_components(
            target_id,
            Entity::new()
                .with(apply_animation_player(), anim_player.0)
                .with(player_animation_state(), PlayerAnimationState::Idle as u32),
        );

        Self(target_id)
    }

    pub fn transition(&mut self, event: PlayerAnimationEvent) -> PlayerAnimationState {
        let target_entity_id = self.0;

        let player_animation_id =
            entity::get_component(target_entity_id, player_animation_state()).unwrap();
        let current_state = match num::FromPrimitive::from_u32(player_animation_id) {
            Some(animation_state) => animation_state,
            None => {
                eprintln!("Unkown animation state!!!");
                PlayerAnimationState::Idle
            }
        };

        let mut clip_looping = false;
        let next_state = match (current_state, event) {
            // Idle
            (PlayerAnimationState::Idle, PlayerAnimationEvent::Walk) => {
                clip_looping = true;
                PlayerAnimationState::Walking
            }
            (PlayerAnimationState::Idle, PlayerAnimationEvent::Stop) => PlayerAnimationState::Idle,

            (PlayerAnimationState::Idle, PlayerAnimationEvent::Dash) => {
                PlayerAnimationState::Dashing
            }

            (PlayerAnimationState::Idle, PlayerAnimationEvent::Punch) => {
                PlayerAnimationState::Punching
            }

            (PlayerAnimationState::Idle, PlayerAnimationEvent::Jump) => {
                PlayerAnimationState::Jumping
            }

            // Walking
            (PlayerAnimationState::Walking, PlayerAnimationEvent::Walk) => {
                clip_looping = true;
                PlayerAnimationState::Walking
            }
            (PlayerAnimationState::Walking, PlayerAnimationEvent::Stop) => {
                PlayerAnimationState::Idle
            }
            (PlayerAnimationState::Walking, PlayerAnimationEvent::Dash) => {
                PlayerAnimationState::Dashing
            }
            (PlayerAnimationState::Walking, PlayerAnimationEvent::Punch) => {
                PlayerAnimationState::Punching
            }

            (PlayerAnimationState::Walking, PlayerAnimationEvent::Jump) => {
                PlayerAnimationState::Jumping
            }

            //Dashing
            (PlayerAnimationState::Dashing, PlayerAnimationEvent::Dash) => {
                PlayerAnimationState::Dashing
            }
            (PlayerAnimationState::Dashing, PlayerAnimationEvent::Stop) => {
                PlayerAnimationState::Idle
            }

            (PlayerAnimationState::Dashing, PlayerAnimationEvent::Walk) => {
                clip_looping = true;
                PlayerAnimationState::Walking
            }

            (PlayerAnimationState::Dashing, PlayerAnimationEvent::Punch) => {
                PlayerAnimationState::Punching
            }

            (PlayerAnimationState::Dashing, PlayerAnimationEvent::Jump) => {
                PlayerAnimationState::Jumping
            }

            //Punching
            (PlayerAnimationState::Punching, PlayerAnimationEvent::Stop) => {
                PlayerAnimationState::Idle
            }

            (PlayerAnimationState::Punching, PlayerAnimationEvent::Walk) => {
                PlayerAnimationState::Walking
            }

            (PlayerAnimationState::Punching, PlayerAnimationEvent::Dash) => {
                PlayerAnimationState::Dashing
            }

            (PlayerAnimationState::Punching, PlayerAnimationEvent::Punch) => {
                PlayerAnimationState::Punching
            }

            (PlayerAnimationState::Punching, PlayerAnimationEvent::Jump) => {
                PlayerAnimationState::Jumping
            }

            //Jumping
            (PlayerAnimationState::Jumping, PlayerAnimationEvent::Stop) => {
                PlayerAnimationState::Jumping
            }

            (PlayerAnimationState::Jumping, PlayerAnimationEvent::Walk) => {
                clip_looping = true;
                PlayerAnimationState::Walking
            }

            (PlayerAnimationState::Jumping, PlayerAnimationEvent::Dash) => {
                PlayerAnimationState::Dashing
            }

            (PlayerAnimationState::Jumping, PlayerAnimationEvent::Punch) => {
                PlayerAnimationState::Jumping
            }
            (PlayerAnimationState::Jumping, PlayerAnimationEvent::Jump) => {
                PlayerAnimationState::Jumping
            }
        };

        if current_state.eq(&next_state) {
            // return early no state transition
            return next_state;
        }

        println!(
            "animation transition {:?} -> {:?} = {:?}",
            current_state, event, next_state
        );

        let clip_path = lookup_clip_path(next_state);
        let clip = PlayClipFromUrlNode::new(asset::url(clip_path).unwrap());
        clip.looping(clip_looping);

        let anim_player_id =
            entity::get_component(target_entity_id, apply_animation_player()).unwrap();

        let anim_player = AnimationPlayer(anim_player_id);
        anim_player.play(clip);

        entity::set_component(
            target_entity_id,
            player_animation_state(),
            next_state as u32,
        );
        next_state
    }
}
