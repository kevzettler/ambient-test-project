use ambient_api::{
    animation::{AnimationPlayer, BlendNode, PlayClipFromUrlNode},
    components::core::animation::apply_animation_player,
    entity::get_component,
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

pub struct PlayerAnimationController {
    state: PlayerAnimationState,
    map: HashMap<PLayerAnimationState, PlayClipFromUrlNode>,
}

impl PlayerAnimationController {
    pub fn new() -> Self {
        let state = PlayerAnimationState;
        let events = PlayerAnimationEvent;
        let mut map = HashMap::new();
        map.insert(
            PlayerAnimationState::Idle,
            PlayClipFromUrlNode::new(
                asset::url("assets/mecha.glb/animations/idle_1.anim").unwrap(),
            ),
        );

        map.insert(
            PlayerAnimationState::Walking,
            PlayClipFromUrlNode::new(
                asset::url("assets/mecha.glb/animations/walk_4.anim").unwrap(),
            ),
        );

        map.insert(
            PlayerAnimationState::Walking,
            PlayClipFromUrlNode::new(
                asset::url("assets/mecha.glb/animations/dash_0.anim").unwrap(),
            ),
        );

        PlayerAnimationController {
            state: PlayerAnimationState::Idle,
            map,
        }
    }

    pub fn transition(
        &mut self,
        entity_id: EntityId,
        event: PlayerAnimationEvent,
    ) -> PlayerAnimationState {
        return match (self.state, event) {
            (PlayerAnimationState::Idle, PlayerAnimationEvent::Walk) => {
                println!("State machine Idle -> Walk!");
                let anim_player_id =
                    entity::get_component(entity_id, apply_animation_player()).unwrap();
                let anim_player = AnimationPlayer(anim_player_id);
                let walk = self.map.get(&PlayerAnimationState::Walking).unwarap();
                anim_player.play(walk);
                PlayerAnimationState::Walking
            }
            (PlayerAnimationState::Idle, PlayerAnimationEvent::Stop) => PlayerAnimationState::Idle,

            (PlayerAnimationState::Idle, PlayerAnimationEvent::Dash) => {
                println!("State machine Idle -> Dash!");
                let anim_player_id =
                    entity::get_component(entity_id, apply_animation_player()).unwrap();
                let anim_player = AnimationPlayer(anim_player_id);
                let dashing = self.map.get(&PlayerAnimationState::Dashing).unwarap();
                anim_player.play(idle);
                PlayerAnimationState::Dashing
            }

            (PlayerAnimationState::Walking, PlayerAnimationEvent::Walk) => {
                PlayerAnimationState::Walking
            }
            (PlayerAnimationState::Walking, PlayerAnimationEvent::Stop) => {
                println!("State machine Walking -> Stop!");
                let anim_player_id =
                    entity::get_component(entity_id, apply_animation_player()).unwrap();
                let anim_player = AnimationPlayer(anim_player_id);
                let idle = self.map.get(&PlayerAnimationState::Idle).unwarap();
                anim_player.play(idle);
                PlayerAnimationState::Idle
            }
            (PlayerAnimationState::Walking, PlayerAnimationEvent::Dash) => {
                println!("State machine Walking -> Dash!");
                let anim_player_id =
                    entity::get_component(entity_id, apply_animation_player()).unwrap();
                let anim_player = AnimationPlayer(anim_player_id);
                let dashing = self.map.get(&PlayerAnimationState::Dashing).unwarap();
                anim_player.play(idle);
                PlayerAnimationState::Dashing
            }

            (PlayerAnimationState::Dashing, PlayerAnimationEvent::Dash) => {
                PlayerAnimationState::Dashing
            }
            (PlayerAnimationState::Dashing, PlayerAnimationEvent::Stop) => {
                println!("State machine Dashing -> Stop!");
                let anim_player_id =
                    entity::get_component(entity_id, apply_animation_player()).unwrap();
                let anim_player = AnimationPlayer(anim_player_id);
                let idle = self.map.get(&PlayerAnimationState::Idle).unwarap();
                anim_player.play(idle);
                PlayerAnimationState::Idle
            }

            (PlayerAnimationState::Dashing, PlayerAnimationEvent::Walk) => {
                println!("State machine Dashing -> Walk!");
                let anim_player_id =
                    entity::get_component(entity_id, apply_animation_player()).unwrap();
                let anim_player = AnimationPlayer(anim_player_id);
                let walk = self.map.get(&PlayerAnimationState::Walking).unwarap();
                anim_player.play(walk);
                PlayerAnimationState::Walking
            }
        };
    }
}
