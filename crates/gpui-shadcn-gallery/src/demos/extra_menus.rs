use super::Demos;
use crate::{
    catalog::Page,
    gallery::{Gallery, window_corner_radius},
    icon, motion,
    theme::{Theme, Variant, column, row},
};
use gpui_shadcn::gpui::{prelude::FluentBuilder, *};
use gpui_shadcn::{
    Button, Input, Radio,
    form::RadioGroup,
    overlay::{Dialog, HoverCard, Popover, Positioner},
};

const DELIVERY_TIMES: [(&str, &str); 5] = [
    ("Standard delivery", "25–35 min · Driver assigned now"),
    ("5:00 PM – 5:15 PM", "Prep starts at 4:45 PM"),
    ("5:30 PM – 5:45 PM", "Good if you're heading home"),
    ("6:00 PM – 6:15 PM", "Most popular · High demand"),
    ("6:30 PM – 6:45 PM", "Last slot before kitchen closes"),
];

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum DrawerExample {
    Delivery,
    Position,
    SwipeHandle,
}

#[derive(Clone, Copy)]
pub(super) enum DrawerSide {
    Top,
    Right,
    Bottom,
    Left,
}

impl DrawerSide {
    const ALL: [Self; 4] = [Self::Top, Self::Right, Self::Bottom, Self::Left];

    fn label(self) -> &'static str {
        match self {
            Self::Top => "Up",
            Self::Right => "Right",
            Self::Bottom => "Down",
            Self::Left => "Left",
        }
    }

    fn is_horizontal(self) -> bool {
        matches!(self, Self::Left | Self::Right)
    }

    fn drag_distance(self, origin: Point<Pixels>, current: Point<Pixels>) -> Pixels {
        match self {
            Self::Top => origin.y - current.y,
            Self::Right => current.x - origin.x,
            Self::Bottom => current.y - origin.y,
            Self::Left => origin.x - current.x,
        }
        .max(px(0.))
    }

    fn grip(self, t: Theme) -> Stateful<Div> {
        let horizontal = self.is_horizontal();
        div()
            .id("drawer-grip")
            .absolute()
            .flex()
            .items_center()
            .justify_center()
            .cursor(CursorStyle::OpenHand)
            .when(horizontal, |s| s.top_0().bottom_0().w(t.unit(20.)))
            .when(!horizontal, |s| s.left_0().right_0().h(t.unit(24.)))
            .map(|s| match self {
                Self::Top => s.bottom_0(),
                Self::Right => s.left_0(),
                Self::Bottom => s.top_0(),
                Self::Left => s.right_0(),
            })
            .child(
                div()
                    .rounded_full()
                    .bg(t.muted)
                    .when(horizontal, |s| s.w(t.unit(6.)).h(t.unit(100.)))
                    .when(!horizontal, |s| s.w(t.unit(100.)).h(t.unit(6.))),
            )
    }

    fn place<T: Styled>(self, panel: T, position: Pixels, margin: Pixels) -> T {
        match self {
            Self::Top => panel.left(margin).right(margin).top(position),
            Self::Right => panel.top(margin).bottom(margin).right(position),
            Self::Bottom => panel.left(margin).right(margin).bottom(position),
            Self::Left => panel.top(margin).bottom(margin).left(position),
        }
    }
}

impl Demos {
    pub(super) fn extra_menus(
        &self,
        page: Page,
        t: Theme,
        window: &mut Window,
        cx: &mut Context<Gallery>,
    ) -> AnyElement {
        match page {
            Page::HoverCard => HoverCard::new("profile-hover")
                .trigger(gpui_shadcn::Button::new(
                    "profile-link",
                    "@alex",
                    Variant::Link,
                ))
                .content(move |_, _| {
                    t.popup_surface()
                        .id("profile-preview")
                        .w(t.unit(250.))
                        .p(t.unit(16.))
                        .child(
                            column()
                                .gap(t.unit(4.))
                                .child(div().font_weight(FontWeight::SEMIBOLD).child("Alex Morgan"))
                                .child(
                                    div()
                                        .text_color(t.muted_foreground)
                                        .child("Design systems and desktop tools."),
                                ),
                        )
                })
                .into_any_element(),
            Page::Command => self.command(t, cx).into_any_element(),
            Page::ContextMenu => self.context_menu(t, cx).into_any_element(),
            Page::Menubar => t
                .surface()
                .rounded(t.radius_control())
                .p(t.unit(3.))
                .child(
                    row()
                        .gap(t.unit(3.))
                        .child(self.demo_menu(t, "File", &["New file", "Open file", "Save"], cx))
                        .child(self.demo_menu(t, "Edit", &["Undo", "Redo", "Copy"], cx))
                        .child(self.demo_menu(t, "View", &["Zoom in", "Zoom out"], cx)),
                )
                .into_any_element(),
            Page::NavigationMenu => row()
                .gap(t.unit(6.))
                .child(self.demo_menu(
                    t,
                    "Getting started",
                    &["Introduction", "Installation", "Typography"],
                    cx,
                ))
                .child(self.demo_menu(t, "Components", &["Forms", "Overlays", "Feedback"], cx))
                .into_any_element(),
            Page::Drawer => self.drawer(t, window, cx),
            _ => unreachable!(),
        }
    }

    fn command(&self, t: Theme, cx: &Context<Gallery>) -> Div {
        const COMMANDS: [(&str, &str); 4] = [
            ("Open project", "layout-grid"),
            ("Search files", "search"),
            ("Toggle theme", "sun"),
            ("Show settings", "sliders-horizontal"),
        ];

        let query = &self.extra.command_query;
        let filter = query.read(cx).value(cx).trim().to_lowercase();
        let matches: Vec<_> = COMMANDS
            .iter()
            .enumerate()
            .filter_map(|(index, (label, _))| {
                label.to_lowercase().contains(&filter).then_some(index)
            })
            .collect();
        let active_position = self
            .extra
            .command_active
            .and_then(|index| matches.iter().position(|item| *item == index))
            .unwrap_or_default();
        let active = matches.get(active_position).copied();
        let list_height = t.unit(167. + 2. * t.typography.xs.line_height);
        let input_focus = query.read(cx).focus_handle(cx);

        let groups = [("Workspace", 0..2), ("Preferences", 2..4)]
            .into_iter()
            .filter_map(|(heading, range)| {
                let rows = matches
                    .iter()
                    .copied()
                    .filter(move |index| range.contains(index))
                    .map(|index| {
                        let (label, glyph) = COMMANDS[index];
                        let selected = active == Some(index);
                        Button::with_content(("command", index), Variant::Ghost)
                            .accessibility_label(label)
                            .w_full()
                            .h(t.unit(34.))
                            .px(t.unit(8.))
                            .justify_start()
                            .gap(t.unit(9.))
                            .rounded(t.popup_item_radius())
                            .font_weight(FontWeight::NORMAL)
                            .when(selected, |button| {
                                button.bg(t.accent).text_color(t.accent_foreground)
                            })
                            .hover(move |style| {
                                if selected {
                                    style.bg(t.accent).text_color(t.accent_foreground)
                                } else {
                                    style.bg(transparent_black()).text_color(t.card_foreground)
                                }
                            })
                            .on_hover(cx.listener(move |gallery, hovered, _, cx| {
                                if *hovered && gallery.demos.extra.command_active != Some(index) {
                                    gallery.demos.extra.command_active = Some(index);
                                    cx.notify();
                                }
                            }))
                            .child(icon(glyph, t.unit(15.), t.muted_foreground))
                            .child(div().flex_1().min_w_0().truncate().child(label))
                            .children(selected.then(|| {
                                div()
                                    .text_size(t.unit(t.typography.xs.size))
                                    .text_color(t.muted_foreground)
                                    .child("↵")
                            }))
                            .on_click(Self::message(cx, format!("{label} executed.")))
                    })
                    .collect::<Vec<_>>();
                (!rows.is_empty()).then(|| {
                    column()
                        .child(
                            div()
                                .px(t.unit(8.))
                                .py(t.unit(5.))
                                .text_size(t.unit(t.typography.xs.size))
                                .line_height(t.unit(t.typography.xs.line_height))
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(t.muted_foreground)
                                .child(heading),
                        )
                        .children(rows)
                })
            })
            .collect::<Vec<_>>();

        t.surface()
            .bg(t.card)
            .text_color(t.card_foreground)
            .w_full()
            .max_w(t.unit(384.))
            .overflow_hidden()
            .child(
                column()
                    .id("command-palette")
                    .capture_key_down(cx.listener(
                        move |gallery, event: &KeyDownEvent, window, cx| {
                            let key = &event.keystroke;
                            if !input_focus.is_focused(window)
                                || matches.is_empty()
                                || key.modifiers.control
                                || key.modifiers.platform
                                || key.modifiers.alt
                                || key.modifiers.shift
                            {
                                return;
                            }
                            let next = match key.key.as_str() {
                                "down" => Some((active_position + 1) % matches.len()),
                                "up" => Some((active_position + matches.len() - 1) % matches.len()),
                                "enter" => {
                                    let (label, _) = COMMANDS[matches[active_position]];
                                    gallery.demos.status =
                                        Some(format!("{label} executed.").into());
                                    None
                                }
                                _ => return,
                            };
                            if let Some(next) = next {
                                gallery.demos.extra.command_active = Some(matches[next]);
                            }
                            cx.stop_propagation();
                            window.prevent_default();
                            cx.notify();
                        },
                    ))
                    .child(
                        div().p(t.unit(8.)).child(
                            Input::search("command-search", query, cx)
                                .h(t.unit(34.))
                                .px(t.unit(10.)),
                        ),
                    )
                    .child(if active.is_some() {
                        column()
                            .min_h(list_height)
                            .px(t.unit(6.))
                            .pb(t.unit(6.))
                            .gap(t.unit(5.))
                            .children(groups)
                    } else {
                        column()
                            .h(list_height)
                            .items_center()
                            .justify_center()
                            .text_color(t.muted_foreground)
                            .child("No commands found.")
                    })
                    .child(
                        row()
                            .border_t(t.borders.width)
                            .border_color(t.border)
                            .gap(t.unit(12.))
                            .px(t.unit(14.))
                            .py(t.unit(8.))
                            .text_size(t.unit(t.typography.xs.size))
                            .text_color(t.muted_foreground)
                            .child("↑ ↓ Navigate")
                            .child("↵ Run"),
                    ),
            )
    }

    fn context_menu(&self, t: Theme, cx: &Context<Gallery>) -> Div {
        let trigger_focus = self.modal_trigger_focus.clone();
        let menu_focus = self.modal_focus.clone();
        let show_menu = cx.listener(move |gallery, event: &MouseDownEvent, window, cx| {
            gallery.demos.extra.context_menu_at = Some(event.position);
            gallery.demos.extra.context_open_with = false;
            let focus = menu_focus.clone();
            window.on_next_frame(move |window, cx| focus.focus(window, cx));
            cx.notify();
        });
        let menu = self.extra.context_menu_at.map(|position| {
            let filename = if self.extra.context_show_extension {
                "component-notes.md"
            } else {
                "component-notes"
            };
            let item = |label: &'static str, shortcut: &'static str| {
                gpui_shadcn::Button::new(label, label, Variant::Ghost)
                    .w_full()
                    .h(t.unit(30.))
                    .rounded(t.popup_item_radius())
                    .justify_between()
                    .px(t.unit(8.))
                    .child(
                        div()
                            .text_size(t.unit(t.typography.caption.size))
                            .text_color(t.muted_foreground)
                            .child(shortcut),
                    )
            };
            let action = |label, shortcut| {
                let focus = trigger_focus.clone();
                item(label, shortcut).on_click(cx.listener(move |gallery, _, window, cx| {
                    if label == "Copy filename" {
                        cx.write_to_clipboard(ClipboardItem::new_string(
                            "component-notes.md".into(),
                        ));
                    }
                    gallery.demos.extra.context_menu_at = None;
                    gallery.demos.status = Some(if label == "Copy filename" {
                        "Filename copied.".into()
                    } else {
                        format!("{label} selected.").into()
                    });
                    focus.focus(window, cx);
                    cx.notify();
                }))
            };
            let toggle = |label, checked, change: fn(&mut Demos)| {
                item(label, if checked { "✓" } else { "" })
                    .selected(checked)
                    .on_click(Self::action(cx, change))
            };
            let separator = || div().h(px(1.)).my(t.unit(4.)).bg(t.border);
            let expanded = self.extra.context_open_with;
            let panel = t
                .popup_surface()
                .id("context-menu")
                .track_focus(&self.modal_focus)
                .w(t.unit(224.))
                .p(t.unit(4.))
                .on_mouse_down_out(cx.listener(|gallery, _, _, cx| {
                    gallery.demos.extra.context_menu_at = None;
                    cx.notify();
                }))
                .child(
                    column()
                        .child(
                            div()
                                .px(t.unit(8.))
                                .py(t.unit(4.))
                                .text_size(t.unit(t.typography.xs.size))
                                .text_color(t.muted_foreground)
                                .child(filename),
                        )
                        .child(action("Open", ""))
                        .child(
                            item("Open with", if expanded { "⌄" } else { "›" })
                                .selected(expanded)
                                .on_click(Self::action(cx, |d| {
                                    d.extra.context_open_with = !d.extra.context_open_with
                                })),
                        )
                        .when(expanded, |s| {
                            s.child(
                                column()
                                    .ml(t.unit(10.))
                                    .pl(t.unit(5.))
                                    .border_l(t.borders.width)
                                    .border_color(t.border)
                                    .child(action("Markdown preview", ""))
                                    .child(action("Text editor", "")),
                            )
                        })
                        .child(action("Copy filename", ""))
                        .child(separator())
                        .child(toggle("Pin to sidebar", self.extra.context_pinned, |d| {
                            d.extra.context_pinned = !d.extra.context_pinned
                        }))
                        .child(toggle(
                            "Show extension",
                            self.extra.context_show_extension,
                            |d| d.extra.context_show_extension = !d.extra.context_show_extension,
                        ))
                        .child(separator())
                        .child(
                            div()
                                .px(t.unit(8.))
                                .py(t.unit(4.))
                                .text_size(t.unit(t.typography.xs.size))
                                .text_color(t.muted_foreground)
                                .child("Sort by"),
                        )
                        .child(
                            item(
                                "Name",
                                if self.extra.context_sort_by_name {
                                    "●"
                                } else {
                                    ""
                                },
                            )
                            .selected(self.extra.context_sort_by_name)
                            .on_click(Self::action(cx, |d| d.extra.context_sort_by_name = true)),
                        )
                        .child(
                            item(
                                "Date modified",
                                if self.extra.context_sort_by_name {
                                    ""
                                } else {
                                    "●"
                                },
                            )
                            .selected(!self.extra.context_sort_by_name)
                            .on_click(Self::action(cx, |d| d.extra.context_sort_by_name = false)),
                        )
                        .child(separator())
                        .child(
                            item("Move to Trash", "")
                                .text_color(t.destructive)
                                .disabled(true),
                        ),
                );
            deferred(
                Positioner::corner(Anchor::TopLeft, position)
                    .occlude()
                    .child(panel),
            )
            .with_priority(10)
        });
        div()
            .child(
                gpui_shadcn::Button::new("context-trigger", "Right-click here", Variant::Outline)
                    .w(t.unit(210.))
                    .h(t.unit(100.))
                    .child(icon("terminal", t.unit(15.), t.muted_foreground))
                    .track_focus(&trigger_focus)
                    .on_mouse_down(MouseButton::Right, show_menu),
            )
            .children(menu)
    }

    fn demo_menu(
        &self,
        t: Theme,
        title: &'static str,
        items: &'static [&'static str],
        cx: &Context<Gallery>,
    ) -> Popover {
        let gallery = cx.entity().downgrade();
        let item_radius = t.popup_item_radius();
        Popover::new(format!("example-menu-{title}"))
            .offset(t.unit(5.))
            .trigger(gpui_shadcn::Button::new(title, title, Variant::Ghost).rounded(item_radius))
            .content(move |popup, _, pcx| {
                let panel =
                    t.popup_surface()
                        .w(t.unit(180.))
                        .p(t.unit(4.))
                        .child(column().children(items.iter().map(|&label| {
                            let gallery = gallery.clone();
                            let popup = popup.clone();
                            gpui_shadcn::Button::new(label, label, Variant::Ghost)
                                .justify_start()
                                .rounded(item_radius)
                                .h(t.unit(30.))
                                .on_click(move |_, window, cx| {
                                    popup.dismiss(window, cx);
                                    let _ = gallery.update(cx, |g, cx| {
                                        g.demos.status = Some(format!("{label} selected.").into());
                                        cx.notify();
                                    });
                                })
                        })));
                motion::enter(
                    panel,
                    "menu-entry",
                    t.motion.fast,
                    point(px(0.), px(-3.)),
                    pcx,
                )
            })
    }

    fn drawer(&self, t: Theme, window: &mut Window, cx: &mut Context<Gallery>) -> AnyElement {
        let viewport = window.viewport_size();
        let narrow = viewport.width < px(768.);
        let trigger = |index, label, variant, example, side, cx: &mut Context<Gallery>| {
            gpui_shadcn::Button::new(("open-drawer", index), label, variant)
                .track_focus(&self.extra.drawer_triggers[index])
                .on_click(cx.listener(move |g, _, window, cx| {
                    if g.demos.extra.drawer_side.is_none() {
                        g.demos.extra.drawer_side = Some(side);
                        g.demos.extra.drawer_example = example;
                        g.demos.extra.drawer_trigger = index;
                        g.demos.extra.drawer_drag = None;
                        g.demos.extra.drawer_release = None;
                        g.demos
                            .extra
                            .drawer_scroll
                            .set_offset(point(px(0.), px(0.)));
                        g.demos.modal_focus.focus(window, cx);
                        cx.notify();
                    }
                }))
        };
        let selected = self.example_index(Page::Drawer);
        let triggers = column()
            .items_center()
            .gap(t.unit(16.))
            .when(selected == 0, |s| {
                s.child(trigger(
                    0,
                    "Open Drawer",
                    Variant::Secondary,
                    DrawerExample::Delivery,
                    if narrow {
                        DrawerSide::Bottom
                    } else {
                        DrawerSide::Right
                    },
                    cx,
                ))
            })
            .when(selected > 0, |s| {
                let example = if selected == 1 {
                    DrawerExample::Position
                } else {
                    DrawerExample::SwipeHandle
                };
                s.child(
                    row().flex_wrap().gap(t.unit(8.)).children(
                        DrawerSide::ALL
                            .into_iter()
                            .enumerate()
                            .map(|(index, side)| {
                                trigger(
                                    1 + 4 * (selected - 1) + index,
                                    side.label(),
                                    Variant::Outline,
                                    example,
                                    side,
                                    cx,
                                )
                            }),
                    ),
                )
            });
        let Some(side) = self.extra.drawer_side else {
            return triggers.into_any_element();
        };
        let closing = self.extra.drawer_closing;
        let reduce_motion = cx.reduce_motion();
        let gallery = cx.entity().downgrade();
        let trigger_focus = self.extra.drawer_triggers[self.extra.drawer_trigger].clone();
        let dismiss = move |window: &mut Window, cx: &mut App| {
            let _ = gallery.update(cx, |g, cx| {
                if g.demos.extra.drawer_closing {
                    return;
                }
                if let Some((_, distance)) = g.demos.extra.drawer_drag.take() {
                    g.demos.extra.drawer_release = Some(distance);
                }
                if cx.reduce_motion() {
                    g.demos.extra.drawer_side = None;
                    trigger_focus.focus(window, cx);
                } else {
                    g.demos.extra.drawer_closing = true;
                    let timer = cx
                        .background_executor()
                        .timer(std::time::Duration::from_millis(t.motion.drawer));
                    let trigger_focus = trigger_focus.clone();
                    cx.spawn_in(window, async move |weak, cx| {
                        timer.await;
                        let _ = weak.update_in(cx, |g, window, cx| {
                            g.demos.extra.drawer_side = None;
                            g.demos.extra.drawer_closing = false;
                            trigger_focus.focus(window, cx);
                            cx.notify();
                        });
                    })
                    .detach();
                }
                cx.notify();
            });
        };
        let close = dismiss.clone();
        let drag = self.extra.drawer_drag;
        let released = self.extra.drawer_release;
        let gesture = self.extra.drawer_gesture;
        let example = self.extra.drawer_example;
        let horizontal = side.is_horizontal();
        let show_handle = example == DrawerExample::SwipeHandle
            || (example == DrawerExample::Delivery && !horizontal);
        let gallery_for_drag = cx.entity().downgrade();
        let start_drag = move |event: &MouseDownEvent, _: &mut Window, cx: &mut App| {
            let _ = gallery_for_drag.update(cx, |g, cx| {
                if !g.demos.extra.drawer_closing {
                    g.demos.extra.drawer_drag = Some((event.position, px(0.)));
                    cx.stop_propagation();
                    cx.notify();
                }
            });
        };
        let mouse_tracking = drag.map(|(origin, _)| {
            let gallery = cx.entity().downgrade();
            let finish = dismiss.clone();
            canvas(
                |_, _, _| (),
                move |_, (), window, _| {
                    let moved = gallery.clone();
                    window.on_mouse_event(move |event: &MouseMoveEvent, phase, _, cx| {
                        if phase != DispatchPhase::Capture
                            || event.pressed_button != Some(MouseButton::Left)
                        {
                            return;
                        }
                        let _ = moved.update(cx, |g, cx| {
                            g.demos.extra.drawer_drag =
                                Some((origin, side.drag_distance(origin, event.position)));
                            cx.notify();
                        });
                    });
                    window.on_mouse_event(move |event: &MouseUpEvent, phase, window, cx| {
                        if phase != DispatchPhase::Capture || event.button != MouseButton::Left {
                            return;
                        }
                        let distance = side.drag_distance(origin, event.position);
                        let _ = gallery.update(cx, |g, cx| {
                            g.demos.extra.drawer_drag = None;
                            g.demos.extra.drawer_release = Some(distance);
                            g.demos.extra.drawer_gesture += 1;
                            cx.notify();
                        });
                        if distance >= t.unit(72.) {
                            finish(window, cx);
                        }
                        cx.stop_propagation();
                    });
                },
            )
            .absolute()
            .inset_0()
        });

        let margin = t.unit(8.);
        let max_height = (viewport.height - t.unit(96.)).max(t.unit(140.));
        let title = match example {
            DrawerExample::Delivery => "Pick a delivery time",
            DrawerExample::Position => "Move Goal",
            DrawerExample::SwipeHandle => "Drawer",
        };
        let description = match example {
            DrawerExample::Delivery => "We'll prepare your order as soon as possible.",
            DrawerExample::Position => "Set your daily activity goal.",
            DrawerExample::SwipeHandle => "Drawer with a swipe handle.",
        };
        let header = column()
            .id("drawer-header")
            .flex_shrink_0()
            .p(t.unit(16.))
            .pb_0()
            .gap(t.unit(6.))
            .when(!horizontal && narrow, |s| s.text_center())
            .when(show_handle && matches!(side, DrawerSide::Bottom), |s| {
                s.pt(t.unit(23.))
            })
            .when(show_handle && matches!(side, DrawerSide::Right), |s| {
                s.pl(t.unit(24.))
            })
            .when(
                cfg!(target_os = "macos") && matches!(side, DrawerSide::Left),
                |s| s.pt(t.unit(16.).max(px(34.))),
            )
            .child(
                div()
                    .font_weight(FontWeight::MEDIUM)
                    .text_size(t.unit(t.typography.md.size))
                    .child(title),
            )
            .child(div().text_color(t.muted_foreground).child(description))
            .on_mouse_down(MouseButton::Left, start_drag.clone());
        let body = if example == DrawerExample::Delivery {
            let selected = self.extra.delivery_time;
            self.extra
                .drawer_scroll
                .vertical(column().id("drawer-delivery-times"))
                .flex_1()
                .min_h_0()
                .p(t.unit(16.))
                .child(
                    RadioGroup::new("delivery-times")
                        .axis(Axis::Vertical)
                        .flex()
                        .flex_col()
                        .gap(t.unit(8.))
                        .children(DELIVERY_TIMES.into_iter().enumerate().map(
                            |(i, (label, details))| {
                                let checked = selected == i;
                                Radio::unstyled(("delivery-time", i))
                                    .accessibility_label(label)
                                    .checked(checked)
                                    .on_change(Self::change(cx, move |d, _| {
                                        d.extra.delivery_time = i
                                    }))
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .gap(t.unit(8.))
                                    .rounded(t.radius_control())
                                    .border(t.borders.width)
                                    .border_color(t.border)
                                    .when(checked, |s| s.bg(t.muted))
                                    .p(t.unit(12.))
                                    .cursor_pointer()
                                    .child(
                                        column()
                                            .gap(t.unit(4.))
                                            .child(
                                                row()
                                                    .gap(t.unit(6.))
                                                    .flex_wrap()
                                                    .child(
                                                        div()
                                                            .font_weight(FontWeight::MEDIUM)
                                                            .child(label),
                                                    )
                                                    .when(i == 0, |s| {
                                                        s.child(
                                                            div()
                                                                .rounded_full()
                                                                .bg(t.background)
                                                                .px(t.unit(6.))
                                                                .text_size(t.unit(
                                                                    t.typography.caption.size,
                                                                ))
                                                                .child("Fastest"),
                                                        )
                                                    }),
                                            )
                                            .child(
                                                div()
                                                    .text_size(t.unit(t.typography.xs.size))
                                                    .text_color(t.muted_foreground)
                                                    .child(details),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .flex_shrink_0()
                                            .size(t.unit(16.))
                                            .rounded_full()
                                            .border(t.borders.width)
                                            .border_color(if checked { t.primary } else { t.input })
                                            .when(checked, |s| {
                                                s.child(
                                                    div()
                                                        .size(t.unit(8.))
                                                        .rounded_full()
                                                        .bg(t.primary),
                                                )
                                            }),
                                    )
                            },
                        )),
                )
                .into_any_element()
        } else {
            div()
                .id("drawer-position-content")
                .flex_1()
                .min_h_0()
                .flex()
                .p(t.unit(16.))
                .child(
                    div()
                        .w_full()
                        .bg(t.muted)
                        .when(!horizontal, |s| s.h(t.unit(320.)).max_h_full()),
                )
                .on_mouse_down(MouseButton::Left, start_drag.clone())
                .into_any_element()
        };

        let confirmation = cx.entity().downgrade();
        let panel = column()
            .id("drawer-panel")
            .role(Role::Group)
            .aria_label(title)
            .absolute()
            .bg(t.background)
            .rounded(t.radius_surface().min(px(24.)))
            .border(t.borders.width)
            .border_color(t.border)
            .shadow(t.shadows.lg.map(gpui_shadcn::theme::Shadow::paint).to_vec())
            .occlude()
            .overflow_hidden()
            .max_h(if horizontal {
                viewport.height - margin * 2.
            } else {
                max_height
            })
            .when(horizontal, |s| s.w(relative(0.75)).max_w(t.unit(384.)))
            .child(header)
            .child(body)
            .when(example != DrawerExample::SwipeHandle, |s| {
                let cancel = close.clone();
                s.child(
                    column()
                        .flex_shrink_0()
                        .gap(t.unit(8.))
                        .p(t.unit(16.))
                        .pt_0()
                        .child(
                            gpui_shadcn::Button::new(
                                "drawer-confirm",
                                if example == DrawerExample::Delivery {
                                    "Confirm Delivery Time"
                                } else {
                                    "Submit"
                                },
                                Variant::Primary,
                            )
                            .disabled(closing)
                            .on_click(move |_, window, cx| {
                                if example == DrawerExample::Delivery {
                                    let _ = confirmation.update(cx, |g, cx| {
                                        let label = DELIVERY_TIMES[g.demos.extra.delivery_time].0;
                                        g.demos.status = Some(
                                            format!("Delivery time confirmed · {label}").into(),
                                        );
                                        cx.notify();
                                    });
                                }
                                close(window, cx);
                            }),
                        )
                        .child(
                            gpui_shadcn::Button::new("drawer-cancel", "Cancel", Variant::Outline)
                                .disabled(closing)
                                .on_click(move |_, window, cx| cancel(window, cx)),
                        ),
                )
            })
            .when(show_handle, |s| {
                s.child(side.grip(t).on_mouse_down(MouseButton::Left, start_drag))
            })
            .children(mouse_tracking);
        let overlay = div()
            .absolute()
            .inset_0()
            .rounded(window_corner_radius(t, window))
            .bg(t.overlay_subtle);
        let overlay = if reduce_motion || drag.is_some() {
            overlay.into_any_element()
        } else {
            overlay
                .with_animation(
                    ("drawer-overlay", closing as usize),
                    Animation::new(std::time::Duration::from_millis(t.motion.drawer)),
                    move |overlay, progress| {
                        overlay.opacity(if closing { 1. - progress } else { progress })
                    },
                )
                .into_any_element()
        };
        let panel = if let Some((_, distance)) = drag {
            side.place(panel, margin - distance, margin)
                .into_any_element()
        } else if reduce_motion {
            side.place(panel, margin, margin).into_any_element()
        } else {
            let travel = if horizontal {
                t.unit(384.) + margin
            } else {
                max_height + margin
            };
            let (from, to) = if closing {
                (released.unwrap_or(px(0.)), travel)
            } else {
                (released.unwrap_or(travel), px(0.))
            };
            panel
                .with_animation(
                    (
                        if closing {
                            "drawer-exit"
                        } else {
                            "drawer-enter"
                        },
                        gesture,
                    ),
                    Animation::new(std::time::Duration::from_millis(t.motion.drawer))
                        .with_easing(move |progress| t.motion.easing.sample(progress)),
                    move |panel, progress| {
                        side.place(panel, margin - (from + (to - from) * progress), margin)
                    },
                )
                .into_any_element()
        };

        triggers
            .child(
                Dialog::new(cx)
                    .open(true)
                    .focus_handle(self.modal_focus.clone())
                    .backdrop(overlay)
                    .close_on_backdrop_press(!closing)
                    .request_close(move |_, window, cx| dismiss(window, cx))
                    .popup(panel),
            )
            .into_any_element()
    }
}
