/*
 * File: outline.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2026 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

//! Pixel-Perfect one pixel outline for 2D pixel art.

use bevy::{
    asset::{Asset, AssetPath, Assets, Handle, embedded_path},
    color::{Color, LinearRgba},
    ecs::{
        component::Component,
        entity::Entity,
        query::Added,
        system::{Commands, Query, Res, ResMut},
    },
    image::Image,
    math::{Vec2, primitives::Rectangle},
    mesh::{Mesh, Mesh2d},
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderType},
    shader::ShaderRef,
    sprite_render::{AlphaMode2d, Material2d, MeshMaterial2d},
    utils::default,
};

/// [`Image`] with a 2D pixel art outline.
///
/// The outline is pixel-perfect and one pixel wide.
#[derive(Component, Default)]
pub struct Outlined2dTexture {
    /// [`Image`] to be outlined.
    pub texture: Handle<Image>,
    /// [`Color`] for the outline.
    pub color: Color,
}

/// [`Material2d`] to display [`Outlined2dTexture`].
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Default, PartialEq)]
pub(super) struct Outline2dMaterial {
    #[texture(0)]
    #[sampler(1)]
    texture: Handle<Image>,
    #[uniform(2)]
    outline: Outline2d,
}
impl Outline2dMaterial {
    fn with_outline(self, outline: Outline2d) -> Self {
        Self { outline, ..self }
    }
}
impl From<&Outlined2dTexture> for Outline2dMaterial {
    fn from(outline: &Outlined2dTexture) -> Self {
        Self {
            texture: outline.texture.clone(),
            ..default()
        }
    }
}
impl Material2d for Outline2dMaterial {
    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
    fn fragment_shader() -> ShaderRef {
        AssetPath::from_path_buf(embedded_path!("outline_2d.wgsl"))
            .with_source("embedded")
            .into()
    }
}

/// [`ShaderType`] with parameters for a 2D pixel art outline.
#[derive(Default, Clone, Copy, ShaderType, Debug, PartialEq)]
struct Outline2d {
    color: LinearRgba,
    texel_step: Vec2,
    _padding: Vec2,
}
impl Outline2d {
    fn with_texel_step(self, texel_step: Vec2) -> Self {
        Self { texel_step, ..self }
    }
}
impl From<&Outlined2dTexture> for Outline2d {
    fn from(outline_texture: &Outlined2dTexture) -> Self {
        Self {
            color: outline_texture.color.to_linear(),
            ..default()
        }
    }
}

/// Add [`Mesh2d`] and [`MeshMaterial2d`] for [`Outlined2dTexture`]s.
///
/// [`Mesh2d`] will be a [`Rectangle`] with the same size as [`Outlined2dTexture::texture`].
pub(super) fn add_outline(
    query: Query<(Entity, &Outlined2dTexture), Added<Outlined2dTexture>>,
    mut commands: Commands,
    mut materials: ResMut<Assets<Outline2dMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    images: Res<Assets<Image>>,
) {
    for (entity, outline) in query {
        let Some(image) = images.get(&outline.texture) else {
            continue;
        };
        let image_size = image.size_f32();
        let texel_step = 1. / image_size;
        let material = materials.add(
            Outline2dMaterial::from(outline)
                .with_outline(Outline2d::from(outline).with_texel_step(texel_step)),
        );

        commands.entity(entity).insert((
            Mesh2d(meshes.add(Rectangle::from_size(image_size))),
            MeshMaterial2d(material),
        ));
    }
}
