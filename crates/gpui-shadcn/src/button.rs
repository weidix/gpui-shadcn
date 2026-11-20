use gpui::*;

use crate::{Theme, Variant};

/// An accessible button styled from the current Shadcn theme at render time.
#[derive(IntoElement)]
pub struct Button(gpui_base::Button, Option<Presentation>, Vec<AnyElement>);

struct Presentation {
    variant: Variant,
    theme: Option<Theme>,
    icon: Option<(&'static str, f32)>,
    hover_background: Option<Hsla>,
    hover: Option<StyleRefinement>,
    active: Option<StyleRefinement>,
    focus_visible: Option<StyleRefinement>,
}

impl Button {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>, variant: Variant) -> Self {
        let label = label.into();
        Self::with_content(id, variant)
            .accessibility_label(label.clone())
            .child(label)
    }

    /// Creates a themed button whose contents are supplied by the caller.
    pub fn with_content(id: impl Into<ElementId>, variant: Variant) -> Self {
        Self(
            gpui_base::Button::new(id),
            Some(Presentation {
                variant,
                theme: None,
                icon: None,
                hover_background: None,
                hover: None,
                active: None,
                focus_visible: None,
            }),
            Vec::new(),
        )
    }

    pub fn icon(
        id: impl Into<ElementId>,
        label: impl Into<SharedString>,
        glyph: &'static str,
        variant: Variant,
        size: f32,
    ) -> Self {
        let mut button = Self::with_content(id, variant).accessibility_label(label);
        button.1.as_mut().unwrap().icon = Some((glyph, size));
        button
    }

    /// Creates a button whose presentation is supplied by the caller.
    pub fn unstyled(id: impl Into<ElementId>) -> Self {
        Self(gpui_base::Button::new(id), None, Vec::new())
    }

    pub fn with_theme(mut self, theme: Theme) -> Self {
        if let Some(presentation) = &mut self.1 {
            presentation.theme = Some(theme);
        }
        self
    }

    pub(crate) fn hover_background(mut self, color: Hsla) -> Self {
        if let Some(presentation) = &mut self.1 {
            presentation.hover_background = Some(color);
        }
        self
    }

    pub fn hover(mut self, style: impl FnOnce(StyleRefinement) -> StyleRefinement) -> Self {
        if let Some(presentation) = &mut self.1 {
            presentation.hover = Some(style(StyleRefinement::default()));
        } else {
            self.0 = self.0.hover(style);
        }
        self
    }

    pub fn active(mut self, style: impl FnOnce(StyleRefinement) -> StyleRefinement) -> Self {
        if let Some(presentation) = &mut self.1 {
            presentation.active = Some(style(StyleRefinement::default()));
        } else {
            self.0 = self.0.active(style);
        }
        self
    }

    pub fn focus_visible(mut self, style: impl FnOnce(StyleRefinement) -> StyleRefinement) -> Self {
        if let Some(presentation) = &mut self.1 {
            presentation.focus_visible = Some(style(StyleRefinement::default()));
        } else {
            self.0 = self.0.focus_visible(style);
        }
        self
    }

    pub(crate) fn is_selected(&self) -> bool {
        gpui_base::Selectable::is_selected(&self.0)
    }

    builders! {
        disabled(disabled: bool);
        selected(selected: bool);
        accessibility_label(label: impl Into<SharedString>);
        role(role: Role);
        track_focus(focus: &FocusHandle);
        tab_index(index: isize);
        tab_stop(enabled: bool);
        focusable(enabled: bool);
        on_click(handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static);
    }
}

element!(@trait Button, Styled);
element!(@trait Button, Interactive);

impl ParentElement for Button {
    fn extend(&mut self, children: impl IntoIterator<Item = AnyElement>) {
        self.2.extend(children);
    }
}

impl RenderOnce for Button {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        self.into_button(cx)
    }
}

impl Button {
    fn into_button(self, cx: &App) -> gpui_base::Button {
        let Self(mut button, presentation, children) = self;
        if let Some(p) = presentation {
            let t = p.theme.unwrap_or_else(|| Theme::global(cx));
            let foreground = match p.variant {
                Variant::Primary => t.primary_foreground,
                Variant::Secondary => t.secondary_foreground,
                Variant::Destructive => t.destructive_foreground,
                _ => t.foreground,
            };
            let (background, hover) = match p.variant {
                Variant::Primary => (t.primary, t.primary.opacity(t.states.primary_hover)),
                Variant::Secondary => (t.secondary, t.secondary.opacity(t.states.secondary_hover)),
                Variant::Outline | Variant::Ghost | Variant::Link => {
                    (transparent_black(), t.accent)
                }
                Variant::Destructive => (
                    t.destructive,
                    t.destructive.opacity(t.states.destructive_hover),
                ),
            };
            let overrides = std::mem::take(button.style());
            let border = if p.variant == Variant::Outline {
                t.input
            } else {
                transparent_black()
            };
            let link = p.variant == Variant::Link;
            button = t
                .text(button, t.typography.sm)
                .flex()
                .items_center()
                .justify_center()
                .gap(t.unit(8.))
                .h(t.unit(36.))
                .px(t.unit(14.))
                .rounded(t.radius_control())
                .border(t.borders.width)
                .border_color(border)
                .bg(background)
                .text_color(foreground)
                .cursor_pointer()
                .hover(move |s| {
                    let mut s = if link {
                        s.underline()
                    } else {
                        s.bg(p.hover_background.unwrap_or(hover)).text_color(
                            if matches!(p.variant, Variant::Outline | Variant::Ghost) {
                                t.accent_foreground
                            } else {
                                foreground
                            },
                        )
                    };
                    if let Some(override_style) = p.hover {
                        s.refine(&override_style);
                    }
                    s
                })
                .active(move |s| {
                    let mut s = if link {
                        s.underline()
                    } else {
                        s.bg(hover.blend(t.foreground.opacity(t.states.active)))
                    };
                    if let Some(override_style) = p.active {
                        s.refine(&override_style);
                    }
                    s
                })
                .focus_visible(move |s| {
                    let mut s = if link {
                        s.underline()
                    } else {
                        s.border_color(t.ring).shadow(t.focus_shadow(t.ring))
                    };
                    if let Some(override_style) = p.focus_visible {
                        s.refine(&override_style);
                    }
                    s
                })
                .styles(|s| s.disabled(|s| s.opacity(t.states.disabled).cursor_default()));
            if let Some((glyph, size)) = p.icon {
                button = button.size(t.unit(size)).p_0().child(crate::icon(
                    glyph,
                    t.unit(16.),
                    foreground,
                ));
            }
            button.style().refine(&overrides);
        }
        button.children(children)
    }
}

#[cfg(test)]
mod tests {
    use gpui::{Fill, Hsla, Styled, TestAppContext, rgb};

    use crate::{Button, Density, Theme, Variant};

    #[gpui::test]
    fn button_resolves_the_theme_when_rendered(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let fixed =
                Button::new("fixed", "Fixed", Variant::Primary).with_theme(Theme::default());
            for dark in [false, true] {
                for (density, _) in Density::ALL {
                    for radius in [0., 8., Theme::DEFAULT_RADIUS, 18.] {
                        let next = Button::new("changing", "Changing", Variant::Primary);
                        let mut theme = Theme::new(dark, density, radius);
                        theme.primary = Hsla::from(rgb(if dark { 0x123456 } else { 0xabcdef }));
                        theme.set(cx);
                        let mut rendered = next.into_button(cx);
                        assert_eq!(rendered.style().background, Some(Fill::from(theme.primary)));
                        assert_eq!(
                            rendered.style().text.font_size,
                            Some(theme.unit(theme.typography.sm.size).into())
                        );
                    }
                }
            }
            assert_eq!(
                fixed.into_button(cx).style().background,
                Some(Fill::from(Theme::default().primary))
            );
        });
    }
}
