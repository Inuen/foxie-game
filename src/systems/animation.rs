use bevy::prelude::{AnimationGraph, AnimationNodeIndex, AnimationTransitions, KeyCode, Local, Query, Res, Resource};
use bevy::asset::Handle;
use bevy::input::ButtonInput;
use bevy::animation::{AnimationPlayer, RepeatAnimation};
use std::time::Duration;

#[derive(Resource)]
pub struct Animations {
    pub animations: Vec<AnimationNodeIndex>,
    #[allow(dead_code)]
    pub graph: Handle<AnimationGraph>,
}

pub fn keyboard_animation_control(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut animation_players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
    animations: Res<Animations>,
    mut current_animation: Local<usize>,
) {
    for (mut player, mut transitions) in &mut animation_players {
        let Some((&playing_animation_index, _)) = player.playing_animations().next() else {
            continue;
        };
        if keyboard_input.just_pressed(KeyCode::Space) {
            println!("{:?}", keyboard_input);
            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
            if playing_animation.is_paused() {
                playing_animation.resume();
            } else {
                playing_animation.pause();
            }
        }

        if keyboard_input.just_pressed(KeyCode::ArrowUp) {
            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
            let speed = playing_animation.speed();
            playing_animation.set_speed(speed * 1.2);
        }

        if keyboard_input.just_pressed(KeyCode::ArrowDown) {
            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
            let speed = playing_animation.speed();
            playing_animation.set_speed(speed * 0.8);
        }

        if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
            let elapsed = playing_animation.seek_time();
            playing_animation.seek_to(elapsed - 0.1);
        }

        if keyboard_input.just_pressed(KeyCode::ArrowRight) {
            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
            let elapsed = playing_animation.seek_time();
            playing_animation.seek_to(elapsed + 0.1);
        }

        if keyboard_input.just_pressed(KeyCode::Enter) {
            *current_animation = (*current_animation + 1) % animations.animations.len();

            transitions
                .play(
                    &mut player,
                    animations.animations[*current_animation],
                    Duration::from_millis(250),
                )
                .repeat();
        }

        if keyboard_input.just_pressed(KeyCode::Digit1) {
            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
            playing_animation
                .set_repeat(RepeatAnimation::Count(1))
                .replay();
        }

        if keyboard_input.just_pressed(KeyCode::Digit3) {
            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
            playing_animation
                .set_repeat(RepeatAnimation::Count(3))
                .replay();
        }

        if keyboard_input.just_pressed(KeyCode::Digit5) {
            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
            playing_animation
                .set_repeat(RepeatAnimation::Count(5))
                .replay();
        }

        if keyboard_input.just_pressed(KeyCode::KeyL) {
            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
            playing_animation.set_repeat(RepeatAnimation::Forever);
        }
    }
}