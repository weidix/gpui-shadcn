use super::{Demos, text_link};
use crate::{
    catalog::Page,
    gallery::Gallery,
    icon,
    theme::{Theme, Variant, column, row},
};
use gpui_shadcn::gpui::{prelude::FluentBuilder, *};
use gpui_shadcn::{display, motion::MotionReveal};

impl Demos {
    pub(super) fn display(
        &self,
        page: Page,
        t: Theme,
        window: &mut Window,
        cx: &mut Context<Gallery>,
    ) -> AnyElement {
        use Page::*;
        let muted = || div().text_color(t.muted_foreground);
        let strong = || div().font_weight(FontWeight::SEMIBOLD);
        let bar = || div().w_full().h(px(1.)).bg(t.border);
        match page {
            Accordion => display::Accordion::new("questions").flex().flex_col().w_full().max_w(px(460.))
                .children([("Is it accessible?", "It supports keyboard interaction and exposes semantic roles."),
                    ("Is it styled?", "Yes. It uses the Neutral palette and shared corner radius."),
                    ("Can I adjust the size?", "Open Settings to change the density of every preview.")]
                    .into_iter().enumerate().map(|(i, (title, body))| {
                        let open = self.expanded == Some(i);
                        let progress = crate::motion::glide(
                            (ElementId::from("accordion-progress"), title), f32::from(open), t.motion.slow, window, cx,
                        );
                        display::AccordionItem::new().open(open).border_b(t.borders.width).border_color(t.border)
                            .header(display::AccordionHeader::new(display::AccordionTrigger::new(title).open(open)
                                .flex().items_center().justify_between().w_full().py(t.unit(16.))
                                .cursor_pointer().font_weight(FontWeight::MEDIUM)
                                .child(title).child(icon("chevron-down", t.unit(16.), t.muted_foreground)
                                    .with_transformation(Transformation::rotate(percentage(progress * 0.5))))
                                .on_change(Self::change(cx, move |d, value: bool| d.expanded = value.then_some(i)))))
                            .panel(display::AccordionPanel::new().open(open).keep_mounted(true).child(
                                MotionReveal::new(("accordion-reveal", i), progress,
                                    muted().pb(t.unit(16.)).opacity(progress).child(body).into_any_element()),
                            ))
                    })).into_any_element(),
            Avatar => {
                let initials = |label| {
                    display::AvatarFallback::new().size_full().rounded_full().flex().items_center()
                        .justify_center().bg(t.muted).text_size(t.unit(t.typography.compact.size))
                        .font_weight(FontWeight::MEDIUM).child(label)
                };
                let portrait = |source| {
                    display::Avatar::new().size_full().rounded_full().image(display::AvatarImage::new(source).size_full().rounded_full())
                };
                match self.example_index(page) {
                    0 => div().size(t.unit(40.)).child(portrait("avatars/10.png")).into_any_element(),
                    1 => div().size(t.unit(40.)).relative()
                        .child(portrait("avatars/02.png"))
                        .child(div().absolute().bottom_0().right_0().size(t.unit(12.))
                            .rounded_full().border(t.borders.strong).border_color(t.background).bg(t.primary))
                        .into_any_element(),
                    2 => display::Avatar::new().size(t.unit(40.)).rounded_full().fallback(initials("CN")).into_any_element(),
                    3 => row().children(["avatars/10.png", "avatars/02.png"].into_iter().enumerate().map(|(i, source)| {
                        div().size(t.unit(42.)).rounded_full().bg(t.background).p(t.unit(2.))
                            .when(i > 0, |s| s.ml(t.unit(-10.))).child(portrait(source))
                    })).child(div().size(t.unit(42.)).rounded_full().bg(t.background).p(t.unit(2.))
                        .ml(t.unit(-10.)).child(display::Avatar::new().size_full().fallback(initials("+3"))))
                        .into_any_element(),
                    _ => unreachable!("unknown avatar example"),
                }
            }
            Badge => {
                use display::BadgeVariant;
                let (label, variant) = [
                    ("Default", BadgeVariant::Default),
                    ("Secondary", BadgeVariant::Secondary),
                    ("Outline", BadgeVariant::Outline),
                    ("Destructive", BadgeVariant::Destructive),
                ][self.example_index(page)];
                display::Badge::new(label, variant).into_any_element()
            }
            Card => display::Card::new().w_full().max_w(px(340.))
                .child(column().gap(t.unit(5.)).child(strong().text_size(t.unit(t.typography.lg.size)).child("Project settings"))
                    .child(muted().child("Manage your project and its visibility.")))
                .child(bar())
                .child(column().gap(t.unit(5.)).child(strong().child("Visibility")).child(muted().child("Only your team can see this project.")))
                .child(row().gap(t.unit(8.))
                    .child(gpui_shadcn::Button::new("cancel-card", "Cancel", Variant::Outline).on_click(Self::message(cx, "No changes saved.")))
                    .child(gpui_shadcn::Button::new("save-card", "Save changes", Variant::Primary).on_click(Self::message(cx, "Changes saved."))))
                .into_any_element(),
            Empty => column().w_full().max_w(px(360.)).items_center().text_center().gap(t.unit(12.))
                .child(row().size(t.unit(42.)).justify_center().bg(t.muted).rounded(t.radius()).child(icon("inbox", t.unit(20.), t.foreground)))
                .child(strong().text_size(t.unit(t.typography.lg.size)).child("No projects yet"))
                .child(muted().child("Create your first project to get started."))
                .child(gpui_shadcn::Button::new("create-project", "Create project", Variant::Primary).on_click(Self::message(cx, "Project created.")))
                .into_any_element(),
            Kbd => row().gap(t.unit(5.)).child(muted().pr(t.unit(6.)).child("Search"))
                .children(["⌘", "K"].map(|key| row().justify_center().min_w(t.unit(22.)).h(t.unit(22.))
                    .bg(t.muted).text_color(t.muted_foreground).rounded(t.radius_small())
                    .text_size(t.unit(t.typography.xs.size)).child(key))).into_any_element(),
            Separator => column().w_full().max_w(px(320.)).gap(t.unit(12.))
                .child(column().gap(t.unit(4.)).child(strong().child("shadcn/ui"))
                    .child(muted().child("A collection of reusable components.")))
                .child(bar()).child(row().gap(t.unit(12.)).child("Docs")
                    .child(div().h(t.unit(16.)).w(px(1.)).bg(t.border)).child("Components"))
                .into_any_element(),
            Table => {
                let grid = |id, index| display::TableRow::new(id, index).flex().border_b(t.borders.width).border_color(t.border);
                display::Table::new("invoices").accessibility_label("Invoices").column_count(4).row_count(4)
                    .flex().flex_col().w_full().max_w(px(560.))
                    .child(display::TableHeader::new("header").child(grid("heading", 0)
                        .children(["Invoice", "Status", "Method", "Amount"].into_iter().enumerate().map(|(i, label)| {
                            display::TableHead::new(label, i).flex_1().p(t.unit(8.)).font_weight(FontWeight::MEDIUM)
                                .when(i == 3, |s| s.text_right()).child(label)
                        }))))
                    .child(display::TableBody::new("body").children([
                        ["INV001", "Paid", "Credit card", "$250.00"],
                        ["INV002", "Pending", "PayPal", "$150.00"],
                        ["INV003", "Unpaid", "Bank transfer", "$350.00"],
                    ].into_iter().enumerate().map(|(i, data)| {
                        display::TableRow::new(("invoice", i), i + 1).flex().border_b(t.borders.width).border_color(t.border)
                            .children(data.into_iter().enumerate().map(|(j, label)| {
                                display::TableCell::new(label, j).flex_1().p(t.unit(8.))
                                    .when(j == 3, |s| s.text_right()).child(label)
                            }))
                    })))
                    .child(display::TableCaption::new("caption").pt(t.unit(12.)).text_center().text_color(t.muted_foreground).child("A list of recent invoices."))
                    .into_any_element()
            }
            Typography => column().w_full().max_w(px(430.)).gap(t.unit(14.))
                .child(strong().text_size(t.unit(t.typography.display.size)).line_height(t.unit(t.typography.display.line_height)).child("The art of the interface"))
                .child(muted().text_size(t.unit(t.typography.md.size)).child("Typography brings order and clarity to an interface."))
                .child(bar())
                .child(strong().text_size(t.unit(t.typography.xl.size)).line_height(t.unit(t.typography.xl.line_height)).child("A closer look"))
                .child(div().child("Use headings to guide the reader, body text to explain, and muted text for secondary details."))
                .into_any_element(),
            Breadcrumb => row().gap(t.unit(8.)).text_color(t.muted_foreground)
                .child(text_link("home", "Home").text_color(t.muted_foreground).on_click(Self::message(cx, "Home selected.")))
                .child(icon("chevron-right", t.unit(14.), t.muted_foreground))
                .child(text_link("components", "Components").text_color(t.muted_foreground).on_click(Self::message(cx, "Components selected.")))
                .child(icon("chevron-right", t.unit(14.), t.muted_foreground))
                .child(strong().text_color(t.foreground).child("Breadcrumb")).into_any_element(),
            Pagination => {
                let current = self.pagination;
                display::Pagination::new("pages", current, 3).flex().items_center().gap(t.unit(4.))
                    .child(gpui_shadcn::Button::new("back", "Previous", Variant::Ghost).disabled(current == 1)
                        .on_click(Self::action(cx, |d| d.pagination = d.pagination.saturating_sub(1).max(1))))
                    .children((1..=3).map(|i| gpui_shadcn::Button::new(("page", i), i.to_string(), if i == current { Variant::Outline } else { Variant::Ghost })
                        .w(t.unit(36.)).px_0().on_click(Self::action(cx, move |d| d.pagination = i))))
                    .child(gpui_shadcn::Button::new("forward", "Next", Variant::Ghost).disabled(current == 3)
                        .on_click(Self::action(cx, |d| d.pagination = (d.pagination + 1).min(3))))
                    .into_any_element()
            }
            Tabs => {
                let position = crate::motion::glide("tab-indicator", self.tab as f32, t.motion.normal, window, cx);
                column().w_full().max_w(px(360.)).gap(t.unit(8.))
                    .child(display::Tabs::new("account-tabs").relative().flex().p(t.unit(3.)).bg(t.muted).rounded(t.radius())
                        .child(div().absolute().inset(t.unit(3.)).child(div().relative().left(relative(position * 0.5))
                            .w(relative(0.5)).h_full().rounded(t.radius_control()).bg(t.background).shadow(t.shadows.sm.map(gpui_shadcn::theme::Shadow::paint).to_vec())))
                        .children(["Account", "Password"].into_iter().enumerate().map(|(i, label)| {
                            let selected = i == self.tab;
                            display::Tab::new(label).selected(selected).set_position(i + 1, 2).relative().flex().flex_1().justify_center().items_center()
                                .h(t.unit(30.)).rounded(t.radius_control()).cursor_pointer().font_weight(FontWeight::MEDIUM)
                                .text_color(if selected { t.foreground } else { t.muted_foreground })
                                .child(label).on_click(Self::action(cx, move |d| d.tab = i))
                        })))
                    .child(t.surface().p(t.unit(20.)).child(crate::motion::enter(
                        column().gap(t.unit(4.))
                            .child(strong().text_size(t.unit(t.typography.md.size)).child(if self.tab == 0 { "Account" } else { "Password" }))
                            .child(muted().child(if self.tab == 0 { "Manage your account preferences." } else { "Manage how you sign in to your account." })),
                        ("tab-content", self.tab), t.motion.normal, point(px(0.), px(3.)), cx,
                    )))
                    .into_any_element()
            }
            Alert => column().w_full().max_w(px(420.)).gap(t.unit(12.))
                .children([("Heads up!", "You can add components to your app using the CLI.", "terminal", t.foreground),
                    ("Something went wrong", "Check your connection and try again.", "circle-alert", t.destructive)]
                    .map(|(title, body, glyph, color)| t.surface().p(t.unit(14.)).flex().gap(t.unit(10.)).text_color(color)
                        .child(icon(glyph, t.unit(16.), color)).child(column().gap(t.unit(3.))
                            .child(strong().child(title)).child(div().child(body)))))
                .into_any_element(),
            Progress => {
                let target = if self.progress_full { 1. } else { 0.33 };
                let progress = crate::motion::glide("progress-fill", target, t.motion.slow.saturating_mul(2), window, cx);
                column().w_full().max_w(px(320.)).gap(t.unit(10.))
                    .child(row().justify_between().child("Upload progress").child(muted().child(format!("{:.0}%", progress * 100.))))
                    .child(display::Progress::new("upload", progress).accessibility_label("Upload progress"))
                    .child(row().justify_end().child(
                        gpui_shadcn::Button::new("change-progress", if self.progress_full { "Reset progress" } else { "Complete upload" }, Variant::Outline)
                            .on_click(Self::action(cx, |d| d.progress_full = !d.progress_full)),
                    ))
                    .into_any_element()
            }
            Skeleton => row().gap(t.unit(12.)).w_full().max_w(px(290.))
                .child(div().size(t.unit(48.)).rounded_full().bg(t.muted))
                .child(column().flex_1().gap(t.unit(8.))
                    .child(div().w_full().h(t.unit(12.)).rounded(t.radius()).bg(t.muted))
                    .child(div().w(relative(0.72)).h(t.unit(12.)).rounded(t.radius()).bg(t.muted)))
                .with_animation("skeleton-pulse", Animation::new(std::time::Duration::from_millis(t.motion.pulse)).repeat(),
                    |element, progress| element.opacity(1. - 0.48 * (std::f32::consts::PI * progress).sin().powi(2)))
                .into_any_element(),
            Spinner => row().gap(t.unit(8.))
                .child(icon("loader", t.unit(16.), t.foreground)
                    .with_animation("spinner", Animation::new(std::time::Duration::from_millis(t.motion.spinner)).repeat(),
                        |svg, progress| svg.with_transformation(Transformation::rotate(percentage(progress)))))
                .child("Loading…").into_any_element(),
            _ => unreachable!("page is not a display demo"),
        }
    }
}
