//! Form controls with their own editable state.

use gpui::*;

use crate::{Theme, theme::row};

element!(RadioGroup(gpui_base::RadioGroup) + Styled + Parent + Interactive);
constructor!(RadioGroup(id: impl Into<ElementId>));

impl RadioGroup {
    builders! {
        axis(axis: Axis);
    }
}

/// The current value and configuration of a single-thumb slider.
pub struct SliderState {
    slider: Entity<gpui_base::slider::SliderState>,
    _changes: Subscription,
}

impl SliderState {
    /// Creates a slider for finite, increasing bounds and a positive step.
    pub fn new(min: f32, max: f32, step: f32, initial_value: f32, cx: &mut Context<Self>) -> Self {
        assert!(
            min.is_finite() && max.is_finite() && min < max,
            "invalid slider bounds"
        );
        assert!(step.is_finite() && step > 0., "invalid slider step");
        let slider = cx.new(|_| {
            gpui_base::slider::SliderState::new()
                .max(max.max(100.))
                .min(min)
                .max(max)
                .step(step)
                .default_value(initial_value.clamp(min, max))
        });
        let changes = cx.observe(&slider, |_, _, cx| cx.notify());
        Self {
            slider,
            _changes: changes,
        }
    }

    pub fn value(&self, cx: &App) -> f32 {
        let state = self.slider.read(cx);
        state
            .value()
            .end()
            .clamp(state.min_value(), state.max_value())
    }

    pub fn set_value(&mut self, value: f32, window: &mut Window, cx: &mut Context<Self>) {
        self.slider.update(cx, |state, cx| {
            state.set_value(
                value.clamp(state.min_value(), state.max_value()),
                window,
                cx,
            );
        });
    }
}

/// A horizontal slider with pointer and keyboard input.
#[derive(IntoElement)]
pub struct Slider {
    id: ElementId,
    state: Entity<SliderState>,
    label: SharedString,
}

impl Slider {
    pub fn new(
        id: impl Into<ElementId>,
        state: &Entity<SliderState>,
        label: impl Into<SharedString>,
    ) -> Self {
        Self {
            id: id.into(),
            state: state.clone(),
            label: label.into(),
        }
    }
}

impl RenderOnce for Slider {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::global(cx);
        let state = self.state.read(cx);
        let value = state.value(cx);
        let state = state.slider.clone();
        let min = state.read(cx).min_value();
        let max = state.read(cx).max_value();
        let fraction = (value - min) / (max - min);
        let focus = window
            .use_keyed_state((self.id.clone(), "slider-focus"), cx, |_, cx| {
                cx.focus_handle().tab_stop(true)
            })
            .read(cx)
            .clone();
        let key_state = state.clone();
        let drag_state = state.clone();

        div()
            .id(self.id)
            .track_focus(&focus)
            .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                focus.focus(window, cx)
            })
            .on_key_down(move |event, window, cx| {
                let delta = match event.keystroke.key.as_str() {
                    "left" | "down" => -1.,
                    "right" | "up" => 1.,
                    _ => return,
                };
                cx.stop_propagation();
                key_state.update(cx, |s, cx| {
                    let next = (s.value().end() + delta * s.step_value())
                        .clamp(s.min_value(), s.max_value());
                    s.set_value(next, window, cx);
                });
            })
            .child(
                gpui_base::Slider::new(&state)
                    .accessibility_label(self.label)
                    .child(
                        gpui_base::SliderTrack::new(&state)
                            .flex()
                            .items_center()
                            .h(t.unit(20.))
                            .on_mouse_move(move |event, window, cx| {
                                if event.pressed_button == Some(MouseButton::Left) {
                                    drag_state.update(cx, |s, cx| {
                                        s.update_value_by_position(
                                            Axis::Horizontal,
                                            event.position,
                                            false,
                                            window,
                                            cx,
                                        );
                                    });
                                }
                            })
                            .cursor_pointer()
                            .child(
                                gpui_base::SliderIndicator::new(&state)
                                    .relative()
                                    .w_full()
                                    .h(t.unit(6.))
                                    .rounded_full()
                                    .bg(t.muted)
                                    .child(
                                        div()
                                            .w(relative(fraction))
                                            .h_full()
                                            .bg(t.primary)
                                            .rounded_full(),
                                    )
                                    .child(
                                        div()
                                            .absolute()
                                            .left(relative(fraction))
                                            .top(t.unit(-5.))
                                            .ml(t.unit(-8.))
                                            .size(t.unit(16.))
                                            .rounded_full()
                                            .border_1()
                                            .border_color(t.primary)
                                            .bg(t.background)
                                            .shadow(
                                                t.shadows
                                                    .sm
                                                    .map(crate::theme::Shadow::paint)
                                                    .to_vec(),
                                            ),
                                    ),
                            ),
                    ),
            )
    }
}

/// Editing state for a fixed-length numeric one-time code.
pub struct OtpState {
    editor: Entity<gpui_base::OtpState>,
    _changes: Subscription,
}

impl OtpState {
    pub fn new(length: usize, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let editor = cx.new(|cx| gpui_base::OtpState::new(length, window, cx));
        let changes = cx.observe(&editor, |_, _, cx| cx.notify());
        Self {
            editor,
            _changes: changes,
        }
    }

    pub fn value(&self, cx: &App) -> SharedString {
        self.editor.read(cx).value().clone()
    }

    pub fn set_value(
        &mut self,
        value: impl Into<SharedString>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.editor
            .update(cx, |state, cx| state.set_value(value, window, cx));
    }
}

impl Focusable for OtpState {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.editor.read(cx).focus_handle(cx)
    }
}

/// A one-time-code input with one slot per digit.
#[derive(IntoElement)]
pub struct OtpInput(Entity<OtpState>);

impl OtpInput {
    pub fn new(state: &Entity<OtpState>) -> Self {
        Self(state.clone())
    }
}

impl RenderOnce for OtpInput {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::global(cx);
        let state = self.0.read(cx);
        let focus = state.focus_handle(cx);
        let editor = state.editor.read(cx);
        let mut digits = editor.value().chars();
        gpui_base::OtpInput::new(&state.editor)
            .flex()
            .items_center()
            .gap(t.unit(8.))
            .children((0..editor.len()).map(|index| {
                let focus = focus.clone();
                row()
                    .id(("otp-slot", index))
                    .size(t.unit(38.))
                    .justify_center()
                    .border_1()
                    .border_color(t.input)
                    .rounded(t.radius_control())
                    .cursor_text()
                    .on_click(move |_, window, cx| focus.focus(window, cx))
                    .children(digits.next().map(|digit| digit.to_string()))
            }))
    }
}

/// A group of controlled toggles with a horizontal or vertical layout.
#[derive(IntoElement)]
pub struct ToggleGroup(gpui_base::ToggleGroup, Axis);

impl ToggleGroup {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self(gpui_base::ToggleGroup::new(id), Axis::Horizontal)
    }

    pub fn axis(mut self, axis: Axis) -> Self {
        self.0 = self.0.axis(axis);
        self.1 = axis;
        self
    }
}

element!(@trait ToggleGroup, Styled);
element!(@trait ToggleGroup, Parent);
element!(@trait ToggleGroup, Interactive);

impl RenderOnce for ToggleGroup {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let Self(mut group, axis) = self;
        let overrides = std::mem::take(group.style());
        let theme = Theme::global(cx);
        let mut group = group.flex().items_center().gap(theme.unit(4.));
        if axis == Axis::Vertical {
            group = group.flex_col();
        }
        group.style().refine(&overrides);
        group
    }
}

/// Applies the field-label typography to a composable element.
pub fn label<T: Styled>(element: T, cx: &App) -> T {
    let t = Theme::global(cx);
    t.text(element, t.typography.sm)
}

/// Groups a label, a control, and optional help text.
#[derive(IntoElement)]
pub struct Field {
    label: Div,
    description: SharedString,
    control: AnyElement,
    style: StyleRefinement,
}

impl Field {
    pub fn new(
        label: Div,
        description: impl Into<SharedString>,
        control: impl IntoElement,
    ) -> Self {
        Self {
            label,
            description: description.into(),
            control: control.into_any_element(),
            style: StyleRefinement::default(),
        }
    }
}

impl Styled for Field {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Field {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::global(cx);
        let mut field = crate::theme::column()
            .w_full()
            .gap(t.unit(8.))
            .child(label(self.label, cx))
            .child(self.control);
        if !self.description.is_empty() {
            field = field.child(div().text_color(t.muted_foreground).child(self.description));
        }
        field.style().refine(&self.style);
        field
    }
}
