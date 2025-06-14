// Re-export items from the sys crate for convenience
pub use wxdragon_sys as ffi;

#[macro_use]
pub mod macros;
pub mod app;
pub mod art_provider;
pub mod bitmap;
pub mod bitmap_bundle;
pub mod clipboard;
pub mod color;
pub mod data_object;
pub mod datetime;
pub mod dc;
pub mod dialogs;
pub mod dnd;
pub mod event;
pub mod font;
pub mod font_data;
pub mod geometry;
pub mod id;
pub mod menus;
pub mod prelude;
pub mod sizers;
pub mod timer;
pub mod types;
pub mod utils;
pub mod widgets;
pub mod window;
pub mod xrc;

// Re-export the prelude for convenience - users should use this
pub use prelude::*;

// Re-export procedural macros from wxdragon-macros
pub use wxdragon_macros::include_xrc;

// Re-export XRC macros (these are macro_rules! macros, not procedural macros)
// include_xrc_dialog!, include_xrc_panel! are available via #[macro_use]
