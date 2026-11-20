mod carousel;
mod code_block;
mod controls;
mod display;
mod examples;
mod extra_display;
mod extra_forms;
mod extra_menus;
mod input;
mod messages;
mod overlays;
mod sidebar;

use crate::{
    catalog::Page,
    gallery::Gallery,
    scroll::SmoothScroll,
    theme::{Theme, Variant, column, row},
};
use gpui_shadcn::gpui::*;
use gpui_shadcn::{
    Button, InputEvent, InputState, TextareaState,
    calendar::{CalendarState, DateSelection},
    form::{OtpState, SliderState},
    overlay::DialogHandle,
};

struct Extra {
    context_menu_at: Option<Point<Pixels>>,
    context_open_with: bool,
    context_pinned: bool,
    context_show_extension: bool,
    context_sort_by_name: bool,
    code_example: code_block::Example,
    code_wrap: bool,
    calendar: Entity<CalendarState>,
    date_range: Entity<CalendarState>,
    query: Entity<InputState>,
    command_query: Entity<InputState>,
    command_active: Option<usize>,
    group_query: Entity<InputState>,
    filter: Entity<InputState>,
    choice: Option<&'static str>,
    question: usize,
    answers: [Option<&'static str>; 2],
    collapsed: bool,
    sidebar: sidebar::SidebarDemo,
    sort_descending: bool,
    selected_rows: [bool; 4],
    drawer_side: Option<extra_menus::DrawerSide>,
    drawer_closing: bool,
    drawer_triggers: [FocusHandle; 9],
    drawer_example: extra_menus::DrawerExample,
    drawer_trigger: usize,
    delivery_time: usize,
    rtl: bool,
    attachment: bool,
    trace_step: Option<usize>,
    messages_scroll: SmoothScroll,
    scroll_area: SmoothScroll,
    code_scroll: SmoothScroll,
    drawer_scroll: SmoothScroll,
    discrete_scroll: bool,
    drawer_drag: Option<(Point<Pixels>, Pixels)>,
    drawer_release: Option<Pixels>,
    drawer_gesture: usize,
    message_chunk: Option<(usize, usize)>,
    resize_split: [f32; 2],
    resize_focus: [FocusHandle; 2],
    resize_drag: Option<(usize, Point<Pixels>, f32)>,
}

impl Extra {
    fn new(window: &mut Window, cx: &mut Context<Gallery>) -> Self {
        let calendar = cx.new(|cx| CalendarState::new(DateSelection::Single(None), window, cx));
        let date_range =
            cx.new(|cx| CalendarState::new(DateSelection::Range(None, None), window, cx));
        let query = cx.new(|cx| InputState::new(window, cx, "Search…"));
        let filter = cx.new(|cx| InputState::new(window, cx, "Filter emails…"));
        cx.observe(&calendar, |_, _, cx| cx.notify()).detach();
        cx.observe(&date_range, |_, _, cx| cx.notify()).detach();
        let command_query = cx.new(|cx| InputState::new(window, cx, "Type a command or search…"));
        let group_query = cx.new(|cx| InputState::new(window, cx, "Search…"));
        cx.subscribe(&command_query, |gallery, _, event: &InputEvent, cx| {
            if *event == InputEvent::Change {
                gallery.demos.extra.command_active = None;
                cx.notify();
            }
        })
        .detach();
        cx.observe(&query, |_, _, cx| cx.notify()).detach();
        cx.observe(&filter, |_, _, cx| cx.notify()).detach();
        Self {
            context_menu_at: None,
            context_open_with: false,
            context_pinned: false,
            context_show_extension: true,
            context_sort_by_name: true,
            code_example: code_block::Example::Rust,
            code_wrap: false,
            calendar,
            date_range,
            query,
            command_query,
            command_active: None,
            group_query,
            filter,
            choice: None,
            question: 0,
            answers: [None; 2],
            collapsed: true,
            sidebar: sidebar::SidebarDemo::default(),
            sort_descending: false,
            selected_rows: [false; 4],
            drawer_side: None,
            drawer_closing: false,
            drawer_triggers: std::array::from_fn(|_| cx.focus_handle().tab_stop(true)),
            drawer_example: extra_menus::DrawerExample::Delivery,
            drawer_trigger: 0,
            delivery_time: 0,
            rtl: false,
            attachment: true,
            trace_step: None,
            messages_scroll: SmoothScroll::default(),
            scroll_area: SmoothScroll::default(),
            code_scroll: SmoothScroll::default(),
            drawer_scroll: SmoothScroll::default(),
            discrete_scroll: false,
            drawer_drag: None,
            drawer_release: None,
            drawer_gesture: 0,
            message_chunk: None,
            resize_split: [0.5, 0.25],
            resize_focus: std::array::from_fn(|_| cx.focus_handle().tab_stop(true)),
            resize_drag: None,
        }
    }
}

pub struct Demos {
    carousel: carousel::CarouselDemo,
    example_selection: Vec<(Page, usize)>,
    extra: Extra,
    input: Entity<InputState>,
    input_examples: input::InputExamples,
    textarea: [Entity<TextareaState>; 6],
    otp: Entity<OtpState>,
    slider: Entity<SliderState>,
    checked: bool,
    switched: bool,
    radio: usize,
    toggled: [bool; 3],
    choice: usize,
    expanded: Option<usize>,
    tab: usize,
    pagination: usize,
    progress_full: bool,
    modal_focus: FocusHandle,
    modal_trigger_focus: FocusHandle,
    dialog: DialogHandle,
    alert_dialog: DialogHandle,
    sheet: DialogHandle,
    status: Option<SharedString>,
    toast: u64,
    toast_visible: bool,
}

impl Demos {
    pub fn new(window: &mut Window, cx: &mut Context<Gallery>) -> Self {
        let slider = cx.new(|cx| SliderState::new(0., 100., 1., 60., cx));
        let otp = cx.new(|cx| OtpState::new(6, window, cx));
        cx.observe(&slider, |_, _, cx| cx.notify()).detach();
        cx.observe(&otp, |_, _, cx| cx.notify()).detach();
        Self {
            carousel: carousel::CarouselDemo::new(cx),
            example_selection: Vec::new(),
            extra: Extra::new(window, cx),
            input: cx.new(|cx| InputState::new(window, cx, "you@example.com")),
            input_examples: input::InputExamples::new(window, cx),
            textarea: std::array::from_fn(|index| {
                cx.new(|cx| {
                    let placeholder = if index == 5 {
                        "اكتب رسالتك هنا."
                    } else {
                        "Type your message here."
                    };
                    let mut textarea = TextareaState::new(window, cx, placeholder);
                    textarea.set_rows(3, cx);
                    if index == 2 {
                        textarea.set_disabled(true, cx);
                    }
                    if index == 5 {
                        textarea.set_text_align(TextAlign::Right, cx);
                    }
                    textarea
                })
            }),
            otp,
            slider,
            checked: false,
            switched: true,
            radio: 1,
            toggled: [false; 3],
            choice: 0,
            expanded: Some(0),
            tab: 0,
            pagination: 1,
            progress_full: false,
            modal_focus: cx.focus_handle(),
            modal_trigger_focus: cx.focus_handle().tab_stop(true),
            dialog: DialogHandle::new(false),
            alert_dialog: DialogHandle::new(false),
            sheet: DialogHandle::new(false),
            status: None,
            toast: 0,
            toast_visible: false,
        }
    }

    pub fn preview(
        &self,
        page: Page,
        t: Theme,
        window: &mut Window,
        cx: &mut Context<Gallery>,
    ) -> Div {
        use Page::*;
        let demo = match page {
            Carousel => self
                .carousel
                .render(self.example_index(page), t, window, cx)
                .into_any_element(),
            Input => self
                .input_examples
                .render(self.example_index(page), t, window, cx)
                .into_any_element(),
            Button | ButtonGroup | Toggle | ToggleGroup | Textarea | Checkbox | Radio | Switch
            | Slider | Select | Field | InputOtp => self.controls(page, t, window, cx),
            Accordion | Avatar | Badge | Card | Empty | Kbd | Separator | Table | Typography
            | Breadcrumb | Pagination | Tabs | Alert | Progress | Skeleton | Spinner => {
                self.display(page, t, window, cx)
            }
            Dialog | AlertDialog | Dropdown | Popover | Sheet | Tooltip | Toast => {
                self.overlays(page, t, window, cx)
            }
            Calendar | DatePicker | Combobox | InputGroup | Label | Questionnaire => {
                self.extra_forms(page, t, cx)
            }
            DataTable | Chart | CodeBlock | Collapsible | Item | Sidebar | ScrollArea
            | Resizable | AspectRatio | Direction => self.extra_display(page, t, window, cx),
            Menubar | NavigationMenu | Command | ContextMenu | HoverCard | Drawer => {
                self.extra_menus(page, t, window, cx)
            }
            Attachment | Bubble | Marker | Message | MessageScroller | Trace => {
                self.messages(page, t, window, cx)
            }
        };
        if !examples::titles(page).is_empty() {
            return self.example_browser(page, demo, t, cx);
        }
        column()
            .gap(px(12.))
            .child(
                row()
                    .h(px(36.))
                    .w_full()
                    .border_b(t.borders.width)
                    .border_color(t.border)
                    .child(
                        row()
                            .h_full()
                            .px(px(12.))
                            .border_b_2()
                            .border_color(t.foreground)
                            .font_weight(FontWeight::MEDIUM)
                            .child("Preview"),
                    ),
            )
            .child(
                t.surface()
                    .w_full()
                    .min_h(px(350.))
                    .p(t.unit(28.))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(t.unit(t.typography.sm.size))
                    .line_height(t.unit(20.))
                    .child(demo),
            )
            .children(self.status.as_ref().map(|value| {
                div()
                    .text_color(t.muted_foreground)
                    .text_size(px(t.typography.xs.size))
                    .child(value.clone())
            }))
    }

    fn action<F: Fn(&mut Self) + 'static>(
        cx: &Context<Gallery>,
        change: F,
    ) -> impl Fn(&ClickEvent, &mut Window, &mut App) + 'static + use<F> {
        let entity = cx.entity().downgrade();
        move |_, _, cx| {
            let _ = entity.update(cx, |gallery, cx| {
                change(&mut gallery.demos);
                cx.notify();
            });
        }
    }

    fn change<T, F: Fn(&mut Self, T) + 'static>(
        cx: &Context<Gallery>,
        change: F,
    ) -> impl Fn(T, &ClickEvent, &mut Window, &mut App) + 'static + use<T, F> {
        let entity = cx.entity().downgrade();
        move |value, _, _, cx| {
            let _ = entity.update(cx, |gallery, cx| {
                change(&mut gallery.demos, value);
                cx.notify();
            });
        }
    }

    fn message<M: Into<SharedString>>(
        cx: &Context<Gallery>,
        message: M,
    ) -> impl Fn(&ClickEvent, &mut Window, &mut App) + 'static + use<M> {
        let message = message.into();
        Self::action(cx, move |d| d.status = Some(message.clone()))
    }

    pub fn dismiss_context_menu(&mut self, window: &mut Window, cx: &mut App) -> bool {
        if self.extra.context_menu_at.take().is_none() {
            return false;
        }
        self.modal_trigger_focus.focus(window, cx);
        true
    }

    pub fn clear_status(&mut self) {
        self.status = None;
        self.extra.context_menu_at = None;
    }
}

fn text_link(id: &'static str, label: &'static str) -> Button {
    gpui_shadcn::Button::new(id, label, Variant::Link)
        .h_auto()
        .p_0()
}
