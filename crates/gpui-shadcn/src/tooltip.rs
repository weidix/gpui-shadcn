use gpui::{prelude::FluentBuilder, *};

use crate::{
    Button, Theme,
    theme::{column, row},
};

#[derive(Clone, Copy)]
pub enum TooltipSide {
    Left,
    Top,
    Bottom,
    Right,
}

impl TooltipSide {
    fn popup(self, t: Theme, label: SharedString, shortcut: Option<SharedString>, cx: &App) -> Div {
        use TooltipSide::*;
        let gap = t.unit(4.);
        let background = t.foreground;
        let horizontal = matches!(self, Left | Right);
        let (width, height) = if horizontal { (5., 10.) } else { (10., 5.) };
        let arrow = canvas(
            |_, _, _| (),
            move |b, _, window, _| {
                let [start, end, tip] = match self {
                    Left => [
                        b.origin,
                        point(b.left(), b.bottom()),
                        point(b.right(), b.center().y),
                    ],
                    Right => [
                        point(b.right(), b.top()),
                        b.bottom_right(),
                        point(b.left(), b.center().y),
                    ],
                    Top => [
                        b.origin,
                        point(b.right(), b.top()),
                        point(b.center().x, b.bottom()),
                    ],
                    Bottom => [
                        point(b.left(), b.bottom()),
                        b.bottom_right(),
                        point(b.center().x, b.top()),
                    ],
                };
                let mut path = PathBuilder::fill();
                path.move_to(start);
                path.line_to(end);
                path.line_to(tip);
                path.close();
                if let Ok(path) = path.build() {
                    window.paint_path(path, background);
                }
            },
        )
        .w(t.unit(width))
        .h(t.unit(height))
        .flex_shrink_0();
        let anchor = row().absolute().justify_center();
        let (anchor, content, offset) = match self {
            Left => (
                anchor.top_0().bottom_0().right(relative(1.)).pr(gap),
                row(),
                point(px(3.), px(0.)),
            ),
            Right => (
                anchor.top_0().bottom_0().left(relative(1.)).pl(gap),
                row().flex_row_reverse(),
                point(px(-3.), px(0.)),
            ),
            Top => (
                anchor.left_0().right_0().bottom_full().pb(gap),
                column().items_center(),
                point(px(0.), px(3.)),
            ),
            Bottom => (
                anchor.left_0().right_0().top(relative(1.)).pt(gap),
                column().flex_col_reverse().items_center(),
                point(px(0.), px(-3.)),
            ),
        };
        anchor.child(crate::motion::enter(
            content
                .flex_shrink_0()
                .child(
                    div()
                        .id("tooltip-content")
                        .role(Role::Tooltip)
                        .rounded(t.radius_control())
                        .bg(background)
                        .text_color(t.background)
                        .px(t.unit(12.))
                        .py(t.unit(6.))
                        .text_size(t.unit(t.typography.xs.size))
                        .line_height(t.unit(t.typography.xs.line_height))
                        .whitespace_nowrap()
                        .child(
                            row()
                                .gap(t.unit(6.))
                                .child(label)
                                .children(shortcut.map(|text| div().opacity(0.72).child(text))),
                        ),
                )
                .child(arrow),
            "tooltip-entry",
            t.motion.normal,
            offset,
            cx,
        ))
    }
}

struct TooltipState {
    focus: FocusHandle,
    hovered: bool,
    dismissed: bool,
}

/// A tooltip that appears on pointer hover or keyboard focus.
#[derive(IntoElement)]
pub struct Tooltip {
    id: ElementId,
    label: SharedString,
    trigger: Button,
    side: TooltipSide,
    shortcut: Option<SharedString>,
}

impl Tooltip {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>, trigger: Button) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            trigger,
            side: TooltipSide::Top,
            shortcut: None,
        }
    }

    pub fn side(mut self, side: TooltipSide) -> Self {
        self.side = side;
        self
    }
    pub fn shortcut(mut self, shortcut: impl Into<SharedString>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }
}

impl RenderOnce for Tooltip {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::global(cx);
        let state = window.use_keyed_state((self.id.clone(), "tooltip-state"), cx, |window, cx| {
            let focus = cx.focus_handle().tab_stop(true);
            cx.on_focus(&focus, window, |_, window, cx| {
                cx.notify();
                window.refresh();
            })
            .detach();
            cx.on_blur(&focus, window, |state: &mut TooltipState, window, cx| {
                state.dismissed = false;
                cx.notify();
                window.refresh();
            })
            .detach();
            TooltipState {
                focus,
                hovered: false,
                dismissed: false,
            }
        });
        let current = state.read(cx);
        let focus = current.focus.clone();
        let visible = !current.dismissed
            && (current.hovered || (focus.is_focused(window) && window.last_input_was_keyboard()));
        let hover_state = state.clone();
        let click_state = state.clone();
        div()
            .id(self.id)
            .relative()
            .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                click_state.update(cx, |state, cx| {
                    state.dismissed = true;
                    cx.notify();
                });
                window.refresh();
            })
            .on_key_down(move |event, window, cx| {
                if event.keystroke.key == "escape" && visible {
                    state.update(cx, |state, cx| {
                        state.dismissed = true;
                        cx.notify();
                    });
                    window.refresh();
                }
            })
            .on_hover(move |hovered, window, cx| {
                hover_state.update(cx, |state, cx| {
                    state.hovered = *hovered;
                    state.dismissed = false;
                    cx.notify();
                });
                window.refresh();
            })
            .child(self.trigger.track_focus(&focus))
            .when(visible, |element| {
                element.child(self.side.popup(t, self.label, self.shortcut, cx))
            })
    }
}
