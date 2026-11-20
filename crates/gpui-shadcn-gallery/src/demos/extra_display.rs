use super::Demos;
use crate::{
    catalog::Page,
    gallery::Gallery,
    motion,
    theme::{Theme, Variant, column, row},
};
use gpui_shadcn::gpui::{prelude::FluentBuilder, *};
use gpui_shadcn::{
    Checkbox, CheckboxState, Input,
    display::{Collapsible, Table, TableBody, TableCell, TableHead, TableHeader, TableRow},
};

const PAYMENTS: [(&str, &str, u32); 4] = [
    ("olivia@example.com", "Success", 316),
    ("jackson@example.com", "Processing", 242),
    ("isabella@example.com", "Failed", 837),
    ("william@example.com", "Success", 125),
];

fn table_checkbox(
    t: Theme,
    id: impl Into<ElementId>,
    state: CheckboxState,
    label: impl Into<SharedString>,
) -> Checkbox {
    let marked = state != CheckboxState::Unchecked;
    Checkbox::unstyled(id)
        .state(state)
        .accessibility_label(label)
        .size(t.unit(16.))
        .flex_shrink_0()
        .flex()
        .items_center()
        .justify_center()
        .rounded(t.checkbox_radius())
        .border(t.borders.width)
        .border_color(if marked { t.primary } else { t.input })
        .cursor_pointer()
        .focus_visible(move |s| s.border_color(t.ring))
        .when(marked, |s| {
            s.bg(t.primary).child(
                canvas(
                    |_, _, _| (),
                    move |bounds, (), window, _| {
                        let p = |x, y| {
                            bounds.origin + point(bounds.size.width * x, bounds.size.height * y)
                        };
                        let mut mark = PathBuilder::stroke(t.unit(1.5));
                        if state == CheckboxState::Checked {
                            mark.move_to(p(0.16, 0.52));
                            mark.line_to(p(0.40, 0.75));
                            mark.line_to(p(0.84, 0.27));
                        } else {
                            mark.move_to(p(0.18, 0.50));
                            mark.line_to(p(0.82, 0.50));
                        }
                        if let Ok(path) = mark.build() {
                            window.paint_path(path, t.primary_foreground);
                        }
                    },
                )
                .size(t.unit(11.)),
            )
        })
}

fn resize_divider(
    t: Theme,
    index: usize,
    split: f32,
    active: bool,
    focus: &FocusHandle,
    cx: &mut Context<Gallery>,
) -> Stateful<Div> {
    let vertical = index == 0;
    let color = if active { t.ring } else { t.border };
    div()
        .id(("resize-divider", index))
        .role(Role::Splitter)
        .aria_numeric_value(f64::from(split) * 100.)
        .aria_min_numeric_value(20.)
        .aria_max_numeric_value(80.)
        .aria_label(if vertical {
            "Resize left and right panes"
        } else {
            "Resize top and bottom panes"
        })
        .track_focus(focus)
        .absolute()
        .flex()
        .items_center()
        .justify_center()
        .when(vertical, |s| {
            s.top_0()
                .bottom_0()
                .left(relative(split))
                .ml(px(-5.))
                .w(px(10.))
                .cursor_col_resize()
        })
        .when(!vertical, |s| {
            s.left_0()
                .right_0()
                .top(relative(split))
                .mt(px(-5.))
                .h(px(10.))
                .cursor_row_resize()
        })
        .focus_visible(move |s| s.bg(t.ring.opacity(0.2)))
        .on_mouse_down(
            MouseButton::Left,
            cx.listener(move |g, event: &MouseDownEvent, window, cx| {
                g.demos.extra.resize_focus[index].focus(window, cx);
                g.demos.extra.resize_drag =
                    Some((index, event.position, g.demos.extra.resize_split[index]));
                cx.stop_propagation();
                cx.notify();
            }),
        )
        .on_key_down(cx.listener(move |g, event: &KeyDownEvent, _, cx| {
            let adjustment = match (vertical, event.keystroke.key.as_str()) {
                (true, "left") | (false, "up") => -0.03,
                (true, "right") | (false, "down") => 0.03,
                _ => return,
            };
            let split = &mut g.demos.extra.resize_split[index];
            *split = (*split + adjustment).clamp(0.2, 0.8);
            cx.stop_propagation();
            cx.notify();
        }))
        .child(div().bg(color).map(|s| {
            if vertical {
                s.w(px(1.)).h_full()
            } else {
                s.h(px(1.)).w_full()
            }
        }))
        .child(
            div()
                .absolute()
                .inset_0()
                .flex()
                .items_center()
                .justify_center()
                .child(
                    div()
                        .bg(t.background)
                        .border(t.borders.width)
                        .border_color(color)
                        .rounded(t.radius_small().min(t.unit(3.)))
                        .w(t.unit(if vertical { 8. } else { 16. }))
                        .h(t.unit(if vertical { 16. } else { 8. })),
                ),
        )
}

impl Demos {
    pub(super) fn extra_display(
        &self,
        page: Page,
        t: Theme,
        window: &mut Window,
        cx: &mut Context<Gallery>,
    ) -> AnyElement {
        match page {
            Page::AspectRatio => column()
                .w_full()
                .max_w(t.unit(360.))
                .gap(t.unit(6.))
                .child(
                    t.surface()
                        .w_full()
                        .aspect_ratio(16. / 9.)
                        .bg(t.muted)
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(div().font_weight(FontWeight::SEMIBOLD).child("16 : 9")),
                )
                .child(
                    div()
                        .text_color(t.muted_foreground)
                        .child("Resize the window to see the frame adapt."),
                )
                .into_any_element(),
            Page::Direction => {
                let rtl = self.extra.rtl;
                t.surface()
                    .w_full()
                    .max_w(t.unit(330.))
                    .p(t.unit(12.))
                    .child(
                        column()
                            .gap(t.unit(10.))
                            .child(
                                row()
                                    .justify_between()
                                    .child(
                                        gpui_shadcn::Button::new(
                                            "direction",
                                            if rtl { "Use LTR" } else { "Use RTL" },
                                            Variant::Outline,
                                        )
                                        .on_click(Self::action(cx, |d| d.extra.rtl = !d.extra.rtl)),
                                    )
                                    .child(div().text_color(t.muted_foreground).child(if rtl {
                                        "Right to left"
                                    } else {
                                        "Left to right"
                                    })),
                            )
                            .child(
                                row()
                                    .gap(t.unit(8.))
                                    .when(rtl, |s| s.justify_end())
                                    .children(if rtl {
                                        ["الإعدادات", "›", "الرئيسية"]
                                    } else {
                                        ["Home", "›", "Settings"]
                                    }),
                            ),
                    )
                    .into_any_element()
            }
            Page::DataTable => self.data_table(t, cx).into_any_element(),
            Page::Chart => {
                let points = [
                    ("Jan", 72.),
                    ("Feb", 48.),
                    ("Mar", 86.),
                    ("Apr", 61.),
                    ("May", 94.),
                    ("Jun", 79.),
                ];
                t.surface()
                    .w_full()
                    .max_w(t.unit(420.))
                    .p(t.unit(16.))
                    .child(
                        column()
                            .gap(t.unit(12.))
                            .child(
                                div()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("Monthly activity"),
                            )
                            .child(row().items_end().gap(t.unit(8.)).children(points.map(
                                |(label, value)| {
                                    column()
                                        .flex_1()
                                        .items_center()
                                        .gap(t.unit(6.))
                                        .child(
                                            div()
                                                .text_size(t.unit(t.typography.caption.size))
                                                .text_color(t.muted_foreground)
                                                .child(format!("{value:.0}")),
                                        )
                                        .child(
                                            div()
                                                .w_full()
                                                .max_w(t.unit(42.))
                                                .h(t.unit(value * 1.4))
                                                .rounded_t(t.radius())
                                                .bg(t.primary),
                                        )
                                        .child(div().text_color(t.muted_foreground).child(label))
                                },
                            ))),
                    )
                    .into_any_element()
            }
            Page::CodeBlock => self.code_block(t, cx).into_any_element(),
            Page::Collapsible => {
                let open = !self.extra.collapsed;
                let progress = motion::glide(
                    "collapsible-content",
                    f32::from(open),
                    t.motion.normal,
                    window,
                    cx,
                );
                let detail_row = || {
                    t.surface()
                        .w_full()
                        .rounded(t.radius_control())
                        .px(t.unit(16.))
                        .py(t.unit(8.))
                };
                Collapsible::new()
                    .open(open)
                    .reveal("order-details", progress)
                    .w_full()
                    .max_w(t.unit(350.))
                    .flex()
                    .flex_col()
                    .child(
                        row()
                            .justify_between()
                            .px(t.unit(16.))
                            .mb(t.unit(8.))
                            .child(div().font_weight(FontWeight::SEMIBOLD).child("Order #4189"))
                            .child(
                                gpui_shadcn::Button::icon(
                                    "order-details-toggle",
                                    "Toggle details",
                                    "chevrons-up-down",
                                    Variant::Ghost,
                                    32.,
                                )
                                .on_click(Self::action(cx, |d| {
                                    d.extra.collapsed = !d.extra.collapsed
                                })),
                            ),
                    )
                    .child(
                        detail_row()
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(div().text_color(t.muted_foreground).child("Status"))
                            .child(div().font_weight(FontWeight::MEDIUM).child("Shipped")),
                    )
                    .content(
                        column()
                            .w_full()
                            .pt(t.unit(8.))
                            .gap(t.unit(8.))
                            .opacity(progress)
                            .children(
                                [
                                    ("Shipping address", "100 Market St, San Francisco"),
                                    ("Items", "2x Studio Headphones"),
                                ]
                                .map(|(label, value)| {
                                    detail_row()
                                        .child(div().font_weight(FontWeight::MEDIUM).child(label))
                                        .child(div().text_color(t.muted_foreground).child(value))
                                }),
                            ),
                    )
                    .into_any_element()
            }
            Page::Item => t
                .surface()
                .w_full()
                .max_w(t.unit(420.))
                .p(t.unit(12.))
                .child(
                    row()
                        .gap(t.unit(12.))
                        .justify_between()
                        .child(
                            column()
                                .child(
                                    div()
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("Project access"),
                                )
                                .child(
                                    div()
                                        .text_color(t.muted_foreground)
                                        .child("Manage team permissions."),
                                ),
                        )
                        .child(
                            gpui_shadcn::Button::new("manage-access", "Manage", Variant::Outline)
                                .on_click(Self::message(cx, "Project access selected.")),
                        ),
                )
                .into_any_element(),
            Page::Sidebar => self.sidebar(t, window, cx).into_any_element(),
            Page::ScrollArea => {
                let discrete = self.extra.discrete_scroll;
                let viewport = column()
                    .id(("scroll-area-demo", discrete as usize))
                    .h(t.unit(210.))
                    .p(t.unit(12.))
                    .child(
                        div()
                            .font_weight(FontWeight::SEMIBOLD)
                            .pb(t.unit(6.))
                            .child("Tags"),
                    )
                    .children((1..=30).rev().map(|patch| {
                        div()
                            .flex_shrink_0()
                            .py(t.unit(6.))
                            .border_b(t.borders.width)
                            .border_color(t.border)
                            .child(format!("v1.2.{patch}"))
                    }));
                column()
                    .w_full()
                    .max_w(t.unit(290.))
                    .gap(t.unit(12.))
                    .child(
                        row().gap(t.unit(6.)).children(
                            ["Smooth", "Discrete"]
                                .into_iter()
                                .enumerate()
                                .map(|(index, label)| {
                                    let selected = index == 1;
                                    gpui_shadcn::Button::new(
                                        label,
                                        label,
                                        if selected == discrete {
                                            Variant::Secondary
                                        } else {
                                            Variant::Outline
                                        },
                                    )
                                    .on_click(Self::action(cx, move |d| {
                                        d.extra.discrete_scroll = selected
                                    }))
                                }),
                        ),
                    )
                    .child(t.surface().overflow_hidden().child(if discrete {
                        viewport.overflow_y_scroll()
                    } else {
                        self.extra.scroll_area.vertical(viewport)
                    }))
                    .into_any_element()
            }
            Page::Resizable => self.resizable(t, cx).into_any_element(),
            _ => unreachable!(),
        }
    }

    fn resizable(&self, t: Theme, cx: &mut Context<Gallery>) -> Div {
        let [horizontal, vertical] = self.extra.resize_split;
        let drag = self.extra.resize_drag;
        let gallery = cx.entity().downgrade();
        let capture = canvas(
            |_, _, _| (),
            move |bounds, (), window, _| {
                let update = move |position: Point<Pixels>, released, cx: &mut App| {
                    let _ = gallery.update(cx, |g, cx| {
                        let extra = &mut g.demos.extra;
                        let Some((index, start, split)) = extra.resize_drag else {
                            return;
                        };
                        let delta = position - start;
                        let fraction = if index == 0 {
                            delta.x / bounds.size.width
                        } else {
                            delta.y / bounds.size.height
                        };
                        let next = (split + fraction).clamp(0.2, 0.8);
                        if released {
                            extra.resize_drag = None;
                        }
                        if released || next != extra.resize_split[index] {
                            extra.resize_split[index] = next;
                            cx.notify();
                        }
                    });
                };
                let moved = update.clone();
                window.on_mouse_event(move |event: &MouseMoveEvent, phase, _, cx| {
                    if phase == DispatchPhase::Capture
                        && event.pressed_button == Some(MouseButton::Left)
                    {
                        moved(event.position, false, cx);
                    }
                });
                window.on_mouse_event(move |event: &MouseUpEvent, phase, _, cx| {
                    if phase == DispatchPhase::Capture && event.button == MouseButton::Left {
                        update(event.position, true, cx);
                    }
                });
            },
        )
        .absolute()
        .inset_0();
        let pane = |title| {
            row()
                .justify_center()
                .font_weight(FontWeight::SEMIBOLD)
                .child(title)
        };
        t.surface()
            .rounded(t.radius_control())
            .w_full()
            .max_w(t.unit(384.))
            .h(t.unit(200.))
            .overflow_hidden()
            .child(
                row()
                    .size_full()
                    .relative()
                    .child(pane("One").w(relative(horizontal)).h_full())
                    .child(
                        column()
                            .relative()
                            .flex_1()
                            .min_w_0()
                            .h_full()
                            .child(pane("Two").h(relative(vertical)))
                            .child(pane("Three").flex_1())
                            .child(resize_divider(
                                t,
                                1,
                                vertical,
                                drag.is_some_and(|(i, _, _)| i == 1),
                                &self.extra.resize_focus[1],
                                cx,
                            )),
                    )
                    .child(resize_divider(
                        t,
                        0,
                        horizontal,
                        drag.is_some_and(|(i, _, _)| i == 0),
                        &self.extra.resize_focus[0],
                        cx,
                    ))
                    .child(capture),
            )
    }

    fn data_table(&self, t: Theme, cx: &mut Context<Gallery>) -> Div {
        let filter = self.extra.filter.read(cx).value(cx).to_lowercase();
        let mut rows: Vec<_> = PAYMENTS
            .iter()
            .enumerate()
            .filter(|(_, (email, _, _))| email.contains(filter.trim()))
            .collect();
        rows.sort_by_key(|(_, (_, _, amount))| *amount);
        if self.extra.sort_descending {
            rows.reverse();
        }
        let count = rows
            .iter()
            .filter(|(index, _)| self.extra.selected_rows[*index])
            .count();
        let select_all = match count {
            0 => CheckboxState::Unchecked,
            n if n == rows.len() => CheckboxState::Checked,
            _ => CheckboxState::Indeterminate,
        };
        let visible_rows: Vec<_> = rows.iter().map(|(index, _)| *index).collect();
        column()
            .w_full()
            .max_w(t.unit(560.))
            .gap(t.unit(8.))
            .child(
                gpui_shadcn::Input::frame("field-frame", cx).child(Input::new(&self.extra.filter)),
            )
            .child(
                t.surface().overflow_hidden().child(
                    Table::new("payments")
                        .accessibility_label("Payments")
                        .column_count(4)
                        .row_count(rows.len() + 1)
                        .child(
                            TableHeader::new("header").child(
                                TableRow::new("columns", 0)
                                    .flex()
                                    .items_center()
                                    .bg(t.muted)
                                    .child(
                                        TableHead::new("select", 0)
                                            .w(t.unit(36.))
                                            .flex_shrink_0()
                                            .flex()
                                            .justify_center()
                                            .child(
                                                table_checkbox(
                                                    t,
                                                    "select-visible-payments",
                                                    select_all,
                                                    "Select all visible payments",
                                                )
                                                .disabled(visible_rows.is_empty())
                                                .on_change(Self::change(cx, move |d, value| {
                                                    for &index in &visible_rows {
                                                        d.extra.selected_rows[index] =
                                                            value == CheckboxState::Checked;
                                                    }
                                                })),
                                            ),
                                    )
                                    .child(
                                        TableHead::new("email", 1)
                                            .flex_1()
                                            .min_w_0()
                                            .child("Email"),
                                    )
                                    .child(
                                        TableHead::new("status", 2).w(t.unit(94.)).child("Status"),
                                    )
                                    .child(
                                        TableHead::new("amount", 3).w(t.unit(96.)).child(
                                            gpui_shadcn::Button::new(
                                                "sort-amount",
                                                if self.extra.sort_descending {
                                                    "Amount ↓"
                                                } else {
                                                    "Amount ↑"
                                                },
                                                Variant::Ghost,
                                            )
                                            .w_full()
                                            .px_0()
                                            .on_click(
                                                Self::action(cx, |d| {
                                                    d.extra.sort_descending =
                                                        !d.extra.sort_descending
                                                }),
                                            ),
                                        ),
                                    ),
                            ),
                        )
                        .child(TableBody::new("rows").children(rows.iter().enumerate().map(
                            |(position, &(index, &(email, status, amount)))| {
                                let checked = self.extra.selected_rows[index];
                                TableRow::new(("payment", index), position + 1)
                                    .flex()
                                    .items_center()
                                    .h(t.unit(37.))
                                    .border_t(t.borders.width)
                                    .border_color(t.border)
                                    .when(checked, |s| s.bg(t.muted))
                                    .child(
                                        TableCell::new("select", 0)
                                            .w(t.unit(36.))
                                            .flex_shrink_0()
                                            .flex()
                                            .justify_center()
                                            .child(
                                                table_checkbox(
                                                    t,
                                                    ("payment-select", index),
                                                    if checked {
                                                        CheckboxState::Checked
                                                    } else {
                                                        CheckboxState::Unchecked
                                                    },
                                                    format!("Select {email}"),
                                                )
                                                .on_change(Self::change(cx, move |d, value| {
                                                    d.extra.selected_rows[index] =
                                                        value == CheckboxState::Checked
                                                })),
                                            ),
                                    )
                                    .child(
                                        TableCell::new("email", 1)
                                            .flex_1()
                                            .min_w_0()
                                            .child(div().truncate().child(email)),
                                    )
                                    .child(TableCell::new("status", 2).w(t.unit(94.)).child(status))
                                    .child(
                                        TableCell::new("amount", 3)
                                            .w(t.unit(96.))
                                            .text_center()
                                            .child(format!("${amount}.00")),
                                    )
                            },
                        )))
                        .when(rows.is_empty(), |s| {
                            s.child(
                                div()
                                    .p(t.unit(12.))
                                    .text_center()
                                    .child("No matching payments."),
                            )
                        }),
                ),
            )
            .child(
                div()
                    .text_color(t.muted_foreground)
                    .child(format!("{count} of {} rows selected", rows.len())),
            )
    }
}
