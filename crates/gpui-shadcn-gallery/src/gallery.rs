use gpui_shadcn::gpui::{prelude::FluentBuilder, *};
use gpui_shadcn::{Button, Input, InputState, overlay::Popover};

use crate::{
    catalog::{GROUPS, Page},
    demos::Demos,
    icon, motion,
    scroll::SmoothScroll,
    theme::{Density, Theme, Variant, column, row},
};

pub struct Gallery {
    page: Page,
    sidebar_open: bool,
    search: Entity<InputState>,
    content_scroll: SmoothScroll,
    sidebar_scroll: SmoothScroll,
    pub demos: Demos,
}

impl Gallery {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let search = cx.new(|cx| InputState::new(window, cx, "Search components…"));
        cx.observe(&search, |_, _, cx| cx.notify()).detach();
        Self {
            page: Page::Button,
            sidebar_open: true,
            search,
            content_scroll: SmoothScroll::default(),
            sidebar_scroll: SmoothScroll::default(),
            demos: Demos::new(window, cx),
        }
    }

    fn show(&mut self, page: Page, cx: &mut Context<Self>) {
        if self.page == page {
            return;
        }
        self.page = page;
        self.demos.set_carousel_autoplay(page == Page::Carousel, cx);
        self.demos.clear_status();
        self.content_scroll.set_offset(point(px(0.), px(0.)));
        cx.notify();
    }

    fn navigation(&self, t: Theme, width: Pixels, cx: &mut Context<Self>) -> Div {
        let query = self.search.read(cx).value(cx).trim().to_lowercase();
        let matches_page = |page: &Page| {
            page.title().to_lowercase().contains(&query)
                || (*page == Page::CodeBlock && query == "codeblock")
        };
        column()
            .w(width)
            .flex_shrink_0()
            .h_full()
            .overflow_hidden()
            .text_color(t.sidebar_foreground)
            .child(
                row()
                    .h(px(68.))
                    .flex_shrink_0()
                    .px(px(16.))
                    .gap(px(10.))
                    .child(
                        row()
                            .size(px(34.))
                            .justify_center()
                            .bg(t.sidebar_primary)
                            .rounded(t.radius_control())
                            .child(icon("layers", px(18.), t.sidebar_primary_foreground)),
                    )
                    .child(
                        column()
                            .child(div().font_weight(FontWeight::SEMIBOLD).child("gpui-shadcn"))
                            .child(
                                div()
                                    .text_size(px(t.typography.caption.size))
                                    .text_color(t.muted_foreground)
                                    .child("GPUI Component Gallery"),
                            ),
                    ),
            )
            .child(
                div().px(px(12.)).pb(px(12.)).child(
                    row()
                        .h(px(34.))
                        .line_height(px(t.typography.sm.line_height))
                        .px(px(8.))
                        .gap(px(8.))
                        .bg(t.background)
                        .border(t.borders.width)
                        .border_color(t.input)
                        .rounded(t.input_radius())
                        .child(icon("search", px(15.), t.muted_foreground))
                        .child(div().flex_1().min_w_0().child(Input::new(&self.search))),
                ),
            )
            .child(
                self.sidebar_scroll
                    .vertical(column().id("sidebar-scroll"))
                    .flex_1()
                    .min_h_0()
                    .px(px(12.))
                    .pb(px(16.))
                    .gap(px(16.))
                    .when(
                        GROUPS.iter().all(|(g, _, pages)| {
                            !g.to_lowercase().contains(&query)
                                && pages.iter().all(|p| !matches_page(p))
                        }),
                        |this| {
                            this.child(
                                div()
                                    .px(px(8.))
                                    .py(px(8.))
                                    .text_color(t.muted_foreground)
                                    .child("No components found."),
                            )
                        },
                    )
                    .children(GROUPS.iter().filter_map(|&(label, glyph, pages)| {
                        let matches: Vec<_> = pages
                            .iter()
                            .copied()
                            .filter(|p| label.to_lowercase().contains(&query) || matches_page(p))
                            .collect();
                        if matches.is_empty() {
                            return None;
                        }
                        Some(
                            column()
                                .gap(px(2.))
                                .child(
                                    row()
                                        .h(px(26.))
                                        .px(px(8.))
                                        .gap(px(8.))
                                        .text_size(px(t.typography.xs.size))
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(t.muted_foreground)
                                        .child(icon(glyph, px(14.), t.muted_foreground))
                                        .child(label),
                                )
                                .children(matches.into_iter().map(|page| {
                                    Button::unstyled(page.title())
                                        .selected(page == self.page)
                                        .flex()
                                        .items_center()
                                        .w_full()
                                        .h(px(30.))
                                        .pl(px(30.))
                                        .pr(px(8.))
                                        .rounded(t.radius_control())
                                        .cursor_pointer()
                                        .hover(move |s| {
                                            s.bg(t.sidebar_accent)
                                                .text_color(t.sidebar_accent_foreground)
                                        })
                                        .focus_visible(move |s| {
                                            s.bg(t.sidebar_accent)
                                                .text_color(t.sidebar_accent_foreground)
                                        })
                                        .when(page == self.page, |s| {
                                            s.bg(t.sidebar_accent)
                                                .text_color(t.sidebar_accent_foreground)
                                                .font_weight(FontWeight::MEDIUM)
                                        })
                                        .justify_start()
                                        .accessibility_label(page.title())
                                        .child(page.title())
                                        .on_click(
                                            cx.listener(move |this, _, _, cx| this.show(page, cx)),
                                        )
                                })),
                        )
                    })),
            )
            .child(
                row()
                    .h(px(45.))
                    .flex_shrink_0()
                    .px(px(16.))
                    .gap(px(7.))
                    .border_t(t.borders.width)
                    .border_color(t.sidebar_border)
                    .text_size(px(t.typography.xs.size))
                    .text_color(t.muted_foreground)
                    .child(div().size(px(6.)).rounded_full().bg(t.muted_foreground))
                    .child(format!(
                        "{} components",
                        GROUPS.iter().map(|(_, _, p)| p.len()).sum::<usize>()
                    )),
            )
    }

    fn settings(&self, t: Theme) -> Popover {
        Popover::new("gallery-settings")
            .anchor(Anchor::TopRight)
            .offset(px(6.))
            .trigger(
                gpui_shadcn::Button::new("settings", "Settings", Variant::Outline)
                    .h(px(32.))
                    .child(icon("sliders-horizontal", px(14.), t.foreground)),
            )
            .content(move |_, _, popup_cx| {
                let setting = |title, content: Div| {
                    column()
                        .gap(px(8.))
                        .child(div().font_weight(FontWeight::MEDIUM).child(title))
                        .child(content)
                };
                let panel = t
                    .popup_surface()
                    .w(px(324.))
                    .p(px(16.))
                    .flex()
                    .flex_col()
                    .gap(px(14.))
                    .child(
                        column()
                            .gap(px(3.))
                            .child(
                                div()
                                    .text_size(px(t.typography.md.size))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("Preview settings"),
                            )
                            .child(
                                div()
                                    .text_color(t.muted_foreground)
                                    .child("Applies to every component preview."),
                            ),
                    )
                    .child(setting(
                        "Component size",
                        row()
                            .gap(px(4.))
                            .children(Density::ALL.map(|(density, label)| {
                                gpui_shadcn::Button::new(
                                    ("density", density as usize),
                                    label,
                                    if density == t.density {
                                        Variant::Primary
                                    } else {
                                        Variant::Outline
                                    },
                                )
                                .px(px(9.))
                                .h(px(30.))
                                .on_click(move |_, _, cx| {
                                    Theme {
                                        density,
                                        ..Theme::global(cx)
                                    }
                                    .set(cx);
                                })
                            })),
                    ))
                    .child(setting(
                        "Corner radius",
                        row()
                            .gap(px(4.))
                            .children([0., 8., Theme::DEFAULT_RADIUS, 18.].map(|radius| {
                                gpui_shadcn::Button::new(
                                    ("radius", radius as usize),
                                    format!("{radius} px"),
                                    if radius == t.radius {
                                        Variant::Primary
                                    } else {
                                        Variant::Outline
                                    },
                                )
                                .px(px(9.))
                                .h(px(30.))
                                .on_click(move |_, _, cx| {
                                    Theme {
                                        radius,
                                        ..Theme::global(cx)
                                    }
                                    .set(cx);
                                })
                            })),
                    ))
                    .child(setting(
                        "Appearance",
                        row()
                            .gap(px(4.))
                            .children([(false, "Light"), (true, "Dark")].map(|(dark, label)| {
                                gpui_shadcn::Button::new(
                                    ("appearance", dark as usize),
                                    label,
                                    if dark == t.dark {
                                        Variant::Primary
                                    } else {
                                        Variant::Outline
                                    },
                                )
                                .h(px(30.))
                                .on_click(move |_, _, cx| {
                                    Theme::global(cx).with_dark(dark).set(cx);
                                })
                            })),
                    ));
                motion::enter(
                    panel,
                    "settings-entry",
                    t.motion.normal,
                    point(px(0.), px(-5.)),
                    popup_cx,
                )
            })
    }

    fn content(&self, t: Theme, inset: Pixels, window: &mut Window, cx: &mut Context<Self>) -> Div {
        let pages: Vec<_> = GROUPS
            .iter()
            .flat_map(|(_, _, pages)| pages.iter().copied())
            .collect();
        let index = pages
            .iter()
            .position(|p| *p == self.page)
            .unwrap_or_default();
        column()
            .h_full()
            .bg(t.background)
            .rounded(t.radius_surface())
            .shadow(t.shadows.sm.map(gpui_shadcn::theme::Shadow::paint).to_vec())
            .overflow_hidden()
            .flex_1()
            .min_w_0()
            .min_h_0()
            .child(
                row()
                    .h(px(57.))
                    .flex_shrink_0()
                    .px(inset)
                    .gap(px(8.))
                    .justify_between()
                    .border_b(t.borders.width)
                    .border_color(t.border)
                    .child(
                        row()
                            .min_w_0()
                            .gap(px(9.))
                            .child(
                                gpui_shadcn::Button::new("toggle-sidebar", "", Variant::Ghost)
                                    .accessibility_label(if self.sidebar_open {
                                        "Hide sidebar"
                                    } else {
                                        "Show sidebar"
                                    })
                                    .size(px(32.))
                                    .p_0()
                                    .flex_shrink_0()
                                    .child(icon("panels-top-left", px(16.), t.foreground))
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.sidebar_open = !this.sidebar_open;
                                        cx.notify();
                                    })),
                            )
                            .child(div().w(px(1.)).h(px(16.)).bg(t.border).flex_shrink_0())
                            .child(
                                row()
                                    .min_w_0()
                                    .gap(px(7.))
                                    .text_color(t.muted_foreground)
                                    .when(inset >= px(24.), |s| {
                                        s.child(self.page.group()).child(icon(
                                            "chevron-right",
                                            px(14.),
                                            t.muted_foreground,
                                        ))
                                    })
                                    .child(
                                        div()
                                            .truncate()
                                            .text_color(t.foreground)
                                            .font_weight(FontWeight::MEDIUM)
                                            .child(self.page.title()),
                                    ),
                            ),
                    )
                    .child(
                        row()
                            .flex_shrink_0()
                            .gap(px(8.))
                            .child(self.settings(t))
                            .child(
                                gpui_shadcn::Button::new(
                                    "theme",
                                    if t.dark { "Light" } else { "Dark" },
                                    Variant::Ghost,
                                )
                                .h(px(32.))
                                .px(px(8.))
                                .child(icon(
                                    if t.dark { "sun" } else { "moon" },
                                    px(14.),
                                    t.foreground,
                                ))
                                .on_click(|_, _, cx| {
                                    let theme = Theme::global(cx);
                                    theme.with_dark(!theme.dark).set(cx);
                                }),
                            ),
                    ),
            )
            .child(
                self.content_scroll
                    .vertical(column().id("preview-scroll"))
                    .flex_1()
                    .min_h_0()
                    .px(inset)
                    .py(px(28.))
                    .child(
                        column()
                            .w_full()
                            .max_w(px(900.))
                            .mx_auto()
                            .gap(px(20.))
                            .child(
                                column()
                                    .gap(px(9.))
                                    .child(
                                        div()
                                            .text_size(px(t.typography.display.size))
                                            .line_height(px(42.))
                                            .font_weight(FontWeight::BOLD)
                                            .child(self.page.title()),
                                    )
                                    .child(
                                        div()
                                            .text_size(px(t.typography.md.size))
                                            .text_color(t.muted_foreground)
                                            .child(self.page.description()),
                                    ),
                            )
                            .child(self.demos.preview(self.page, t, window, cx))
                            .child(
                                row()
                                    .w_full()
                                    .justify_between()
                                    .gap(px(10.))
                                    .pt(px(6.))
                                    .children(index.checked_sub(1).map(|i| {
                                        let page = pages[i];
                                        gpui_shadcn::Button::new(
                                            "previous",
                                            page.title(),
                                            Variant::Outline,
                                        )
                                        .child(icon("arrow-left", px(14.), t.foreground))
                                        .on_click(
                                            cx.listener(move |this, _, _, cx| this.show(page, cx)),
                                        )
                                    }))
                                    .child(div().flex_1())
                                    .children(pages.get(index + 1).copied().map(|page| {
                                        gpui_shadcn::Button::new(
                                            "next",
                                            page.title(),
                                            Variant::Outline,
                                        )
                                        .child(icon("arrow-right", px(14.), t.foreground))
                                        .on_click(
                                            cx.listener(move |this, _, _, cx| this.show(page, cx)),
                                        )
                                    })),
                            ),
                    ),
            )
    }
}

#[cfg(any(target_os = "macos", target_os = "windows"))]
fn titlebar_frame() -> Stateful<Div> {
    row().id("titlebar").w_full().h(px(40.)).flex_shrink_0()
}

#[cfg(any(target_os = "macos", target_os = "windows"))]
fn titlebar_label(t: Theme) -> Div {
    div()
        .text_size(px(t.typography.xs.size))
        .font_weight(FontWeight::MEDIUM)
        .text_color(t.muted_foreground)
        .child(crate::WINDOW_TITLE)
}

#[cfg(target_os = "macos")]
fn titlebar(t: Theme, _: &Window) -> impl IntoElement {
    titlebar_frame()
        .justify_center()
        .child(titlebar_label(t))
        .on_mouse_down(MouseButton::Left, |event, window, _| {
            if event.click_count == 2 {
                window.titlebar_double_click();
            } else {
                window.start_window_move();
            }
        })
}

#[cfg(target_os = "windows")]
fn titlebar(t: Theme, window: &Window) -> impl IntoElement {
    use WindowControlArea::{Close, Drag, Max, Min};

    let title_color = if window.is_window_active() {
        t.foreground
    } else {
        t.muted_foreground
    };
    let (zoom_label, zoom_icon) = if window.is_maximized() {
        ("Restore", "restore")
    } else {
        ("Maximize", "maximize")
    };

    titlebar_frame()
        .h(px(32.))
        .child(
            row()
                .id("titlebar-drag")
                .h_full()
                .flex_1()
                .min_w_0()
                .pl(px(16.))
                .pr(px(8.))
                .window_control_area(Drag)
                .child(titlebar_label(t).truncate().text_color(title_color)),
        )
        .children(
            [
                (Min, "Minimize", "minus"),
                (Max, zoom_label, zoom_icon),
                (Close, "Close", "x"),
            ]
            .map(|(area, label, glyph)| {
                let close = area == Close;
                let hover = if close {
                    rgb(0xc42b1c).into()
                } else {
                    t.foreground.opacity(0.09)
                };
                let pressed = if close {
                    rgb(0xa82419).into()
                } else {
                    t.foreground.opacity(0.14)
                };
                row()
                    .id(label)
                    .role(Role::Button)
                    .aria_label(label)
                    .window_control_area(area)
                    .w(px(46.))
                    .h_full()
                    .flex_shrink_0()
                    .justify_center()
                    .hover(move |s| s.bg(hover))
                    .active(move |s| s.bg(pressed))
                    .when(close, |s| s.group("caption-close"))
                    .child(icon(glyph, px(16.), title_color).when(close, |s| {
                        s.group_hover("caption-close", |s| s.text_color(white()))
                    }))
            }),
        )
}

impl Render for Gallery {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let t = Theme::global(cx);
        let width = window.viewport_size().width;
        let compact = width < px(900.);
        let inset = px(8.);
        let sidebar_width = px(if compact { 194. } else { 226. });
        let sidebar = motion::glide(
            "sidebar-width",
            if self.sidebar_open { 1. } else { 0. },
            t.motion.slow,
            window,
            cx,
        );
        let focus = self.search.read(cx).focus_handle(cx);
        let workspace = row()
            .flex_1()
            .min_h_0()
            .w_full()
            .items_start()
            .when(sidebar > 0., |root| {
                root.child(
                    div()
                        .h_full()
                        .w(sidebar_width * sidebar)
                        .flex_shrink_0()
                        .when(sidebar < 1., |s| s.overflow_hidden())
                        .child(self.navigation(t, sidebar_width, cx)),
                )
            })
            .child(
                div()
                    .h_full()
                    .flex()
                    .flex_1()
                    .min_w_0()
                    .pb(inset)
                    .pr(inset)
                    .pt(if cfg!(any(target_os = "macos", target_os = "windows")) {
                        px(0.)
                    } else {
                        inset
                    })
                    .pl(inset * (1. - sidebar))
                    .child(self.content(t, px(if compact { 16. } else { 28. }), window, cx)),
            );
        let root = column()
            .id("gallery")
            .capture_key_down(cx.listener(move |this, event: &KeyDownEvent, window, cx| {
                let key = &event.keystroke;
                if key.key == "escape" && this.demos.dismiss_context_menu(window, cx) {
                    cx.stop_propagation();
                    cx.notify();
                    return;
                }
                if key.key == "k" && (key.modifiers.platform || key.modifiers.control) {
                    if !this.sidebar_open {
                        this.sidebar_open = true;
                        cx.notify();
                        let focus = focus.clone();
                        window.on_next_frame(move |window, cx| focus.focus(window, cx));
                    } else {
                        focus.focus(window, cx);
                    }
                    cx.stop_propagation();
                }
            }))
            .relative()
            .size_full()
            .tab_group()
            .bg(t.sidebar)
            .text_color(t.foreground)
            .when_some(t.typography.sans, |root, family| root.font_family(family))
            .text_size(px(t.typography.sm.size))
            .line_height(px(t.typography.sm.line_height));
        #[cfg(any(target_os = "macos", target_os = "windows"))]
        let root = root.child(titlebar(t, window));
        root.child(workspace)
            .children(self.demos.toast_overlay(t, cx))
    }
}
