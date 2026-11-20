use super::Demos;
use crate::{
    catalog::Page,
    gallery::Gallery,
    icon,
    theme::{Theme, Variant, column, row},
};
use gpui_shadcn::gpui::{prelude::FluentBuilder, *};
use gpui_shadcn::{
    Button, Input,
    calendar::{Calendar, CalendarState},
    overlay::Popover,
};

fn calendar(t: Theme, state: &Entity<CalendarState>, months: usize, popup: bool) -> Div {
    let surface = if popup {
        t.popup_surface()
    } else {
        t.surface()
    };
    surface
        .p(t.unit(12.))
        .child(Calendar::new("calendar-grid", state).months(months))
}

impl Demos {
    pub(super) fn extra_forms(
        &self,
        page: Page,
        t: Theme,
        cx: &mut Context<Gallery>,
    ) -> AnyElement {
        let state = &self.extra;
        match page {
            Page::Calendar => calendar(t, &state.calendar, 1, false).into_any_element(),
            Page::DatePicker => {
                let chosen = state.calendar.read(cx).date(cx).format("%b %d, %Y");
                let single_state = state.calendar.clone();
                let single = Popover::new("date-picker")
                    .offset(px(4.))
                    .trigger(gpui_shadcn::Button::new(
                        "pick-date",
                        chosen.unwrap_or_else(|| "Pick a date".into()),
                        Variant::Outline,
                    ))
                    .content(move |_, _, _| calendar(t, &single_state, 1, true));

                let range_state = state.date_range.clone();
                let range = range_state.read(cx).date(cx);
                let label = range.format("%b %d, %Y").unwrap_or_else(|| {
                    range.start().map_or_else(
                        || "Pick a date range".into(),
                        |start| format!("{} – Select end date", start.format("%b %d, %Y")).into(),
                    )
                });
                let range = Popover::new("date-range-picker")
                    .offset(px(4.))
                    .trigger(
                        gpui_shadcn::Button::new("pick-date-range", label, Variant::Outline)
                            .min_w(t.unit(248.))
                            .justify_start(),
                    )
                    .content(move |_, window, _| {
                        let months = if window.viewport_size().width >= t.unit(528.) {
                            2
                        } else {
                            1
                        };
                        calendar(t, &range_state, months, true)
                    });

                row()
                    .justify_center()
                    .flex_wrap()
                    .gap(t.unit(20.))
                    .child(column().gap(t.unit(8.)).child("Date").child(single))
                    .child(column().gap(t.unit(8.)).child("Date range").child(range))
                    .into_any_element()
            }
            Page::Combobox => {
                let query = state.query.clone();
                let gallery = cx.entity().downgrade();
                let selected = state.choice;
                let focus_query = query.clone();
                Popover::new("framework-picker")
                    .offset(px(4.))
                    .on_open_change(move |open, window, cx| {
                        if *open {
                            focus_query.read(cx).focus_handle(cx).focus(window, cx);
                        }
                    })
                    .trigger(
                        gpui_shadcn::Button::new(
                            "framework",
                            selected.unwrap_or("Select framework…"),
                            Variant::Outline,
                        )
                        .w(t.unit(224.))
                        .justify_between()
                        .child(icon(
                            "chevron-down",
                            t.unit(14.),
                            t.muted_foreground,
                        )),
                    )
                    .content(move |popover, _, pcx| {
                        let value = query.read(pcx).value(pcx).to_lowercase();
                        let matches = ["Next.js", "SvelteKit", "Nuxt", "Remix", "Astro"]
                            .into_iter()
                            .filter(|label| label.to_lowercase().contains(&value))
                            .collect::<Vec<_>>();
                        t.popup_surface()
                            .rounded(t.radius())
                            .w(t.unit(224.))
                            .p(t.unit(8.))
                            .child(
                                column()
                                    .gap(t.unit(4.))
                                    .child(Input::search("framework-search", &query, pcx))
                                    .when(matches.is_empty(), |s| {
                                        s.child(div().p(t.unit(8.)).child("No results found."))
                                    })
                                    .children(matches.into_iter().map(|label| {
                                        let gallery = gallery.clone();
                                        let popover = popover.clone();
                                        gpui_shadcn::Button::new(label, label, Variant::Ghost)
                                            .h(t.unit(32.))
                                            .rounded(t.popup_item_radius())
                                            .justify_between()
                                            .when(selected == Some(label), |s| {
                                                s.child(icon("check", t.unit(14.), t.foreground))
                                            })
                                            .on_click(move |_, window, cx| {
                                                let _ = gallery.update(cx, |g, cx| {
                                                    g.demos.extra.choice = Some(label);
                                                    cx.notify();
                                                });
                                                popover.dismiss(window, cx);
                                            })
                                    })),
                            )
                    })
                    .into_any_element()
            }
            Page::InputGroup => {
                let query = state.group_query.clone();
                let submit = cx.listener(move |g, _, _, cx| {
                    let value = query.read(cx).value(cx);
                    g.demos.status = Some(format!("Search: {}", value.trim()).into());
                    cx.notify();
                });
                row()
                    .w_full()
                    .max_w(t.unit(330.))
                    .gap(t.unit(8.))
                    .px(t.unit(8.))
                    .h(t.unit(38.))
                    .border(t.borders.width)
                    .border_color(t.input)
                    .rounded(t.input_radius())
                    .child(icon("search", t.unit(15.), t.muted_foreground))
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .child(Input::new(&state.group_query)),
                    )
                    .child(
                        gpui_shadcn::Button::new("submit-search", "Search", Variant::Ghost)
                            .h(t.unit(30.))
                            .on_click(submit),
                    )
                    .into_any_element()
            }
            Page::Label => {
                let focus = self.input.read(cx).focus_handle(cx);
                column()
                    .w_full()
                    .max_w(px(320.))
                    .gap(t.unit(8.))
                    .child(
                        gpui_shadcn::form::label(Button::unstyled("email-label"), cx)
                            .accessibility_label("Focus email input")
                            .flex()
                            .items_center()
                            .self_start()
                            .cursor_pointer()
                            .child("Email")
                            .on_click(move |_, window, cx| focus.focus(window, cx)),
                    )
                    .child(
                        gpui_shadcn::Input::frame("field-frame", cx).child(Input::new(&self.input)),
                    )
                    .into_any_element()
            }
            Page::Questionnaire => {
                let step = state.question;
                let question = ["What are you building?", "Which platform comes first?"][step];
                let options = [
                    ["Desktop app", "Internal tool", "Prototype"],
                    ["macOS", "Windows", "Linux"],
                ][step];
                t.surface()
                    .w_full()
                    .max_w(t.unit(360.))
                    .p(t.unit(16.))
                    .child(
                        column()
                            .gap(t.unit(8.))
                            .child(
                                div()
                                    .text_color(t.muted_foreground)
                                    .child(format!("Question {} of 2", step + 1)),
                            )
                            .child(div().font_weight(FontWeight::SEMIBOLD).child(question))
                            .children(options.map(|option| {
                                gpui_shadcn::Button::new(
                                    option,
                                    option,
                                    if state.answers[step] == Some(option) {
                                        Variant::Primary
                                    } else {
                                        Variant::Outline
                                    },
                                )
                                .justify_start()
                                .on_click(Self::action(cx, move |d| {
                                    d.extra.answers[step] = Some(option)
                                }))
                            }))
                            .child(
                                row()
                                    .justify_between()
                                    .child(
                                        gpui_shadcn::Button::new(
                                            "previous-question",
                                            "Back",
                                            Variant::Ghost,
                                        )
                                        .disabled(step == 0)
                                        .on_click(Self::action(cx, |d| d.extra.question = 0)),
                                    )
                                    .child(
                                        gpui_shadcn::Button::new(
                                            "next-question",
                                            if step == 0 { "Continue" } else { "Submit" },
                                            Variant::Primary,
                                        )
                                        .disabled(state.answers[step].is_none())
                                        .on_click(
                                            Self::action(cx, move |d| {
                                                if step == 0 {
                                                    d.extra.question = 1;
                                                } else if let [Some(project), Some(platform)] =
                                                    d.extra.answers
                                                {
                                                    d.status = Some(
                                                        format!("{project} for {platform}").into(),
                                                    );
                                                }
                                            }),
                                        ),
                                    ),
                            ),
                    )
                    .into_any_element()
            }
            _ => unreachable!(),
        }
    }
}
