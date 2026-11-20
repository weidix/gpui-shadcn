use std::rc::Rc;

use gpui::{prelude::FluentBuilder, *};
use gpui_base::{
    StyledExt,
    actions::{Confirm, SelectDown, SelectUp},
};

use crate::{Theme, Variant, icon, motion, theme::column};
use gpui_base::Popover;

type ChangeHandler = Rc<dyn Fn(usize, &mut Window, &mut App)>;

struct SelectState {
    open: bool,
    open_generation: usize,
    highlighted: usize,
    keyboard_navigation: bool,
    width: Pixels,
    trigger_focus: FocusHandle,
    list_focus: FocusHandle,
    scroll: ScrollHandle,
}

/// A single-choice select whose selected option aligns over its trigger.
#[derive(IntoElement)]
pub struct Select {
    id: ElementId,
    theme: Option<Theme>,
    options: Rc<Vec<SharedString>>,
    selected: usize,
    label: SharedString,
    style: StyleRefinement,
    on_change: Option<ChangeHandler>,
}

impl Select {
    pub fn new(
        id: impl Into<ElementId>,
        options: impl IntoIterator<Item = impl Into<SharedString>>,
        selected: usize,
    ) -> Self {
        Self {
            id: id.into(),
            theme: None,
            options: Rc::new(options.into_iter().map(Into::into).collect()),
            selected,
            label: "Select".into(),
            style: StyleRefinement::default(),
            on_change: None,
        }
    }

    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = Some(theme);
        self
    }

    pub fn accessibility_label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = label.into();
        self
    }

    pub fn on_change(mut self, handler: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }
}

impl Styled for Select {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

fn change_open(
    state: &Entity<SelectState>,
    open: bool,
    selected: usize,
    start: usize,
    row_height: Pixels,
    window: &mut Window,
    cx: &mut App,
) {
    let keyboard_navigation = window.last_input_was_keyboard();
    state.update(cx, |state, cx| {
        if open && !state.open {
            state.open_generation = state.open_generation.wrapping_add(1);
            state.highlighted = selected;
            state.keyboard_navigation = keyboard_navigation;
            state
                .scroll
                .set_offset(point(px(0.), -(row_height * start as f32)));
        }
        state.open = open;
        cx.notify();
    });
    window.refresh();
}

fn commit(
    state: &Entity<SelectState>,
    index: usize,
    handler: &Option<ChangeHandler>,
    window: &mut Window,
    cx: &mut App,
) {
    state.update(cx, |state, cx| {
        state.open = false;
        state.trigger_focus.focus(window, cx);
        cx.notify();
    });
    if let Some(handler) = handler {
        handler(index, window, cx);
    }
    window.refresh();
}

impl RenderOnce for Select {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = self.theme.unwrap_or_else(|| Theme::global(cx));
        let count = self.options.len();
        let selected = self.selected.min(count.saturating_sub(1));
        let state =
            window.use_keyed_state((self.id.clone(), "select-state"), cx, |_, cx| SelectState {
                open: false,
                open_generation: 0,
                highlighted: selected,
                keyboard_navigation: false,
                width: px(200.),
                trigger_focus: cx.focus_handle().tab_stop(true),
                list_focus: cx.focus_handle(),
                scroll: ScrollHandle::default(),
            });
        let row_height = t.unit(28.);
        let trigger_height = t.unit(32.);
        let inset = t.unit(4.) + px(1.);
        let rows = ((window.viewport_size().height - px(32.) - inset * 2.) / row_height)
            .floor()
            .clamp(1., 10.) as usize;
        let visible = count.min(rows);
        let start = selected
            .saturating_sub(visible / 2)
            .min(count.saturating_sub(visible));
        let offset =
            -(trigger_height + row_height) / 2. - inset - row_height * (selected - start) as f32;
        let open = state.read(cx).open;
        let open_generation = state.read(cx).open_generation;
        let highlighted = state.read(cx).highlighted.min(count.saturating_sub(1));
        let keyboard_navigation = state.read(cx).keyboard_navigation;
        let trigger_focus = state.read(cx).trigger_focus.clone();
        let list_focus = state.read(cx).list_focus.clone();
        let root_list_focus = list_focus.clone();
        let width = state.read(cx).width;
        let scroll = state.read(cx).scroll.clone();
        let value = self
            .options
            .get(selected)
            .cloned()
            .unwrap_or_else(|| "Select…".into());
        let measure_state = state.clone();
        let trigger = crate::Button::with_content((self.id.clone(), "trigger"), Variant::Outline)
            .with_theme(t)
            .accessibility_label(self.label.clone())
            .track_focus(&trigger_focus)
            .disabled(count == 0)
            .relative()
            .w_full()
            .h(trigger_height)
            .rounded(t.input_radius())
            .px(t.unit(10.))
            .justify_between()
            .child(div().min_w_0().truncate().child(value.clone()))
            .child(icon("chevron-down", t.unit(14.), t.muted_foreground))
            .child(
                canvas(
                    move |bounds, window, cx| {
                        if measure_state.read(cx).width != bounds.size.width {
                            measure_state.update(cx, |state, cx| {
                                state.width = bounds.size.width;
                                cx.notify();
                            });
                            window.refresh();
                        }
                    },
                    |_, _, _, _| {},
                )
                .absolute()
                .top_0()
                .left_0()
                .size_full(),
            );

        let root_state = state.clone();
        let popup_state = state.clone();
        let confirm_state = state.clone();
        let confirm_handler = self.on_change.clone();
        let options = self.options;
        let handler = self.on_change;
        let popup = Popover::new((self.id.clone(), "popup"))
            .w_full()
            .open(open && count > 0)
            .track_focus(&list_focus)
            .offset(offset)
            .on_open_change(move |open, window, cx| {
                change_open(&popup_state, *open, selected, start, row_height, window, cx);
            })
            .trigger_with(move |open, _, _| {
                let selected = trigger.is_selected();
                trigger.selected(selected || open).into_any_element()
            })
            .content(move |_, _, popup_cx| {
                let down_state = state.clone();
                let up_state = state.clone();
                let key_state = state.clone();
                let key_options = options.clone();
                let enter_state = state.clone();
                let enter_handler = handler.clone();
                let panel = t.popup_surface().w(width).p(t.unit(4.)).child(
                    column()
                        .id("select-options")
                        .role(Role::ListBox)
                        .track_focus(&list_focus)
                        .key_context("Select")
                        .h(row_height * visible as f32)
                        .overflow_y_scroll()
                        .track_scroll(&scroll)
                        .on_action(move |_: &SelectDown, window, cx| {
                            down_state.update(cx, |state, cx| {
                                state.keyboard_navigation = true;
                                state.highlighted = (state.highlighted + 1).min(count - 1);
                                state.scroll.scroll_to_item(state.highlighted);
                                cx.notify();
                            });
                            window.refresh();
                        })
                        .on_action(move |_: &SelectUp, window, cx| {
                            up_state.update(cx, |state, cx| {
                                state.keyboard_navigation = true;
                                state.highlighted = state.highlighted.saturating_sub(1);
                                state.scroll.scroll_to_item(state.highlighted);
                                cx.notify();
                            });
                            window.refresh();
                        })
                        .on_action(move |_: &Confirm, window, cx| {
                            let index = enter_state.read(cx).highlighted;
                            commit(&enter_state, index, &enter_handler, window, cx);
                        })
                        .on_key_down(move |event, window, cx| {
                            let key = &event.keystroke.key;
                            let current = key_state.read(cx).highlighted;
                            let next = match key.as_str() {
                                "home" => Some(0),
                                "end" => Some(count - 1),
                                _ if key.chars().count() == 1
                                    && !event.keystroke.modifiers.control
                                    && !event.keystroke.modifiers.platform =>
                                {
                                    (1..=count).map(|step| (current + step) % count).find(
                                        |&index| {
                                            key_options[index]
                                                .to_lowercase()
                                                .starts_with(&key.to_lowercase())
                                        },
                                    )
                                }
                                _ => None,
                            };
                            if let Some(index) = next {
                                key_state.update(cx, |state, cx| {
                                    state.keyboard_navigation = true;
                                    state.highlighted = index;
                                    state.scroll.scroll_to_item(index);
                                    cx.notify();
                                });
                                cx.stop_propagation();
                                window.refresh();
                            }
                        })
                        .children(options.iter().enumerate().map(|(index, label)| {
                            let state = state.clone();
                            let handler = handler.clone();
                            let hover_state = state.clone();
                            let hover_background = if keyboard_navigation && index != highlighted {
                                t.background.opacity(0.)
                            } else {
                                t.muted
                            };
                            crate::Button::with_content(("select-option", index), Variant::Ghost)
                                .with_theme(t)
                                .hover_background(hover_background)
                                .accessibility_label(label.clone())
                                .child(label.clone())
                                .role(Role::ListBoxOption)
                                .focusable(false)
                                .aria_selected(index == selected)
                                .when(keyboard_navigation && index == highlighted, |s| {
                                    s.aria_active_descendant().bg(t.muted)
                                })
                                .on_hover(move |hovered, window, cx| {
                                    if *hovered {
                                        hover_state.update(cx, |state, cx| {
                                            state.keyboard_navigation = false;
                                            state.highlighted = index;
                                            cx.notify();
                                        });
                                        window.refresh();
                                    }
                                })
                                .h(row_height)
                                .flex_shrink_0()
                                .w_full()
                                .px(t.unit(6.))
                                .rounded(t.popup_item_radius())
                                .justify_between()
                                .child(
                                    div().w(t.unit(16.)).children(
                                        (index == selected)
                                            .then(|| icon("check", t.unit(14.), t.foreground)),
                                    ),
                                )
                                .on_click(move |_, window, cx| {
                                    commit(&state, index, &handler, window, cx)
                                })
                        })),
                );
                motion::fade_in(
                    panel,
                    ("select-enter", open_generation),
                    t.motion.fast,
                    popup_cx,
                )
            });
        gpui_base::Select::new(self.id)
            .w(px(200.))
            .refine_style(&self.style)
            .open(open)
            .disabled(count == 0)
            .content_focus_handle(&root_list_focus)
            .accessibility_label(self.label)
            .accessibility_value(value)
            .on_open_change(move |open, window, cx| {
                change_open(&root_state, open, selected, start, row_height, window, cx);
            })
            .on_confirm(move |window, cx| {
                let index = confirm_state.read(cx).highlighted;
                commit(&confirm_state, index, &confirm_handler, window, cx);
            })
            .child(popup)
    }
}
