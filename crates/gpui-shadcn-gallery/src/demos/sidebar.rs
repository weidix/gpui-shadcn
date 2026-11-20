use super::Demos;
use crate::{
    gallery::Gallery,
    icon, motion,
    scroll::SmoothScroll,
    theme::{Theme, Variant, column, row},
};
use gpui_shadcn::gpui::{prelude::FluentBuilder, *};
use gpui_shadcn::{Button, overlay::Popover};

const WORKSPACES: [(&str, &str, &str); 3] = [
    ("Acme Inc.", "Enterprise", "layers"),
    ("Acme Corp.", "Startup", "layout-grid"),
    ("Acme Studio", "Free", "compass"),
];

const PLATFORM: [(&str, &str, [&str; 3]); 4] = [
    (
        "Playground",
        "terminal",
        ["History", "Starred", "Playground Settings"],
    ),
    ("Models", "layers", ["Genesis", "Explorer", "Quantum"]),
    (
        "Documentation",
        "book-open",
        ["Introduction", "Get Started", "Changelog"],
    ),
    (
        "Settings",
        "sliders-horizontal",
        ["General", "Team", "Billing"],
    ),
];

const PROJECTS: [[(&str, &str); 3]; 3] = [
    [
        ("Design Engineering", "layout-grid"),
        ("Sales & Marketing", "compass"),
        ("Travel", "inbox"),
    ],
    [
        ("Product Launch", "layout-grid"),
        ("Growth", "compass"),
        ("Research", "inbox"),
    ],
    [
        ("Brand System", "layout-grid"),
        ("Website", "compass"),
        ("Archive", "inbox"),
    ],
];

pub(super) struct SidebarDemo {
    expanded: bool,
    groups: [bool; PLATFORM.len()],
    workspace: usize,
    selected: (&'static str, &'static str),
    scroll: SmoothScroll,
}

impl Default for SidebarDemo {
    fn default() -> Self {
        Self {
            expanded: true,
            groups: [true, false, false, false],
            workspace: 0,
            selected: ("Playground", "History"),
            scroll: SmoothScroll::default(),
        }
    }
}

fn section_label(t: Theme, text: &'static str) -> Div {
    div()
        .px(t.unit(8.))
        .h(t.unit(22.))
        .text_size(t.unit(t.typography.xs.size))
        .text_color(t.muted_foreground)
        .child(text)
}

fn menu_trigger(
    t: Theme,
    id: &'static str,
    glyph: &'static str,
    name: &'static str,
    detail: &'static str,
    expanded: bool,
) -> Button {
    gpui_shadcn::Button::icon(id, name, glyph, Variant::Ghost, 36.)
        .bg(t.background)
        .when(expanded, |button| {
            button
                .w_full()
                .h(t.unit(44.))
                .px(t.unit(8.))
                .justify_start()
                .bg(t.sidebar)
                .child(
                    column()
                        .flex_1()
                        .min_w_0()
                        .items_start()
                        .child(
                            div()
                                .text_size(t.unit(t.typography.compact.size))
                                .truncate()
                                .child(name),
                        )
                        .child(
                            div()
                                .text_size(t.unit(t.typography.caption.size))
                                .text_color(t.muted_foreground)
                                .truncate()
                                .child(detail),
                        ),
                )
                .child(icon("chevrons-up-down", t.unit(14.), t.muted_foreground))
        })
}

fn sidebar_menu(
    t: Theme,
    id: &'static str,
    trigger: Button,
    anchor: Anchor,
    options: [(&'static str, &'static str, bool); 3],
    choose: impl Fn(&mut SidebarDemo, usize, &'static str) + Clone + 'static,
    gallery: WeakEntity<Gallery>,
) -> Popover {
    Popover::new(id)
        .anchor(anchor)
        .offset(t.unit(4.))
        .trigger(trigger)
        .content(move |popup, _, _| {
            t.popup_surface().w(t.unit(210.)).p(t.unit(4.)).child(
                column().children(options.iter().copied().enumerate().map(
                    |(i, (label, glyph, selected))| {
                        let choose = choose.clone();
                        let gallery = gallery.clone();
                        let popup = popup.clone();
                        gpui_shadcn::Button::icon(label, label, glyph, Variant::Ghost, 32.)
                            .rounded(t.popup_item_radius())
                            .w_full()
                            .px(t.unit(8.))
                            .justify_start()
                            .child(div().flex_1().child(label))
                            .when(selected, |s| {
                                s.child(icon("check", t.unit(14.), t.foreground))
                            })
                            .on_click(move |_, window, cx| {
                                let _ = gallery.update(cx, |g, cx| {
                                    choose(&mut g.demos.extra.sidebar, i, label);
                                    cx.notify();
                                });
                                popup.dismiss(window, cx);
                            })
                    },
                )),
            )
        })
}

impl Demos {
    pub(super) fn sidebar(&self, t: Theme, window: &mut Window, cx: &mut Context<Gallery>) -> Div {
        let state = &self.extra.sidebar;
        let expanded = state.expanded;
        let full_width = if window.viewport_size().width < px(800.) {
            186.
        } else {
            216.
        };
        let width = motion::glide(
            "demo-sidebar",
            if expanded { full_width } else { 52. },
            t.motion.slow,
            window,
            cx,
        );
        let (workspace, plan, symbol) = WORKSPACES[state.workspace];
        let (section, page) = state.selected;
        let workspace_menu = sidebar_menu(
            t,
            "workspace-switcher",
            menu_trigger(t, "switch-workspace", symbol, workspace, plan, expanded)
                .accessibility_label("Switch workspace"),
            Anchor::TopLeft,
            WORKSPACES.map(|(name, _, symbol)| (name, symbol, name == workspace)),
            |sidebar, i, _| {
                sidebar.workspace = i;
                if sidebar.selected.0 == "Projects" {
                    sidebar.selected = ("Projects", PROJECTS[i][0].0);
                }
            },
            cx.entity().downgrade(),
        );
        let account_menu = sidebar_menu(
            t,
            "sidebar-account",
            menu_trigger(
                t,
                "sidebar-user",
                "circle-user",
                "shadcn",
                "m@example.com",
                expanded,
            )
            .accessibility_label("User menu"),
            Anchor::BottomLeft,
            [
                ("Account", "circle-user", false),
                ("Billing", "credit-card", false),
                ("Notifications", "bell", false),
            ],
            |sidebar, _, label| sidebar.selected = ("Account", label),
            cx.entity().downgrade(),
        );
        let navigation = state
            .scroll
            .vertical(column().id("sidebar-demo-navigation"))
            .flex_1()
            .min_h_0()
            .gap(t.unit(8.))
            .child(
                column()
                    .gap(t.unit(2.))
                    .when(expanded, |s| s.child(section_label(t, "Platform")))
                    .children(PLATFORM.into_iter().enumerate().map(
                        |(index, (label, glyph, children))| {
                            let is_open = state.groups[index];
                            let selected = section == label;
                            column()
                                .gap(t.unit(2.))
                                .child(
                                    gpui_shadcn::Button::icon(
                                        ("platform-group", index),
                                        label,
                                        glyph,
                                        Variant::Ghost,
                                        32.,
                                    )
                                    .when(selected, |s| s.bg(t.muted))
                                    .when(expanded, |s| {
                                        s.w_full()
                                            .px(t.unit(8.))
                                            .justify_start()
                                            .child(div().flex_1().child(label))
                                            .child(icon(
                                                if is_open {
                                                    "chevron-down"
                                                } else {
                                                    "chevron-right"
                                                },
                                                t.unit(14.),
                                                t.muted_foreground,
                                            ))
                                    })
                                    .on_click(Self::action(
                                        cx,
                                        move |d| {
                                            let sidebar = &mut d.extra.sidebar;
                                            if sidebar.expanded {
                                                sidebar.groups[index] = !sidebar.groups[index];
                                            } else {
                                                sidebar.expanded = true;
                                                sidebar.groups[index] = true;
                                            }
                                        },
                                    )),
                                )
                                .when(expanded && is_open, |s| {
                                    s.child(
                                        column()
                                            .ml(t.unit(16.))
                                            .pl(t.unit(8.))
                                            .border_l(t.borders.width)
                                            .border_color(t.border)
                                            .children(children.map(|child| {
                                                gpui_shadcn::Button::new(
                                                    child,
                                                    child,
                                                    Variant::Ghost,
                                                )
                                                .h(t.unit(26.))
                                                .px(t.unit(8.))
                                                .justify_start()
                                                .when(selected && page == child, |s| s.bg(t.muted))
                                                .on_click(Self::action(cx, move |d| {
                                                    d.extra.sidebar.selected = (label, child)
                                                }))
                                            })),
                                    )
                                })
                        },
                    )),
            )
            .when(expanded, |s| {
                s.child(
                    column()
                        .gap(t.unit(2.))
                        .child(section_label(t, "Projects"))
                        .children(PROJECTS[state.workspace].map(|(project, glyph)| {
                            gpui_shadcn::Button::icon(project, project, glyph, Variant::Ghost, 30.)
                                .w_full()
                                .px(t.unit(8.))
                                .justify_start()
                                .child(project)
                                .when(section == "Projects" && page == project, |s| s.bg(t.muted))
                                .on_click(Self::action(cx, move |d| {
                                    d.extra.sidebar.selected = ("Projects", project)
                                }))
                        })),
                )
            });
        let rail = div()
            .w(t.unit(width))
            .h_full()
            .flex_shrink_0()
            .overflow_hidden()
            .child(
                column()
                    .w(t.unit(if expanded { full_width } else { 52. }))
                    .h_full()
                    .p(t.unit(8.))
                    .gap(t.unit(8.))
                    .child(workspace_menu)
                    .child(navigation)
                    .child(account_menu),
            );
        let inset = column()
            .flex_1()
            .min_w_0()
            .h_full()
            .bg(t.background)
            .rounded(t.radius_control())
            .border(t.borders.width)
            .border_color(t.border)
            .child(
                row()
                    .min_w_0()
                    .h(t.unit(48.))
                    .px(t.unit(8.))
                    .gap(t.unit(8.))
                    .child(
                        gpui_shadcn::Button::icon(
                            "demo-sidebar-toggle",
                            "Toggle sidebar",
                            "panels-top-left",
                            Variant::Ghost,
                            30.,
                        )
                        .on_click(Self::action(cx, |d| {
                            d.extra.sidebar.expanded = !d.extra.sidebar.expanded
                        })),
                    )
                    .child(div().w(px(1.)).h(t.unit(16.)).bg(t.border))
                    .child(
                        div()
                            .text_color(t.muted_foreground)
                            .truncate()
                            .child(section),
                    )
                    .child(icon("chevron-right", t.unit(12.), t.muted_foreground))
                    .child(div().min_w_0().truncate().child(page)),
            )
            .child(
                column()
                    .flex_1()
                    .min_h_0()
                    .p(t.unit(14.))
                    .pt(t.unit(6.))
                    .gap(t.unit(12.))
                    .child(
                        column()
                            .gap(t.unit(4.))
                            .child(
                                div()
                                    .text_size(t.unit(t.typography.heading.size))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child(page),
                            )
                            .child(
                                div()
                                    .text_size(t.unit(t.typography.xs.size))
                                    .text_color(t.muted_foreground)
                                    .truncate()
                                    .child(workspace),
                            ),
                    )
                    .child(row().gap(t.unit(8.)).children((0..3).map(|_| {
                        div()
                            .h(t.unit(70.))
                            .flex_1()
                            .rounded(t.radius_control())
                            .bg(t.muted)
                    })))
                    .child(div().flex_1().rounded(t.radius_control()).bg(t.muted)),
            );
        t.surface()
            .w_full()
            .max_w(t.unit(760.))
            .h(t.unit(480.))
            .bg(t.sidebar)
            .overflow_hidden()
            .child(
                row().size_full().items_start().child(rail).child(
                    div()
                        .h_full()
                        .flex_1()
                        .min_w_0()
                        .p(t.unit(8.))
                        .pl_0()
                        .child(inset),
                ),
            )
    }
}
