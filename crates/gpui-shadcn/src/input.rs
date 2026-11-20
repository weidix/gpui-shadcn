//! Single-line and multiline text editing.

use gpui::*;

use crate::Theme;

/// Events emitted by text inputs and textareas.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputEvent {
    Change,
    PressEnter { secondary: bool, shift: bool },
    Focus,
    Blur,
}

impl InputEvent {
    fn from_backend(event: &gpui_base::input::InputEvent) -> Self {
        use gpui_base::input::InputEvent as Event;

        match *event {
            Event::Change => Self::Change,
            Event::PressEnter { secondary, shift } => Self::PressEnter { secondary, shift },
            Event::Focus => Self::Focus,
            Event::Blur => Self::Blur,
        }
    }
}

fn editor_style(theme: Theme) -> gpui_base::input::InputEditorStyle {
    // The editor uses alpha zero as its unset marker. Keep explicitly
    // transparent Shadcn colors invisible and out of that fallback path.
    let explicit = |mut color: Hsla| {
        if color.a == 0. {
            color.a = f32::MIN_POSITIVE;
        }
        color
    };
    gpui_base::input::InputEditorStyle {
        foreground: explicit(theme.foreground),
        muted_foreground: explicit(theme.muted_foreground),
        caret: explicit(theme.foreground),
        selection: explicit(theme.selection),
        background: explicit(theme.background),
        border: explicit(theme.border),
        ..Default::default()
    }
}

fn editor_scrollbar(theme: Theme) -> gpui_base::input::EditorScrollbarRenderer {
    use gpui_base::{Scrollbar, ScrollbarMode};

    std::rc::Rc::new(move |context, _, _| {
        let normal = theme.foreground.opacity(0.35);
        let active = theme.foreground.opacity(0.55);
        let clear = transparent_black();
        Scrollbar::new(&context)
            .axis(context.axis())
            // The overflow thumb is always available. Fixed geometry also keeps
            // the host's scrollbar expansion animation out of the presentation.
            .mode(ScrollbarMode::Always)
            .styles(|styles| {
                styles
                    .track(|track| track.width(px(16.)).bg(clear).border_color(clear))
                    .thumb(|thumb| {
                        thumb
                            .bg(normal)
                            .width(px(6.))
                            .inset(px(4.))
                            .radius(px(0.))
                            .min_length(px(48.))
                    })
                    .thumb_hover(|thumb| thumb.bg(active))
                    .thumb_active(|thumb| thumb.bg(active))
            })
            .into_any_element()
    })
}

macro_rules! text_control {
    ($name:ident, $state:ident, $description:literal) => {
        #[doc = concat!("Editing state for ", $description, ".")]
        pub struct $state {
            editor: Entity<gpui_base::input::$state>,
            theme: Option<Theme>,
            _subscriptions: [Subscription; 2],
        }

        impl $state {
            pub fn new(
                window: &mut Window,
                cx: &mut Context<Self>,
                placeholder: impl Into<SharedString>,
            ) -> Self {
                let editor =
                    cx.new(|cx| gpui_base::input::$state::new(window, cx).placeholder(placeholder));
                let changes = cx.observe(&editor, |_, _, cx| cx.notify());
                let events = cx.subscribe(&editor, |_, _, event, cx| {
                    let event = InputEvent::from_backend(event);
                    if event == InputEvent::Focus {
                        cx.notify();
                    }
                    cx.emit(event);
                });
                Self {
                    editor,
                    theme: None,
                    _subscriptions: [changes, events],
                }
            }

            /// Returns the current text, including edits which have not been submitted.
            pub fn value(&self, cx: &App) -> SharedString {
                self.editor.read(cx).value()
            }

            pub fn is_editable(&self, cx: &App) -> bool {
                self.editor.read(cx).is_editable()
            }

            pub fn set_value(
                &mut self,
                value: impl Into<SharedString>,
                window: &mut Window,
                cx: &mut Context<Self>,
            ) {
                self.editor
                    .update(cx, |state, cx| state.set_value(value.into(), window, cx));
            }

            pub fn set_disabled(&mut self, disabled: bool, cx: &mut Context<Self>) {
                self.editor
                    .update(cx, |state, cx| state.set_disabled(disabled, cx));
            }

            pub fn set_text_align(&mut self, align: TextAlign, cx: &mut Context<Self>) {
                self.editor
                    .update(cx, |state, cx| state.set_text_align(align, cx));
            }

            fn apply_theme(&mut self, theme: Theme, cx: &mut Context<Self>) {
                if self.theme != Some(theme) {
                    self.theme = Some(theme);
                    self.editor.update(cx, |state, cx| {
                        state.set_editor_style(editor_style(theme));
                        if state.is_multi_line() {
                            state.set_scrollbar_renderer(Some(editor_scrollbar(theme)));
                        }
                        cx.notify();
                    });
                }
            }
        }

        impl EventEmitter<InputEvent> for $state {}

        impl Focusable for $state {
            fn focus_handle(&self, cx: &App) -> FocusHandle {
                self.editor.read(cx).focus_handle(cx)
            }
        }

        #[doc = concat!("Themed editor for ", $description, ".")]
        #[derive(IntoElement)]
        pub struct $name {
            state: Entity<$state>,
            theme: Option<Theme>,
        }

        impl $name {
            pub fn new(state: &Entity<$state>) -> Self {
                Self {
                    state: state.clone(),
                    theme: None,
                }
            }

            /// Overrides the application theme for this editor.
            pub fn with_theme(mut self, theme: Theme) -> Self {
                self.theme = Some(theme);
                self
            }
        }

        impl RenderOnce for $name {
            fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
                let theme = self.theme.unwrap_or_else(|| Theme::global(cx));
                let editor = self.state.update(cx, |state, cx| {
                    state.apply_theme(theme, cx);
                    state.editor.clone()
                });
                gpui_base::input::$name::new(&editor)
            }
        }
    };
}

text_control!(Input, InputState, "single-line text");
text_control!(Textarea, TextareaState, "multiline text");

impl TextareaState {
    pub fn set_rows(&mut self, rows: usize, cx: &mut Context<Self>) {
        self.editor.update(cx, |state, cx| state.set_rows(rows, cx));
    }
}

impl Input {
    /// Styles the frame used to compose a text input, textarea, or input group.
    pub fn frame(id: impl Into<ElementId>, cx: &App) -> Stateful<Div> {
        crate::controls::input_frame(Theme::global(cx), id)
    }

    /// A search field inset into a menu or command surface.
    pub fn search(id: impl Into<ElementId>, state: &Entity<InputState>, cx: &App) -> Stateful<Div> {
        let t = Theme::global(cx);
        let focus_state = state.clone();
        crate::theme::row()
            .id(id)
            .w_full()
            .h(t.unit(32.))
            .flex_shrink_0()
            .gap(t.unit(6.))
            .px(t.unit(8.) + px(1.))
            .line_height(t.unit(t.typography.sm.line_height))
            .bg(t.background.blend(t.input.opacity(t.states.inset_input)))
            .rounded(t.input_radius())
            .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                focus_state.read(cx).focus_handle(cx).focus(window, cx);
            })
            .child(crate::icon("search", t.unit(16.), t.muted_foreground))
            .child(div().flex_1().min_w_0().h_full().child(Self::new(state)))
    }

    /// Creates a single-line field with a themed border and focus ring.
    pub fn field(
        id: impl Into<ElementId>,
        state: &Entity<InputState>,
        invalid: bool,
        window: &Window,
        cx: &App,
    ) -> Stateful<Div> {
        field_frame(id, state, invalid, window, cx).child(Self::new(state))
    }
}

impl Textarea {
    /// Creates a multiline field sharing the input border and focus treatment.
    pub fn field(
        id: impl Into<ElementId>,
        state: &Entity<TextareaState>,
        invalid: bool,
        window: &Window,
        cx: &App,
    ) -> Stateful<Div> {
        let t = Theme::global(cx);
        let editor = state.read(cx);
        let focused = editor.is_editable(cx) && editor.focus_handle(cx).is_focused(window);
        styled_field_frame(t, id, invalid, focused)
            .h_auto()
            .min_h(t.unit(80.))
            .py(t.unit(6.))
            .justify_start()
            .child(Self::new(state))
    }
}

/// Creates the themed field frame for layouts with adornments or grouped inputs.
pub fn field_frame(
    id: impl Into<ElementId>,
    state: &Entity<InputState>,
    invalid: bool,
    window: &Window,
    cx: &App,
) -> Stateful<Div> {
    let state = state.read(cx);
    let focused = state.is_editable(cx) && state.focus_handle(cx).is_focused(window);
    styled_field_frame(Theme::global(cx), id, invalid, focused)
}

fn styled_field_frame(
    t: Theme,
    id: impl Into<ElementId>,
    invalid: bool,
    focused: bool,
) -> Stateful<Div> {
    use gpui::prelude::FluentBuilder as _;

    let border = if invalid {
        t.destructive
    } else if focused {
        t.ring
    } else {
        t.input
    };
    crate::controls::input_frame(t, id)
        .min_w_0()
        .border_color(border)
        .when(t.dark || focused, |frame| {
            frame.bg(if t.dark {
                t.background.blend(t.input.opacity(t.states.inset_input))
            } else {
                t.background
            })
        })
        .when(focused, |frame| {
            let opacity = if invalid {
                t.states.invalid_ring
            } else {
                t.states.focus_ring
            };
            frame.shadow(t.focus_shadow(border.opacity(opacity)))
        })
}

#[cfg(test)]
mod tests {
    use gpui::transparent_black;

    use super::{Theme, editor_style};
    use crate::Density;

    #[test]
    fn editor_colors_do_not_resolve_from_host_tokens() {
        let theme = Theme {
            foreground: transparent_black(),
            selection: transparent_black(),
            ..Theme::new(true, Density::Default, Theme::DEFAULT_RADIUS)
        };
        let host = gpui_base::SemanticThemeTokens::default();
        let projected = editor_style(theme);
        let resolved = projected.resolved(&host);

        assert_eq!(resolved.foreground, projected.foreground);
        assert_eq!(resolved.muted_foreground, theme.muted_foreground);
        assert_eq!(resolved.selection, projected.selection);
        assert_eq!(resolved.caret, projected.caret);
        assert_eq!(resolved.border, theme.border);
        assert_eq!(resolved.background, theme.background);
    }
}
