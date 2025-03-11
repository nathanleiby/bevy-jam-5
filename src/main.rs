// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod bodies;
mod debug;

use bevy::prelude::*;
use bevy::sprite::Wireframe2dPlugin;
use bevy::{asset::AssetMetaCheck, color::palettes::css::PURPLE};
use bevy_dev_tools::fps_overlay::FpsOverlayConfig;
use bodies::bodies_plugin;

use bevy_inspector_egui::prelude::*;
use bevy_kira_audio::prelude::*;
use debug::debug_plugin;

// `InspectorOptions` are completely optional
#[derive(Reflect, Resource, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
struct Configuration {
    name: String,
    #[inspector(min = 0.0, max = 1.0)]
    option: f32,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(PURPLE.into())) // starting background color
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            // Wasm builds will check for meta files (that don't exist) if this isn't set.
            // This causes errors and even panics in web builds on itch.
            // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
            meta_check: AssetMetaCheck::Never,
            ..default()
        }))
        .add_plugins(AudioPlugin)
        .add_plugins(debug_plugin)
        // Game
        .add_plugins(Wireframe2dPlugin)
        .add_systems(Startup, setup_camera)
        .add_plugins(bodies_plugin)
        .add_systems(Update, quit_game)
        .add_systems(Startup, start_background_audio)
        // random prototyping
        .add_systems(Update, customize_config)
        .add_systems(Update, change_clear_color)
        .run();
}

// -- audio --
// TODO: support toggling on and off with a button
fn start_background_audio(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    // audio.play(asset_server.load("loop.ogg")).looped();
}

// -- game --
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

// -- random prototyping --
fn customize_config(input: Res<ButtonInput<KeyCode>>, mut overlay: ResMut<FpsOverlayConfig>) {
    // press "1" or "2" to adjust text params
    if input.just_pressed(KeyCode::Digit1) {
        // Changing resource will affect overlay
        overlay.text_config.color = Color::srgb(1.0, 0.0, 0.0);
    }
    if input.just_pressed(KeyCode::Digit2) {
        overlay.text_config.font_size -= 2.0;
    }
}

fn change_clear_color(input: Res<ButtonInput<KeyCode>>, mut clear_color: ResMut<ClearColor>) {
    // press "c" to change color
    if input.just_pressed(KeyCode::KeyC) {
        clear_color.0 = Color::srgb(0.5, 0.5, 0.9);
    }
}

fn quit_game(mut exit: EventWriter<AppExit>, keyboard: Res<ButtonInput<KeyCode>>) {
    // press "q" to quit
    if keyboard.just_pressed(KeyCode::KeyQ) {
        exit.send(AppExit::Success);
    }
}
