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
}

#[derive(Debug, Clone, Copy)]
pub enum PlayerAnimationEvent {
    Stop,
    Walk,
}

impl PlayerAnimationState {
    pub fn transition(
        self,
        entity_id: EntityId,
        event: PlayerAnimationEvent,
    ) -> PlayerAnimationState {
        return match (self, event) {
            (PlayerAnimationState::Idle, PlayerAnimationEvent::Walk) => {
                let anim_player_id =
                    entity::get_component(entity_id, apply_animation_player()).unwrap();
                let anim_player = AnimationPlayer(anim_player_id);
                let walk = PlayClipFromUrlNode::new(
                    asset::url("assets/mecha.glb/animations/walk_4.anim").unwrap(),
                );
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
                println!("State machine Walking -> Stop!");
                anim_player.play(idle);
                PlayerAnimationState::Idle
            }
            (PlayerAnimationState::Idle, PlayerAnimationEvent::Stop) => PlayerAnimationState::Idle,
        };
    }
}
