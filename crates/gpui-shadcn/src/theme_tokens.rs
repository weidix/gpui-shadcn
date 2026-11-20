use gpui::{BoxShadow, FontWeight, Pixels, black, px};

#[derive(Clone, Copy, PartialEq)]
pub struct TextStyle {
    pub size: f32,
    pub line_height: f32,
    pub weight: FontWeight,
}

impl TextStyle {
    pub const fn new(size: f32, line_height: f32, weight: FontWeight) -> Self {
        Self {
            size,
            line_height,
            weight,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Typography {
    pub sans: Option<&'static str>,
    pub mono: &'static str,
    pub caption: TextStyle,
    pub xs: TextStyle,
    pub compact: TextStyle,
    pub sm: TextStyle,
    pub md: TextStyle,
    pub lg: TextStyle,
    pub heading: TextStyle,
    pub xl: TextStyle,
    pub display: TextStyle,
    pub code: TextStyle,
}

impl Default for Typography {
    fn default() -> Self {
        let normal = FontWeight::NORMAL;
        let medium = FontWeight::MEDIUM;
        let semibold = FontWeight::SEMIBOLD;
        Self {
            sans: None,
            mono: if cfg!(target_os = "macos") {
                "Menlo"
            } else {
                "monospace"
            },
            caption: TextStyle::new(11., 16., normal),
            compact: TextStyle::new(13., 18., normal),
            xs: TextStyle::new(12., 16., normal),
            sm: TextStyle::new(14., 20., medium),
            md: TextStyle::new(16., 22., normal),
            lg: TextStyle::new(18., 24., semibold),
            heading: TextStyle::new(20., 28., semibold),
            xl: TextStyle::new(24., 30., semibold),
            display: TextStyle::new(36., 42., FontWeight::BOLD),
            code: TextStyle::new(13., 22., normal),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Shadows {
    pub sm: [Shadow; 2],
    pub md: [Shadow; 2],
    pub lg: [Shadow; 2],
    pub popup: [Shadow; 2],
}

#[derive(Clone, Copy, PartialEq)]
pub struct Shadow {
    pub offset_x: Pixels,
    pub offset_y: Pixels,
    pub blur: Pixels,
    pub spread: Pixels,
    pub color: gpui::Hsla,
}

impl Shadow {
    pub fn paint(self) -> BoxShadow {
        BoxShadow::new(self.offset_x, self.offset_y, self.color)
            .blur_radius(self.blur)
            .spread_radius(self.spread)
    }
}

impl Default for Shadows {
    fn default() -> Self {
        let layer = |y, blur, spread| Shadow {
            offset_x: px(0.),
            offset_y: px(y),
            blur: px(blur),
            spread: px(spread),
            color: black().opacity(0.1),
        };
        Self {
            sm: [layer(1., 3., 0.), layer(1., 2., -1.)],
            md: [layer(4., 6., -1.), layer(2., 4., -2.)],
            lg: [layer(10., 15., -3.), layer(4., 6., -4.)],
            // GPUI takes Gaussian sigma, half the CSS shadow blur radius.
            popup: [layer(4., 3., -1.), layer(2., 2., -2.)],
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Easing {
    Linear,
    EaseOutCubic,
}

impl Easing {
    pub fn sample(self, progress: f32) -> f32 {
        match self {
            Self::Linear => progress.clamp(0., 1.),
            Self::EaseOutCubic => crate::motion::ease_out_cubic(progress),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Motion {
    pub fast: u64,
    pub normal: u64,
    pub slow: u64,
    pub easing: Easing,
    pub spinner: u64,
    pub pulse: u64,
    pub drawer: u64,
}

impl Default for Motion {
    fn default() -> Self {
        Self {
            fast: 100,
            normal: 160,
            slow: 220,
            easing: Easing::EaseOutCubic,
            spinner: 800,
            pulse: 1600,
            drawer: 400,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct StateOpacity {
    pub primary_hover: f32,
    pub secondary_hover: f32,
    pub destructive_hover: f32,
    pub active: f32,
    pub disabled: f32,
    pub focus_ring: f32,
    pub invalid_ring: f32,
    pub popup_border: f32,
    pub inset_input: f32,
}

impl Default for StateOpacity {
    fn default() -> Self {
        Self {
            primary_hover: 0.87,
            secondary_hover: 0.75,
            destructive_hover: 0.85,
            active: 0.09,
            disabled: 0.5,
            focus_ring: 0.5,
            invalid_ring: 0.25,
            popup_border: 0.1,
            inset_input: 0.3,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Borders {
    pub width: Pixels,
    pub strong: Pixels,
    pub focus_width: Pixels,
    pub focus_offset: Pixels,
}

impl Default for Borders {
    fn default() -> Self {
        Self {
            width: px(1.),
            strong: px(2.),
            focus_width: px(2.),
            focus_offset: px(2.),
        }
    }
}
