//! Scene with a green [`Rectangle`] as background and an [`Outlined2dTexture`] with a black outline.

use bevy::{color::palettes::tailwind, prelude::*};
use bevy_fast_outline::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            FastOutlinePlugin,
        ))
        .add_systems(Startup, setup)
        .run()
}

/// Setup scene
fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(ClearColor(tailwind::NEUTRAL_500.into()));
    commands.spawn(Camera2d);

    // Background object
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(600., 600.))),
        MeshMaterial2d(materials.add(Color::from(tailwind::GREEN_500))),
    ));

    commands.spawn((
        Outlined2dTexture {
            texture: asset_server.load("images/human-male.webp"),
            color: Color::BLACK,
        },
        Transform::from_xyz(0., 0., 1.).with_scale(Vec3::splat(8.)),
    ));
}
