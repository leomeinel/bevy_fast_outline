# Bevy Fast Outline

[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/leomeinel/bevy_fast_outline)
[![Crates.io](https://img.shields.io/crates/v/bevy_fast_outline.svg)](https://crates.io/crates/bevy_fast_outline)
[![Downloads](https://img.shields.io/crates/d/bevy_fast_outline.svg)](https://crates.io/crates/bevy_fast_outline)
[![Docs](https://docs.rs/bevy_fast_outline/badge.svg)](https://docs.rs/bevy_fast_outline/latest/bevy_fast_outline/)

Simple 2D pixel art outline for Bevy focused on performance over features.

:warning: | This is not feature complete and currently only supports standard images.

## Features

- One pixel wide pixel-perfect outline for images.

## Limitations

- Currently this does not support sprites, only standard images.

## Usage

Take a look at [`/examples`](https://github.com/leomeinel/bevy_fast_outline/tree/main/examples) to find out how to use this crate.

### Examples

#### `mesh_outline.rs`

Scene with a green `Rectangle` as background and an `Outlined2dTexture` with a black outline.

## Alternatives

- https://github.com/theseatoad/bevy-simple-2d-outline

## Mixed license

This repository is not entirely licensed as Apache-2.0.

| Files              | Author(s)                                             | License                                                                        |
| ------------------ | ----------------------------------------------------- | ------------------------------------------------------------------------------ |
| `/assets/images/*` | [Leopold Meinel](https://github.com/leomeinel), Shave | [CC-BY-NC-SA-4.0](https://creativecommons.org/licenses/by-nc-sa/4.0/legalcode) |
