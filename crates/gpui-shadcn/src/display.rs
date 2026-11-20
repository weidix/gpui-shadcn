//! Composable display and navigation components.

use gpui::*;

use crate::{Theme, theme::row};

element!(Accordion(gpui_base::Accordion) + Styled + Parent + Interactive);
element!(AccordionItem(gpui_base::AccordionItem) + Styled + Parent);
element!(AccordionHeader(gpui_base::AccordionHeader) + Styled + Parent);
element!(AccordionTrigger(gpui_base::AccordionTrigger) + Styled + Parent + Interactive);
element!(AccordionPanel(gpui_base::AccordionPanel) + Styled + Parent);
constructor!(Accordion(id: impl Into<ElementId>));
constructor!(AccordionItem());
constructor!(AccordionTrigger(id: impl Into<ElementId>));
constructor!(AccordionPanel());

impl AccordionItem {
    builders! {
        open(open: bool);
        disabled(disabled: bool);
    }

    pub fn header(self, header: AccordionHeader) -> Self {
        Self(self.0.header(header.0))
    }

    pub fn panel(self, panel: AccordionPanel) -> Self {
        Self(self.0.panel(panel.0))
    }
}

impl AccordionHeader {
    pub fn new(trigger: AccordionTrigger) -> Self {
        Self(gpui_base::AccordionHeader::new(trigger.0))
    }
}

impl AccordionTrigger {
    builders! {
        open(open: bool);
        disabled(disabled: bool);
        on_change(handler: impl Fn(bool, &ClickEvent, &mut Window, &mut App) + 'static);
    }
}

impl AccordionPanel {
    builders! {
        open(open: bool);
        keep_mounted(keep_mounted: bool);
    }
}

element!(Avatar(gpui_base::Avatar) + Styled);
element!(AvatarImage(gpui_base::AvatarImage) + Styled);
element!(AvatarFallback(gpui_base::AvatarFallback) + Styled + Parent);
constructor!(Avatar());
constructor!(AvatarImage(source: impl Into<ImageSource>));
constructor!(AvatarFallback());

impl Avatar {
    pub fn image(self, image: AvatarImage) -> Self {
        Self(self.0.image(image.0))
    }

    pub fn fallback(self, fallback: AvatarFallback) -> Self {
        Self(self.0.fallback(fallback.0))
    }
}

element!(Table(gpui_base::Table) + Styled + Parent + Interactive);
element!(TableHeader(gpui_base::TableHeader) + Styled + Parent);
element!(TableBody(gpui_base::TableBody) + Styled + Parent);
element!(TableRow(gpui_base::TableRow) + Styled + Parent + Interactive);
element!(TableHead(gpui_base::TableHead) + Styled + Parent + Interactive);
element!(TableCell(gpui_base::TableCell) + Styled + Parent + Interactive);
element!(TableCaption(gpui_base::TableCaption) + Styled + Parent);
constructor!(Table(id: impl Into<ElementId>));
constructor!(TableHeader(id: impl Into<ElementId>));
constructor!(TableBody(id: impl Into<ElementId>));
constructor!(TableRow(id: impl Into<ElementId>, index: usize));
constructor!(TableHead(id: impl Into<ElementId>, index: usize));
constructor!(TableCell(id: impl Into<ElementId>, index: usize));
constructor!(TableCaption(id: impl Into<ElementId>));

impl Table {
    builders! {
        accessibility_label(label: impl Into<SharedString>);
        column_count(count: usize);
        row_count(count: usize);
    }
}

element!(Tabs(gpui_base::Tabs) + Styled + Parent + Interactive);
element!(Tab(gpui_base::Tab) + Styled + Parent + Interactive);
constructor!(Tabs(id: impl Into<ElementId>));
constructor!(Tab(id: impl Into<ElementId>));

impl Tab {
    builders! {
        selected(selected: bool);
        disabled(disabled: bool);
        set_position(position: usize, size: usize);
        accessibility_label(label: impl Into<SharedString>);
        on_click(handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static);
    }
}

element!(Collapsible(gpui_base::Collapsible) + Styled + Parent);
constructor!(Collapsible());

impl Collapsible {
    builders! {
        open(open: bool);
        content(content: impl IntoElement);
        reveal(id: impl Into<ElementId>, progress: f32);
    }
}

element!(Pagination(gpui_base::Pagination) + Styled + Parent + Interactive);

impl Pagination {
    pub fn new(id: impl Into<ElementId>, current_page: usize, total_pages: usize) -> Self {
        Self(gpui_base::Pagination::new(
            id,
            gpui_base::PaginationState::new(current_page, total_pages),
        ))
    }

    builders! {
        accessibility_label(label: impl Into<SharedString>);
    }
}

/// A determinate progress bar. Its value is a fraction in the range 0 to 1.
#[derive(IntoElement)]
pub struct Progress(gpui_base::Progress, f32);

impl Progress {
    pub fn new(id: impl Into<ElementId>, value: f32) -> Self {
        let value = if value.is_nan() {
            0.
        } else {
            value.clamp(0., 1.)
        };
        Self(gpui_base::Progress::new(id).value(value), value)
    }

    builders! {
        accessibility_label(label: impl Into<SharedString>);
    }
}

element!(@trait Progress, Styled);
element!(@trait Progress, Parent);
element!(@trait Progress, Interactive);

impl RenderOnce for Progress {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::global(cx);
        self.0.child(
            gpui_base::ProgressTrack::new()
                .w_full()
                .h(theme.unit(8.))
                .rounded_full()
                .bg(theme.primary.opacity(0.18))
                .overflow_hidden()
                .child(
                    gpui_base::ProgressIndicator::new()
                        .w(relative(self.1))
                        .h_full()
                        .rounded_full()
                        .bg(theme.primary),
                ),
        )
    }
}

/// Color variants for status and category labels.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BadgeVariant {
    #[default]
    Default,
    Secondary,
    Outline,
    Destructive,
}

/// A compact status or category label.
#[derive(IntoElement)]
pub struct Badge(Div, BadgeVariant);

impl Badge {
    pub fn new(label: impl Into<SharedString>, variant: BadgeVariant) -> Self {
        Self(row().child(label.into()), variant)
    }
}

element!(@trait Badge, Styled);

impl RenderOnce for Badge {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let Self(mut badge, variant) = self;
        let theme = Theme::global(cx);
        let (background, foreground, border) = match variant {
            BadgeVariant::Default => (theme.primary, theme.primary_foreground, transparent_black()),
            BadgeVariant::Secondary => (
                theme.secondary,
                theme.secondary_foreground,
                transparent_black(),
            ),
            BadgeVariant::Outline => (theme.background, theme.foreground, theme.border),
            BadgeVariant::Destructive => (
                theme.destructive.opacity(0.12),
                theme.destructive,
                transparent_black(),
            ),
        };
        let overrides = std::mem::take(badge.style());
        let mut badge = theme
            .text(badge, theme.typography.xs)
            .font_weight(theme.typography.sm.weight)
            .h(theme.unit(22.))
            .px(theme.unit(8.))
            .rounded_full()
            .border(theme.borders.width)
            .border_color(border)
            .bg(background)
            .text_color(foreground);
        badge.style().refine(&overrides);
        badge
    }
}

/// A bordered container for related content and actions.
#[derive(IntoElement)]
pub struct Card(Div);

impl Card {
    pub fn new() -> Self {
        Self(div())
    }
}

impl Default for Card {
    fn default() -> Self {
        Self::new()
    }
}

element!(@trait Card, Styled);
element!(@trait Card, Parent);

impl RenderOnce for Card {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::global(cx);
        let mut card = self.0;
        let overrides = std::mem::take(card.style());
        let mut card = card
            .bg(t.card)
            .text_color(t.card_foreground)
            .border(t.borders.width)
            .border_color(t.border)
            .rounded(t.radius_surface())
            .shadow(t.shadows.sm.map(crate::theme::Shadow::paint).to_vec())
            .p(t.unit(24.))
            .flex()
            .flex_col()
            .gap(t.unit(20.));
        card.style().refine(&overrides);
        card
    }
}
