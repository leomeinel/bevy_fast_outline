/*
 * File: lib.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2026 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

//! Simple 2D pixel art outline for Bevy focused on performance over features.

mod outline;
mod plugin;

pub mod prelude {
    pub use crate::outline::Outlined2dTexture;
    pub use crate::plugin::FastOutlinePlugin;
}
