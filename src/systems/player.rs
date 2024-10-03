use bevy::prelude::{default, KeyCode, Query, Res, ResMut, Transform};
use bevy::input::ButtonInput;
use std::f32::consts::PI;
use bevy::math::{Quat, Vec3};
use crate::{Game, Move, Player, PlayerState};

const PLAYER_SPEED: f32 = 10.0;

pub fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut transforms: Query<&mut Transform>,
    mut game: ResMut<Game>,
) where Player: Move {
    let mut x_vector = 0.0;
    let mut z_vector = 0.0;
    let mut rotation = 0.0;
    let mut moving = false;


    // Handle keyboard events.
    if keyboard_input.pressed(KeyCode::KeyT) {
        z_vector += PLAYER_SPEED;
        moving = true;
        rotation = 0.0;
    }
    if keyboard_input.pressed(KeyCode::KeyG) {
        z_vector -= PLAYER_SPEED;
        moving = true;
        rotation = PI;
    }
    if keyboard_input.pressed(KeyCode::KeyF) {
        x_vector += PLAYER_SPEED;
        moving = true;
        rotation = PI / 2.0;
    }
    if keyboard_input.pressed(KeyCode::KeyH) {
        x_vector -= PLAYER_SPEED;
        moving = true;
        rotation = -PI / 2.;
    }

    if moving {
        game.player.state = PlayerState::Running { x: x_vector, z: z_vector };
        game.player.move_by_vector(x_vector, z_vector);
        *transforms.get_mut(game.player.entity.unwrap()).unwrap() = Transform {
                            translation: Vec3::new(
                                game.player.x,
                                0.0,
                                game.player.z,
                            ),
                            rotation: Quat::from_rotation_y(rotation),
                            ..default()
                        };
        // for cam in cameras {
        //     cam.
        // }
        // *transforms.get
    }
}