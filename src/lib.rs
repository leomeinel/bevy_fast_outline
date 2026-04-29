//! Simple 2D pixel art outline for Bevy focused on performance over features.

mod outline;
mod plugin;

pub mod prelude {
    pub use crate::outline::Outlined2dTexture;
    pub use crate::plugin::FastOutlinePlugin;
}
