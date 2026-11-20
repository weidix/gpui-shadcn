//! Shadcn-style components for GPUI applications.
//!
//! Call [`init`] at startup and use [`root`] when creating a window.
//! The default theme is light. [`Theme::set`] changes the active theme
//! and schedules a redraw of all application windows.

use gpui::AppContext;

#[macro_use]
mod element;
mod button;
pub mod calendar;
mod code_block;
mod controls;
pub mod display;
pub mod form;
mod icons;
pub mod input;
pub mod motion;
pub mod overlay;
pub mod scroll;
mod select;
pub mod theme;
mod theme_tokens;
mod tooltip;

pub use button::Button;
pub use code_block::CodeBlock;
pub use controls::{Checkbox, CheckboxState, Radio, Switch, Toggle};
pub use gpui;
pub use input::{Input, InputEvent, InputState, Textarea, TextareaState};

/// Wraps application content in the window host used by overlays and text selection.
pub fn root<V: Into<gpui::AnyView>>(
    view: V,
    window: &mut gpui::Window,
    cx: &mut gpui::App,
) -> gpui::Entity<impl gpui::Render + use<V>> {
    cx.new(|cx| gpui_base::Root::new(view, window, cx))
}

pub use icons::{icon, load_icon};
pub use select::Select;
pub use theme::{Density, Theme, Variant};

/// Registers the shared component actions and theme in the application.
pub fn init(cx: &mut gpui::App) {
    gpui_base::init(cx);
    if !cx.has_global::<Theme>() {
        cx.set_global(Theme::default());
    }
}

/// Registers the bundled Geist fonts when the application opts in.
#[cfg(feature = "geist")]
pub fn load_geist(cx: &gpui::App) -> gpui::Result<()> {
    use std::borrow::Cow;
    cx.text_system().add_fonts(vec![
        Cow::Borrowed(include_bytes!("../assets/Geist-Regular.ttf")),
        Cow::Borrowed(include_bytes!("../assets/Geist-Medium.ttf")),
        Cow::Borrowed(include_bytes!("../assets/Geist-SemiBold.ttf")),
        Cow::Borrowed(include_bytes!("../assets/Geist-Bold.ttf")),
    ])
}
