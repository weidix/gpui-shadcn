use super::Demos;
use crate::{
    catalog::Page,
    gallery::Gallery,
    theme::{Theme, Variant, column, row},
};
use gpui_shadcn::gpui::*;

pub(super) fn titles(page: Page) -> &'static [&'static str] {
    match page {
        Page::Input => &[
            "Basic",
            "API Key",
            "Field",
            "Field Group",
            "Disabled",
            "Invalid",
            "File",
            "Inline",
            "Grid",
            "Required",
            "Badge",
            "Input Group",
            "Button Group",
            "Form",
            "RTL",
        ],
        Page::Textarea => &["Basic", "Field", "Disabled", "Invalid", "Button", "RTL"],
        Page::ToggleGroup => &[
            "Default", "Outline", "Size", "Spacing", "Vertical", "Disabled", "Custom", "RTL",
        ],
        Page::Tooltip => &["Sides", "Keyboard Shortcut", "Disabled Button", "RTL"],
        Page::Carousel => &[
            "Default",
            "Sizes",
            "Spacing",
            "Orientation",
            "Options",
            "API",
            "Autoplay",
            "Indicators",
        ],
        Page::Button => &[
            "Default",
            "Secondary",
            "Outline",
            "Ghost",
            "Destructive",
            "Link",
            "Sizes",
            "Disabled",
            "Icons",
            "With Icon",
        ],
        Page::Avatar => &["Image", "Status", "Fallback", "Group"],
        Page::Badge => &["Default", "Secondary", "Outline", "Destructive"],
        Page::Drawer => &["Default", "Position", "Swipe Handle"],
        _ => &[],
    }
}

impl Demos {
    pub(super) fn example_index(&self, page: Page) -> usize {
        self.example_selection
            .iter()
            .find(|(item, _)| *item == page)
            .map_or(0, |(_, selected)| *selected)
    }

    fn select_example(&mut self, page: Page, index: usize, cx: &mut Context<Gallery>) {
        if let Some((_, selected)) = self
            .example_selection
            .iter_mut()
            .find(|(item, _)| *item == page)
        {
            *selected = index;
        } else {
            self.example_selection.push((page, index));
        }
        self.status = None;
        if page == Page::Carousel {
            self.set_carousel_autoplay(true, cx);
        }
        cx.notify();
    }

    pub(super) fn example_browser(
        &self,
        page: Page,
        content: AnyElement,
        t: Theme,
        cx: &Context<Gallery>,
    ) -> Div {
        let titles = titles(page);
        let selected = self.example_index(page);
        let gallery = cx.entity().downgrade();
        let picker = gpui_shadcn::Select::new("example-select", titles.iter().copied(), selected)
            .accessibility_label("Example")
            .w(px(220.))
            .on_change(move |index, _, cx| {
                let _ = gallery.update(cx, |g, cx| {
                    g.demos.select_example(page, index, cx);
                });
            });
        column()
            .w_full()
            .gap(px(16.))
            .child(
                row()
                    .flex_wrap()
                    .gap(px(8.))
                    .child(picker)
                    .child(div().flex_1())
                    .child(
                        div()
                            .text_size(px(t.typography.xs.size))
                            .text_color(t.muted_foreground)
                            .child(format!("{} / {}", selected + 1, titles.len())),
                    )
                    .children(
                        [
                            (
                                "previous-example",
                                "Previous example",
                                "arrow-left",
                                selected.saturating_sub(1),
                                selected == 0,
                            ),
                            (
                                "next-example",
                                "Next example",
                                "arrow-right",
                                (selected + 1).min(titles.len() - 1),
                                selected + 1 == titles.len(),
                            ),
                        ]
                        .map(|(id, label, glyph, index, disabled)| {
                            gpui_shadcn::Button::icon(id, label, glyph, Variant::Outline, 34.)
                                .disabled(disabled)
                                .on_click(cx.listener(move |g, _, _, cx| {
                                    g.demos.select_example(page, index, cx);
                                }))
                        }),
                    ),
            )
            .child(
                t.surface()
                    .w_full()
                    .min_h(px(360.))
                    .p(t.unit(28.))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(t.unit(t.typography.sm.size))
                    .line_height(t.unit(20.))
                    .child(
                        div()
                            .id(("active-example", selected))
                            .w_full()
                            .flex()
                            .justify_center()
                            .child(content),
                    ),
            )
            .children(self.status.as_ref().map(|status| {
                div()
                    .text_color(t.muted_foreground)
                    .text_size(px(t.typography.xs.size))
                    .child(status.clone())
            }))
    }
}
