use crate::{Theme, icon, motion, theme::row};
use gpui::*;

use gpui_base::{SwitchThumb, SwitchTrack};

struct Presentation {
    id: ElementId,
    value: CheckboxState,
    theme: Option<Theme>,
    hover_style: Option<StyleRefinement>,
    focus_style: Option<StyleRefinement>,
}

macro_rules! control {
    ($name:ident, $state:ident, $value:ty, $convert:expr, $style:ident) => {
        #[derive(IntoElement)]
        pub struct $name(gpui_base::$name, Option<Presentation>, Vec<AnyElement>);

        element!(@trait $name, Styled);
        element!(@trait $name, Interactive);

        impl ParentElement for $name {
            fn extend(&mut self, children: impl IntoIterator<Item = AnyElement>) {
                self.2.extend(children);
            }
        }

        impl $name {
            pub fn new(id: impl Into<ElementId>) -> Self {
                let id = id.into();
                Self(gpui_base::$name::new(id.clone()), Some(Presentation {
                    id,
                    value: CheckboxState::Unchecked,
                    theme: None,
                    hover_style: None,
                    focus_style: None,

                }), Vec::new())
            }

            /// Creates a control whose presentation is supplied by the caller.
            pub fn unstyled(id: impl Into<ElementId>) -> Self {
                Self(gpui_base::$name::new(id), None, Vec::new())
            }

            pub fn with_theme(mut self, theme: Theme) -> Self {
                if let Some(presentation) = &mut self.1 {
                    presentation.theme = Some(theme);
                }
                self
            }

            pub fn $state(mut self, value: bool) -> Self {
                if let Some(presentation) = &mut self.1 {
                    presentation.value = if value { CheckboxState::Checked } else { CheckboxState::Unchecked };
                }
                self.0 = self.0.$state(value);
                self
            }

            builders! {
                disabled(disabled: bool);
                accessibility_label(label: impl Into<SharedString>);
                track_focus(focus: &FocusHandle);
                tab_index(index: isize);
                tab_stop(enabled: bool);
            }

            pub fn on_change(
                mut self,
                handler: impl Fn($value, &ClickEvent, &mut Window, &mut App) + 'static,
            ) -> Self {
                self.0 = self.0.on_change(move |value, event, window, cx| {
                    handler(($convert)(value), event, window, cx);
                });
                self
            }
        }

        impl RenderOnce for $name {
            fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
                let Self(mut control, presentation, children) = self;
                if let Some(presentation) = presentation {
                    let t = presentation.theme.unwrap_or_else(|| Theme::global(cx));
                    let overrides = std::mem::take(control.style());
                    control = $style(control, t, presentation, window, cx);
                    control.style().refine(&overrides);
                }
                control.children(children)
            }
        }
    };
}

control!(
    Checkbox,
    checked,
    CheckboxState,
    CheckboxState::from_backend,
    style_checkbox
);
control!(Switch, checked, bool, std::convert::identity, style_switch);
control!(Toggle, pressed, bool, std::convert::identity, style_toggle);
control!(Radio, checked, bool, std::convert::identity, style_radio);

/// The controlled value of a checkbox.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CheckboxState {
    #[default]
    Unchecked,
    Checked,
    Indeterminate,
}

impl CheckboxState {
    fn from_backend(value: gpui_base::CheckboxState) -> Self {
        match value {
            gpui_base::CheckboxState::Unchecked => Self::Unchecked,
            gpui_base::CheckboxState::Checked => Self::Checked,
            gpui_base::CheckboxState::Indeterminate => Self::Indeterminate,
        }
    }
}

impl Checkbox {
    pub fn state(mut self, value: CheckboxState) -> Self {
        if let Some(presentation) = &mut self.1 {
            presentation.value = value;
        }
        self.0 = self.0.state(match value {
            CheckboxState::Unchecked => gpui_base::CheckboxState::Unchecked,
            CheckboxState::Checked => gpui_base::CheckboxState::Checked,
            CheckboxState::Indeterminate => gpui_base::CheckboxState::Indeterminate,
        });
        self
    }

    pub fn indeterminate(mut self, indeterminate: bool) -> Self {
        if let Some(presentation) = &mut self.1 {
            if indeterminate {
                presentation.value = CheckboxState::Indeterminate;
            } else if presentation.value == CheckboxState::Indeterminate {
                presentation.value = CheckboxState::Unchecked;
            }
        }
        self.0 = self.0.indeterminate(indeterminate);
        self
    }

    builders! {
        role(role: Role);
    }
}

impl Toggle {
    pub fn hover(mut self, style: impl FnOnce(StyleRefinement) -> StyleRefinement) -> Self {
        if let Some(presentation) = &mut self.1 {
            presentation.hover_style = Some(style(StyleRefinement::default()));
        } else {
            self.0 = self.0.hover(style);
        }
        self
    }

    pub fn focus_visible(mut self, style: impl FnOnce(StyleRefinement) -> StyleRefinement) -> Self {
        if let Some(presentation) = &mut self.1 {
            presentation.focus_style = Some(style(StyleRefinement::default()));
        } else {
            self.0 = self.0.focus_visible(style);
        }
        self
    }
}

impl Radio {
    builders! {
        set_position(position: usize, size: usize);
    }
}

fn style_checkbox(
    control: gpui_base::Checkbox,
    t: Theme,
    presentation: Presentation,
    window: &mut Window,
    cx: &mut App,
) -> gpui_base::Checkbox {
    let checked = presentation.value != CheckboxState::Unchecked;
    let progress = motion::glide(
        (presentation.id, "checkbox"),
        f32::from(checked),
        t.motion.normal,
        window,
        cx,
    );
    control
        .styles(|s| s.disabled(|s| s.opacity(t.states.disabled).cursor_default()))
        .flex()
        .items_start()
        .gap(t.unit(8.))
        .cursor_pointer()
        .child(
            row()
                .size(t.unit(16.))
                .mt(px(2.))
                .justify_center()
                .rounded(t.checkbox_radius())
                .border(t.borders.width)
                .border_color(if checked { t.primary } else { t.input })
                .bg(t.primary.opacity(progress))
                .child(
                    icon(
                        if presentation.value == CheckboxState::Indeterminate {
                            "minus"
                        } else {
                            "check"
                        },
                        t.unit(12.),
                        t.primary_foreground,
                    )
                    .opacity(progress)
                    .with_transformation(Transformation::scale(size(
                        0.65 + 0.35 * progress,
                        0.65 + 0.35 * progress,
                    ))),
                ),
        )
}

fn style_switch(
    control: gpui_base::Switch,
    t: Theme,
    presentation: Presentation,
    window: &mut Window,
    cx: &mut App,
) -> gpui_base::Switch {
    let on = presentation.value == CheckboxState::Checked;
    let progress = motion::glide(
        (presentation.id, "switch"),
        f32::from(on),
        t.motion.normal,
        window,
        cx,
    );
    control
        .styles(|s| s.disabled(|s| s.opacity(t.states.disabled).cursor_default()))
        .flex()
        .items_center()
        .gap(t.unit(8.))
        .cursor_pointer()
        .child(
            SwitchTrack::new("track")
                .checked(on)
                .flex()
                .items_center()
                .w(t.unit(36.))
                .h(t.unit(20.))
                .p(t.unit(2.))
                .rounded_full()
                .bg(t.input.blend(t.primary.opacity(progress)))
                .child(
                    SwitchThumb::new(on)
                        .relative()
                        .left(t.unit(16. * progress))
                        .size(t.unit(16.))
                        .rounded_full()
                        .bg(if on {
                            t.primary_foreground
                        } else {
                            t.background
                        }),
                ),
        )
}

fn style_toggle(
    control: gpui_base::Toggle,
    t: Theme,
    presentation: Presentation,
    window: &mut Window,
    cx: &mut App,
) -> gpui_base::Toggle {
    let pressed = presentation.value == CheckboxState::Checked;
    let selected = motion::glide(
        (presentation.id, "toggle"),
        f32::from(pressed),
        t.motion.normal,
        window,
        cx,
    );
    control
        .styles(|s| s.disabled(|s| s.opacity(t.states.disabled).cursor_default()))
        .flex()
        .items_center()
        .justify_center()
        .h(t.unit(36.))
        .px(t.unit(12.))
        .rounded(t.radius_control())
        .border(t.borders.width)
        .border_color(t.input)
        .cursor_pointer()
        .hover(move |s| {
            let mut s = s.bg(t.accent).text_color(t.accent_foreground);
            if let Some(style) = presentation.hover_style {
                s.refine(&style);
            }
            s
        })
        .focus_visible(move |s| {
            let mut s = s.border_color(t.ring).shadow(t.focus_shadow(t.ring));
            if let Some(style) = presentation.focus_style {
                s.refine(&style);
            }
            s
        })
        .bg(t.muted.opacity(selected))
}

fn style_radio(
    control: gpui_base::Radio,
    t: Theme,
    presentation: Presentation,
    window: &mut Window,
    cx: &mut App,
) -> gpui_base::Radio {
    let checked = presentation.value == CheckboxState::Checked;
    let progress = motion::glide(
        (presentation.id, "radio"),
        f32::from(checked),
        t.motion.normal,
        window,
        cx,
    );
    control
        .styles(|s| s.disabled(|s| s.opacity(t.states.disabled).cursor_default()))
        .flex()
        .items_center()
        .gap(t.unit(8.))
        .cursor_pointer()
        .child(
            row()
                .size(t.unit(16.))
                .justify_center()
                .rounded_full()
                .border(t.borders.width)
                .border_color(if checked { t.primary } else { t.input })
                .child(
                    div()
                        .size(t.unit(8.))
                        .rounded_full()
                        .bg(t.primary)
                        .opacity(progress),
                ),
        )
}

pub(crate) fn input_frame(t: Theme, id: impl Into<ElementId>) -> Stateful<Div> {
    div()
        .id(id)
        .w_full()
        .flex()
        .flex_col()
        .justify_center()
        .h(t.unit(32.))
        .min_h(t.unit(32.))
        .px(t.unit(12.))
        .line_height(t.unit(t.typography.sm.line_height))
        .border(t.borders.width)
        .border_color(t.input)
        .rounded(t.input_radius())
}
