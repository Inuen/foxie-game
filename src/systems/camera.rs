use crate::{Game, Player, PlayerState};
use bevy::asset::AssetServer;
use bevy::core_pipeline::fxaa::Fxaa;
use bevy::input::ButtonInput;
use bevy::math::{Quat, Vec3};
use bevy::prelude::{default, Camera, Camera3dBundle, Commands, KeyCode, Query, Res, ResMut, Transform, With};
use std::ops::Range;

const CAMERA_KEYBOARD_ZOOM_SPEED: f32 = 10.0;
const CAMERA_KEYBOARD_ORBIT_SPEED: f32 = 0.02;
const CAMERA_ZOOM_RANGE: Range<f32> = 140.0..1050.0;
/// The sensitivity of the mouse movement
// const MOUSE_SENSITIVITY: f32 = 0.1;
/// Speed for moving the camera forward/backward
// const CAMERA_SPEED: f32 = 10.0;

pub fn spawn_camera(commands: &mut Commands, asset_server: &AssetServer, player: &Player) {
    // Create the camera. Add an environment map and skybox so the water has
    // something interesting to reflect, other than the cube. Enable deferred
    // rendering by adding depth and deferred prepasses. Turn on FXAA to make
    // the scene look a little nicer. Finally, add screen space reflections.
    // commands.spawn(Camera3dBundle {
    //     transform: Transform::from_xyz(100.0, 100.0, 150.0)
    //         .looking_at(Vec3::new(0.0, 20.0, 0.0), Vec3::Y),
    //     ..default()
    // });
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(player.x, player.z + 550.0, -630.0)
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            camera: Camera {
                hdr: true,
                ..default()
            },
            ..default()
        })
        // .insert(EnvironmentMapLight {
        //     diffuse_map: asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
        //     specular_map: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
        //     intensity: 5000.0,
        // })
        // .insert(Skybox {
        //     image: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
        //     brightness: 5000.0,
        // })
        // .insert(ScreenSpaceReflectionsBundle::default())
        .insert(Fxaa::default());
}

// Processes input related to camera movement.
pub fn move_camera(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut cameras: Query<&mut Transform, With<Camera>>,
    game: Res<Game>,
) {
    let (mut distance_delta, mut theta_delta) = (0.0, 0.0);

    // Handle keyboard events.
    if keyboard_input.pressed(KeyCode::KeyW) {
        distance_delta -= CAMERA_KEYBOARD_ZOOM_SPEED;

    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        distance_delta += CAMERA_KEYBOARD_ZOOM_SPEED;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        theta_delta += CAMERA_KEYBOARD_ORBIT_SPEED;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        theta_delta -= CAMERA_KEYBOARD_ORBIT_SPEED;
    }

    let player_vec3 = Vec3::new(game.player.x, 0.0, game.player.z);


    // Update transforms.
    for mut camera_transform in cameras.iter_mut() {

        let local_z = camera_transform.local_z().as_vec3().normalize_or_zero();
        if distance_delta != 0.0 {
            camera_transform.translation = (camera_transform.translation.length() + distance_delta)
                .clamp(CAMERA_ZOOM_RANGE.start, CAMERA_ZOOM_RANGE.end)
                * local_z;

        }
        if theta_delta != 0.0 {
            camera_transform
                .translate_around(Vec3::new(game.player.x, 1.0, game.player.z), Quat::from_axis_angle(Vec3::Y, theta_delta));
            camera_transform.look_at(player_vec3, Vec3::Y);
        }
    }
}

pub fn follow_player(
    mut cameras: Query<&mut Transform, With<Camera>>,
    mut game: ResMut<Game>,
) {
    for mut camera_transform in cameras.iter_mut() {
        if let PlayerState::Running{x, z} = game.player.state {
            camera_transform.translation[0]+=x;
            camera_transform.translation[2]+=z;
            game.player.state = PlayerState::Idle;
        }
    }
}