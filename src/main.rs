// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod bodies;

use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle, Wireframe2dConfig, Wireframe2dPlugin};
use bevy::{asset::AssetMetaCheck, color::palettes::css::PURPLE};
use bevy_dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};
use bodies::bodies_plugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.5, 0.5, 0.9))) // starting background color
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            // Wasm builds will check for meta files (that don't exist) if this isn't set.
            // This causes errors and even panics in web builds on itch.
            // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
            meta_check: AssetMetaCheck::Never,
            ..default()
        }))
        .add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_config: TextStyle {
                    // Here we define size of our overlay
                    font_size: 50.0,
                    // We can also change color of the overlay
                    color: Color::srgb(0.0, 1.0, 0.0),
                    // If we want, we can use a custom font
                    font: default(),
                },
            },
        })
        .add_plugins(Wireframe2dPlugin)
        .add_plugins(bodies_plugin)
        .add_systems(Startup, setup)
        // .add_systems(Startup, setup_shapes)
        // .add_systems(Update, sprite_movement)
        // .add_systems(Update, customize_config)
        .add_systems(Update, change_clear_color)
        .add_systems(Update, toggle_wireframe)
        .add_systems(Update, quit_game)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("ducky.png"),
            ..Default::default()
        },
        Direction::Up,
    ));
}

#[derive(Component)]
enum Direction {
    Up,
    Down,
}

/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame.
fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
    for (mut logo, mut transform) in &mut sprite_position {
        match *logo {
            Direction::Up => transform.translation.y += 150. * time.delta_seconds(),
            Direction::Down => transform.translation.y -= 150. * time.delta_seconds(),
        }

        if transform.translation.y > 200. {
            *logo = Direction::Down;
        } else if transform.translation.y < -200. {
            *logo = Direction::Up;
        }
    }
}

fn customize_config(input: Res<ButtonInput<KeyCode>>, mut overlay: ResMut<FpsOverlayConfig>) {
    if input.just_pressed(KeyCode::Digit1) {
        // Changing resource will affect overlay
        overlay.text_config.color = Color::srgb(1.0, 0.0, 0.0);
    }
    if input.just_pressed(KeyCode::Digit2) {
        overlay.text_config.font_size -= 2.0;
    }
}

fn change_clear_color(input: Res<ButtonInput<KeyCode>>, mut clear_color: ResMut<ClearColor>) {
    if input.just_pressed(KeyCode::Space) {
        clear_color.0 = PURPLE.into();
    }
}

fn toggle_wireframe(
    mut wireframe_config: ResMut<Wireframe2dConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::KeyS) {
        wireframe_config.global = !wireframe_config.global;
    }
}

fn quit_game(mut exit: EventWriter<AppExit>, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::KeyQ) {
        exit.send(AppExit::Success);
    }
}
