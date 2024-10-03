//! Plays animations from a skinned glTF.

mod systems;
mod components;

use std::f32::consts::PI;
use std::time::Duration;



use bevy::{
    animation::animate_targets,
    pbr::CascadeShadowConfigBuilder,
    pbr::{
        ExtendedMaterial, MaterialExtension,
    },
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef, ShaderType},
};
use systems::{animation, camera, player};
use systems::animation::Animations;



const BOARD_WIDTH: f32 = 500000.0;
const BOARD_LENGTH: f32 = 500000.0;

/// Parameters to the water shader.
#[derive(ShaderType, Debug, Clone)]
struct WaterSettings {
    /// How much to displace each octave each frame, in the u and v directions.
    /// Two octaves are packed into each `vec4`.
    octave_vectors: [Vec4; 2],
    /// How wide the waves are in each octave.
    octave_scales: Vec4,
    /// How high the waves are in each octave.
    octave_strengths: Vec4,
}


/// A custom [`ExtendedMaterial`] that creates animated water ripples.
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct Water {
    /// The normal map image.
    ///
    /// Note that, like all normal maps, this must not be loaded as sRGB.
    #[texture(100)]
    #[sampler(101)]
    normals: Handle<Image>,

    // Parameters to the water shader.
    #[uniform(102)]
    settings: WaterSettings,
}


impl MaterialExtension for Water {
    fn deferred_fragment_shader() -> ShaderRef {
        "shaders/water_material.wgsl".into()
    }
}

#[derive(Debug)]
enum PlayerState {
    Idle,
    Running { x: f32, z: f32 },
}

impl Default for PlayerState {
    fn default() -> Self {
        PlayerState::Idle
    }
}

#[derive(Default, Component, Debug)]
struct Player {
    entity: Option<Entity>,
    state: PlayerState,
    x: f32,
    z: f32
}

pub trait Move {
    fn move_by_vector(&mut self, x_move: f32, z_move: f32);
}

impl Move for Player {
    fn move_by_vector(&mut self, x_move: f32, z_move: f32) {
        self.x += x_move;
        self.z += z_move;
    }
}

#[derive(Resource, Default)]
struct Game {
    player: Player,
}

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 2000.,
        })
        .init_resource::<Game>()
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialPlugin::<ExtendedMaterial<StandardMaterial, Water>>::default())
        .add_systems(Startup, setup)
        .add_systems(Update, setup_scene_once_loaded.before(animate_targets))
        .add_systems(Update, animation::keyboard_animation_control)
        .add_systems(Update, camera::move_camera)
        .add_systems(Update, player::move_player)
        .add_systems(Update, camera::follow_player)
        .run();
}

fn setup(
    mut commands: Commands,
    mut game: ResMut<Game>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    game.player.x = 0.0;
    game.player.z = 0.0;
    // Build the animation graph
    let mut graph = AnimationGraph::new();
    let animations = graph
        .add_clips(
            [
                GltfAssetLabel::Animation(2).from_asset("models/animated/Fox.glb"),
                GltfAssetLabel::Animation(1).from_asset("models/animated/Fox.glb"),
                GltfAssetLabel::Animation(0).from_asset("models/animated/Fox.glb"),
            ]
                .into_iter()
                .map(|path| asset_server.load(path)),
            10.0,
            graph.root,
        )
        .collect();

    // Insert a resource with the current scene information
    let graph = graphs.add(graph);
    commands.insert_resource(Animations {
        animations,
        graph: graph.clone(),
    });

    // Camera
    camera::spawn_camera(&mut commands, &asset_server, &game.player);


    // Plane
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(BOARD_WIDTH, BOARD_LENGTH)),
        material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
        ..default()
    });
    // spawn_water(&mut commands,
    //             &asset_server,
    //             &mut meshes,
    //             &mut water_materials, );


    // Light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, 1.0, -PI / 4.)),
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 200.0,
            maximum_distance: 1150.0,
            ..default()
        }
            .into(),
        ..default()
    });


    // Fox
    game.player.entity = Some(
        commands
            .spawn(SceneBundle {
                transform: Transform {
                    translation: Vec3::new(
                        game.player.x,
                        0.0, // height
                        game.player.z,
                    ),
                    ..default()
                },
                    scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/animated/Fox.glb")),
        ..default()
    }).id());

    println!("Animation controls:");
    println!("  - spacebar: play / pause");
    println!("  - arrow up / down: speed up / slow down animation playback");
    println!("  - arrow left / right: seek backward / forward");
    println!("  - digit 1 / 3 / 5: play the animation <digit> times");
    println!("  - L: loop the animation forever");
    println!("  - return: change animation");
    println!("  - T / F / G / H move up / left / down / right");
}


// Spawns the water plane.
// fn spawn_water(
//     commands: &mut Commands,
//     asset_server: &AssetServer,
//     meshes: &mut Assets<Mesh>,
//     water_materials: &mut Assets<ExtendedMaterial<StandardMaterial, Water>>,
// ) {
//     commands.spawn(MaterialMeshBundle {
//         mesh: meshes.add(Plane3d::default().mesh().size(BOARD_WIDTH, BOARD_LENGTH)),
//         material: water_materials.add(ExtendedMaterial {
//             base: StandardMaterial {
//                 base_color: BLACK.into(),
//                 perceptual_roughness: 0.0,
//                 ..default()
//             },
//             extension: Water {
//                 normals: asset_server.load_with_settings::<Image, ImageLoaderSettings>(
//                     "textures/water_normals.png",
//                     |settings| {
//                         settings.is_srgb = false;
//                         settings.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
//                             address_mode_u: ImageAddressMode::Repeat,
//                             address_mode_v: ImageAddressMode::Repeat,
//                             mag_filter: ImageFilterMode::Linear,
//                             min_filter: ImageFilterMode::Linear,
//                             ..default()
//                         });
//                     },
//                 ),
//                 // These water settings are just random values to create some
//                 // variety.
//                 settings: WaterSettings {
//                     octave_vectors: [
//                         vec4(0.080, 0.059, 0.073, -0.062),
//                         vec4(0.153, 0.138, -0.149, -0.195),
//                     ],
//                     octave_scales: vec4(1.0, 2.1, 7.9, 14.9) * 5.0,
//                     octave_strengths: vec4(0.16, 0.18, 0.093, 0.044),
//                 },
//             },
//         }),
//         transform: Transform::from_scale(Vec3::splat(10000.0)),
//         ..default()
//     });
// }


// Once the scene is loaded, start the animation
fn setup_scene_once_loaded(
    mut commands: Commands,
    animations: Res<Animations>,
    mut players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
) {
    for (entity, mut player) in &mut players {
        let mut transitions = AnimationTransitions::new();

        // Make sure to start the animation via the `AnimationTransitions`
        // component. The `AnimationTransitions` component wants to manage all
        // the animations and will get confused if the animations are started
        // directly via the `AnimationPlayer`.
        transitions
            .play(&mut player, animations.animations[0], Duration::ZERO)
            .repeat();

        commands
            .entity(entity)
            .insert(animations.graph.clone())
            .insert(transitions);
    }
}

