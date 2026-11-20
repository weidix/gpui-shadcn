use gpui::*;

pub use crate::theme_tokens::{
    Borders, Easing, Motion, Shadow, Shadows, StateOpacity, TextStyle, Typography,
};

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum Density {
    Compact,
    #[default]
    Default,
    Comfortable,
}

impl Density {
    pub const ALL: [(Self, &'static str); 3] = [
        (Self::Compact, "Compact"),
        (Self::Default, "Default"),
        (Self::Comfortable, "Comfortable"),
    ];

    pub fn scale(self) -> f32 {
        match self {
            Self::Compact => 0.875,
            Self::Default => 1.,
            Self::Comfortable => 1.125,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Theme {
    pub background: Hsla,
    pub foreground: Hsla,
    pub muted: Hsla,
    pub muted_foreground: Hsla,
    pub sidebar: Hsla,
    pub border: Hsla,
    pub input: Hsla,
    pub primary: Hsla,
    pub primary_foreground: Hsla,
    pub destructive: Hsla,
    pub ring: Hsla,
    pub card: Hsla,
    pub card_foreground: Hsla,
    pub popover: Hsla,
    pub popover_foreground: Hsla,
    pub secondary: Hsla,
    pub secondary_foreground: Hsla,
    pub accent: Hsla,
    pub accent_foreground: Hsla,
    pub destructive_foreground: Hsla,
    pub sidebar_foreground: Hsla,
    pub sidebar_accent: Hsla,
    pub sidebar_accent_foreground: Hsla,
    pub sidebar_border: Hsla,
    pub sidebar_ring: Hsla,
    pub sidebar_primary: Hsla,
    pub sidebar_primary_foreground: Hsla,
    pub selection: Hsla,
    pub overlay: Hsla,
    pub overlay_subtle: Hsla,
    pub typography: Typography,
    pub shadows: Shadows,
    pub motion: Motion,
    pub states: StateOpacity,
    pub borders: Borders,
    pub dark: bool,
    pub density: Density,
    pub radius: f32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Variant {
    Primary,
    Secondary,
    Outline,
    Ghost,
    Destructive,
    Link,
}

impl Global for Theme {}

impl Default for Theme {
    fn default() -> Self {
        Self::new(false, Density::Default, Self::DEFAULT_RADIUS)
    }
}

impl Theme {
    pub const DEFAULT_RADIUS: f32 = 14.;

    pub fn new(dark: bool, density: Density, radius: f32) -> Self {
        let color = |light, dark_value| Hsla::from(rgb(if dark { dark_value } else { light }));
        let foreground = color(0x0a0a0a, 0xfafafa);
        let muted = color(0xf5f5f5, 0x262626);
        let border = color(0xe5e5e5, 0x262626);
        let ring = color(0xa3a3a3, 0x737373);
        let primary = color(0x171717, 0xe5e5e5);
        let primary_foreground = color(0xfafafa, 0x171717);
        Self {
            background: color(0xffffff, 0x0a0a0a),
            foreground,
            muted,
            muted_foreground: color(0x737373, 0xa3a3a3),
            sidebar: color(0xfafafa, 0x171717),
            border,
            input: color(0xe5e5e5, 0x373737),
            primary,
            primary_foreground,
            destructive: color(0xe7000b, 0xff6467),
            ring,
            card: color(0xffffff, 0x0a0a0a),
            card_foreground: foreground,
            popover: color(0xffffff, 0x0a0a0a),
            popover_foreground: foreground,
            secondary: muted,
            secondary_foreground: foreground,
            accent: muted,
            accent_foreground: foreground,
            destructive_foreground: color(0xffffff, 0x0a0a0a),
            sidebar_foreground: foreground,
            sidebar_accent: muted,
            sidebar_accent_foreground: foreground,
            sidebar_border: border,
            sidebar_ring: ring,
            sidebar_primary: primary,
            sidebar_primary_foreground: primary_foreground,
            selection: foreground.opacity(if dark { 0.32 } else { 0.26 }),
            overlay: black().opacity(0.5),
            overlay_subtle: black().opacity(0.3),
            typography: Typography::default(),
            shadows: Shadows::default(),
            motion: Motion::default(),
            states: StateOpacity::default(),
            borders: Borders::default(),
            dark,
            density,
            radius,
        }
    }

    /// Selects the built-in light or dark palette, retaining density and radius.
    pub fn with_dark(self, dark: bool) -> Self {
        if dark == self.dark {
            return self;
        }
        Self {
            typography: self.typography,
            shadows: self.shadows,
            motion: self.motion,
            states: self.states,
            borders: self.borders,
            overlay: self.overlay,
            overlay_subtle: self.overlay_subtle,
            ..Self::new(dark, self.density, self.radius)
        }
    }

    /// Returns the active Shadcn theme, or the default light theme.
    pub fn global(cx: &App) -> Self {
        cx.try_global::<Self>().copied().unwrap_or_default()
    }

    /// Installs the Shadcn theme and redraws the application's windows.
    pub fn set(self, cx: &mut App) {
        cx.set_global(self);
        cx.refresh_windows();
    }

    pub fn unit(self, size: f32) -> Pixels {
        px(size * self.density.scale())
    }

    fn radius_offset(self, offset: f32) -> Pixels {
        px((self.radius - offset).max(0.))
    }

    pub fn radius(self) -> Pixels {
        self.radius_offset(0.)
    }

    pub fn radius_small(self) -> Pixels {
        self.radius_offset(8.)
    }

    pub fn checkbox_radius(self) -> Pixels {
        self.unit(4.)
    }

    pub fn radius_control(self) -> Pixels {
        self.radius_offset(4.)
    }

    pub fn input_radius(self) -> Pixels {
        self.radius_control()
    }

    pub fn radius_surface(self) -> Pixels {
        self.radius()
    }

    pub fn surface(self) -> Div {
        div()
            .bg(self.background)
            .border(self.borders.width)
            .border_color(self.border)
            .rounded(self.radius_surface())
    }

    pub fn popup_radius(self) -> Pixels {
        self.radius_control()
    }

    pub fn popup_item_radius(self) -> Pixels {
        self.radius_small()
    }

    pub fn popup_surface(self) -> Div {
        self.surface()
            .bg(self.popover)
            .text_color(self.popover_foreground)
            .rounded(self.popup_radius())
            .border_color(self.popover_foreground.opacity(self.states.popup_border))
            .shadow(self.shadows.popup.map(Shadow::paint).to_vec())
    }

    /// Styles a text role at the active density.
    pub fn text<T: Styled>(self, element: T, style: TextStyle) -> T {
        let element = if let Some(family) = self.typography.sans {
            element.font_family(family)
        } else {
            element
        };
        element
            .text_size(self.unit(style.size))
            .line_height(self.unit(style.line_height))
            .font_weight(style.weight)
    }

    /// Two layers make the gap between the edge and the focus ring visible.
    pub fn focus_shadow(self, color: Hsla) -> Vec<BoxShadow> {
        vec![
            BoxShadow::new(px(0.), px(0.), self.background)
                .spread_radius(self.borders.focus_offset),
            BoxShadow::new(px(0.), px(0.), color)
                .spread_radius(self.borders.focus_offset + self.borders.focus_width),
        ]
    }
}

pub fn column() -> Div {
    div().flex().flex_col()
}

pub fn row() -> Div {
    div().flex().items_center()
}

#[cfg(test)]
mod tests {
    use gpui::{Hsla, TestAppContext, rgb};

    use super::{Density, Theme};

    #[gpui::test]
    fn installing_a_shadcn_theme_leaves_the_host_palette_untouched(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let host_foreground = Hsla::from(rgb(0xff00ff));
            gpui_base::Theme::global_mut(cx).tokens.colors.foreground = host_foreground;

            let dark = Theme::new(true, Density::Compact, 8.);
            dark.set(cx);
            crate::init(cx);
            assert!(Theme::global(cx) == dark);
            assert_eq!(
                gpui_base::Theme::global(cx).tokens.colors.foreground,
                host_foreground
            );

            Theme::default().set(cx);
            assert!(!Theme::global(cx).dark);
            assert_eq!(
                gpui_base::Theme::global(cx).tokens.colors.foreground,
                host_foreground
            );
        });
    }
}
