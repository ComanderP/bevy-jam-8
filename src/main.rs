use bevy::prelude::*;
use bevy::{asset::AssetMetaCheck, color::palettes::css::AQUAMARINE};
use bevy_vector_shapes::prelude::*;


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
        .insert_resource(ClearColor(Color::Srgba(AQUAMARINE)))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
