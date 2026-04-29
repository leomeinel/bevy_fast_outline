//! [`FastOutlinePlugin`] and related.

use bevy::{
    app::{App, Plugin, Update},
    asset::embedded_asset,
    shader::load_shader_library,
    sprite_render::Material2dPlugin,
};

use crate::outline::{Outline2dMaterial, add_outline};

/// [`Plugin`] for fast 2D pixel art outline.
pub struct FastOutlinePlugin;
impl Plugin for FastOutlinePlugin {
    fn build(&self, app: &mut App) {
        load_shader_library!(app, "types.wgsl");
        embedded_asset!(app, "outline_2d.wgsl");

        app.add_plugins(Material2dPlugin::<Outline2dMaterial>::default());
        app.add_systems(Update, add_outline);
    }
}
