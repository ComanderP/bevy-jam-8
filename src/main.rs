use bevy::color::palettes::css::*;
use bevy::prelude::*;
use bevy::asset::AssetMetaCheck;
use bevy::window::WindowResolution;
use bevy_vector_shapes::prelude::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            // Wasm builds will check for meta files (that don't exist) if this isn't set.
            // This causes errors and even panics in web builds on itch.
            // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
            meta_check: AssetMetaCheck::Never,
            ..default()
        }).set(WindowPlugin {
            primary_window: Some(Window {
                name: Some("Oppenheimer".into()),
                title: "Oppenheimer".into(),
                resolution: WindowResolution::new(920, 920),
                ..default()
            }),
          ..default()
        }))
        .add_plugins(Shape2dPlugin::default())
        .insert_resource(ClearColor(Color::Srgba(CORNFLOWER_BLUE)))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
