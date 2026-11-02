use bevy::color::palettes::tailwind::*;
use bevy::prelude::*;
use bevy::{asset::AssetMetaCheck, color::palettes::css::AQUAMARINE};
use bevy_vector_shapes::prelude::*;
use std::f32::consts::{PI, TAU};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            // Wasm builds will check for meta files (that don't exist) if this isn't set.
            // This causes errors and even panics in web builds on itch.
            // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
            meta_check: AssetMetaCheck::Never,
            ..default()
        }))
        .add_plugins(Shape2dPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, draw)
        .run();
}

fn draw(mut painter: ShapePainter, time: Res<Time>) {
    painter.hollow = true;
    painter.thickness = 1.0;
    painter.set_color(AMBER_300);
    painter.arc(
        100.0,
        0.0,
        TAU * (time.elapsed_secs().rem_euclid(10.0) / (10.0)),
    );

    painter.hollow = true;
    painter.translate(Vec3::ONE * 200.0);
    painter.thickness = 1.0;
    painter.set_color(AQUAMARINE);
    painter.arc(
        100.0,
        0.0,
        PI * (time.elapsed_secs().rem_euclid(20.0) / (10.0)),
    );
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    // commands.spawn(Sprite {
    //     image: asset_server.load("ducky.png"),
    //     ..Default::default()
    // });
}
