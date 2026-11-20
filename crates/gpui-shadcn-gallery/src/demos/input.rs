use crate::{
    gallery::Gallery,
    icon,
    theme::{Theme, Variant, column, row},
};
use gpui_shadcn::gpui::{prelude::FluentBuilder, *};
use gpui_shadcn::{
    Button, Input, InputEvent, InputState, form::Field, input::field_frame, overlay::Popover,
};

const COUNTRIES: [&str; 4] = ["United States", "Canada", "United Kingdom", "Australia"];

pub(super) struct InputExamples {
    api_key: Entity<InputState>,
    basic: Entity<InputState>,
    username: Entity<InputState>,
    account: [Entity<InputState>; 2],
    disabled: Entity<InputState>,
    invalid: Entity<InputState>,
    inline: Entity<InputState>,
    names: [Entity<InputState>; 2],
    required: Entity<InputState>,
    webhook: Entity<InputState>,
    website: Entity<InputState>,
    grouped_search: Entity<InputState>,
    contact: [Entity<InputState>; 4],
    rtl_key: Entity<InputState>,
    rtl: bool,
    required_invalid: bool,
    country: usize,
    picture: Option<SharedString>,
    account_status: Option<SharedString>,
    inline_status: Option<SharedString>,
    grouped_status: Option<SharedString>,
    contact_status: Option<SharedString>,
}

impl InputExamples {
    pub(super) fn new(window: &mut Window, cx: &mut Context<Gallery>) -> Self {
        let mut input = |hint| cx.new(|cx| InputState::new(window, cx, hint));
        let examples = Self {
            api_key: input("sk-..."),
            basic: input("Enter text"),
            username: input("Enter your username"),
            account: [input("Jordan Lee"), input("name@example.com")],
            disabled: input("Email"),
            invalid: input("Error"),
            inline: input("Search..."),
            names: [input("Jordan"), input("Lee")],
            required: input("This field is required"),
            webhook: input("https://api.example.com/webhook"),
            website: input("example.com"),
            grouped_search: input("Type to search..."),
            contact: [
                input("Evil Rabbit"),
                input("john@example.com"),
                input("+1 (555) 123-4567"),
                input("123 Main St"),
            ],
            rtl_key: input("sk-..."),
            rtl: true,
            required_invalid: false,
            country: 0,
            picture: None,
            account_status: None,
            inline_status: None,
            grouped_status: None,
            contact_status: None,
        };
        examples
            .disabled
            .update(cx, |state, cx| state.set_disabled(true, cx));
        examples
            .rtl_key
            .update(cx, |state, cx| state.set_text_align(TextAlign::Right, cx));
        let _ = examples.disabled.read(cx).focus_handle(cx).tab_stop(false);
        for input in examples.editors() {
            let focus = input.read(cx).focus_handle(cx);
            cx.on_focus(&focus, window, |_, _, cx| cx.notify()).detach();
            cx.on_blur(&focus, window, |_, _, cx| cx.notify()).detach();
        }
        cx.subscribe(
            &examples.required,
            |gallery, input, event: &InputEvent, cx| {
                let demo = &mut gallery.demos.input_examples;
                match event {
                    InputEvent::PressEnter { .. } | InputEvent::Blur => {
                        demo.required_invalid = input.read(cx).value(cx).trim().is_empty();
                    }
                    InputEvent::Change if demo.required_invalid => {
                        demo.required_invalid = input.read(cx).value(cx).trim().is_empty();
                    }
                    _ => return,
                }
                cx.notify();
            },
        )
        .detach();
        for (fields, contact) in [
            (&examples.account[..], false),
            (&examples.contact[..], true),
        ] {
            for input in fields {
                cx.subscribe(input, move |gallery, _, event: &InputEvent, cx| {
                    if matches!(event, InputEvent::PressEnter { .. }) {
                        gallery.demos.input_examples.submit(contact, cx);
                        cx.notify();
                    }
                })
                .detach();
            }
        }
        for (input, grouped) in [(&examples.inline, false), (&examples.grouped_search, true)] {
            cx.subscribe(input, move |gallery, _, event: &InputEvent, cx| {
                if matches!(event, InputEvent::PressEnter { .. }) {
                    gallery.demos.input_examples.search(grouped, cx);
                    cx.notify();
                }
            })
            .detach();
        }
        examples
    }

    fn editors(&self) -> impl Iterator<Item = &Entity<InputState>> {
        [
            &self.api_key,
            &self.basic,
            &self.username,
            &self.disabled,
            &self.invalid,
            &self.inline,
            &self.required,
            &self.webhook,
            &self.website,
            &self.grouped_search,
            &self.rtl_key,
        ]
        .into_iter()
        .chain(&self.account)
        .chain(&self.names)
        .chain(&self.contact)
    }

    pub(super) fn render(
        &self,
        selected: usize,
        t: Theme,
        window: &Window,
        cx: &Context<Gallery>,
    ) -> Div {
        let editor = |id: &'static str, input| Input::field(id, input, false, window, cx);
        let field = |label: &'static str, description: &'static str, input| {
            labeled(div().child(label), description, editor(label, input))
        };
        let account = column()
            .gap(t.unit(20.))
            .child(field("Name", "", &self.account[0]))
            .child(field(
                "Email",
                "We'll send updates to this address.",
                &self.account[1],
            ))
            .child(
                row()
                    .gap(t.unit(8.))
                    .child(
                        button(t, "account-reset", "Reset", Variant::Outline).on_click(
                            cx.listener(|g, _, window, cx| {
                                let demo = &mut g.demos.input_examples;
                                clear(&demo.account, window, cx);
                                demo.account_status = None;
                                cx.notify();
                            }),
                        ),
                    )
                    .child(
                        button(t, "account-submit", "Submit", Variant::Primary).on_click(
                            cx.listener(|g, _, _, cx| {
                                g.demos.input_examples.submit(false, cx);
                                cx.notify();
                            }),
                        ),
                    ),
            )
            .children(feedback(t, &self.account_status));

        let content = match selected {
            0 => div().child(editor("basic", &self.basic)),
            1 => field("API Key", "Enter the key for your account.", &self.api_key),
            2 => field(
                "Username",
                "Choose a username for your account.",
                &self.username,
            ),
            3 => account,
            4 => labeled(
                div().child("Email"),
                "This field is currently disabled.",
                editor("disabled", &self.disabled),
            )
            .opacity(0.5),
            5 => labeled(
                div().child("Invalid Input"),
                "",
                field_frame("invalid", &self.invalid, true, window, cx)
                    .child(Input::new(&self.invalid)),
            )
            .child(
                div()
                    .text_color(t.destructive)
                    .child("Enter a valid value."),
            ),
            6 => labeled(
                div().child("Picture"),
                "Select a picture to upload.",
                self.file(t, cx),
            ),
            7 => self.search_row(false, t, window, cx),
            8 => row()
                .gap(t.unit(12.))
                .child(field("First Name", "", &self.names[0]).flex_1().min_w_0())
                .child(field("Last Name", "", &self.names[1]).flex_1().min_w_0()),
            9 => labeled(
                div().child("Required Field *"),
                "This field must be filled out.",
                Input::field(
                    "required",
                    &self.required,
                    self.required_invalid,
                    window,
                    cx,
                ),
            ),
            10 => labeled(
                row().gap(t.unit(8.)).child("Webhook URL").child(
                    row()
                        .rounded_full()
                        .px(t.unit(7.))
                        .bg(t.muted)
                        .text_size(t.unit(t.typography.xs.size))
                        .child("Beta"),
                ),
                "",
                editor("webhook", &self.webhook),
            ),
            11 => labeled(
                div().child("Website URL"),
                "",
                field_frame("website", &self.website, false, window, cx)
                    .flex_row()
                    .items_center()
                    .gap(t.unit(8.))
                    .child(div().text_color(t.muted_foreground).child("https://"))
                    .child(div().flex_1().min_w_0().child(Input::new(&self.website)))
                    .child(
                        Popover::new("website-hint")
                            .offset(px(4.))
                            .trigger(
                                Button::unstyled("website-info")
                                    .accessibility_label("Website URL help")
                                    .child(icon("circle-info", t.unit(16.), t.muted_foreground)),
                            )
                            .content(move |_, _, _| {
                                t.surface().p(t.unit(6.)).child("Enter a website domain.")
                            }),
                    ),
            ),
            12 => self.search_row(true, t, window, cx),
            13 => column()
                .gap(t.unit(16.))
                .child(labeled(
                    div().child("Name"),
                    "",
                    editor("contact-name", &self.contact[0]),
                ))
                .child(labeled(
                    div().child("Email"),
                    "Used to contact you about this form.",
                    editor("contact-email", &self.contact[1]),
                ))
                .child(field("Phone", "", &self.contact[2]))
                .child(labeled(div().child("Country"), "", self.country_select(cx)))
                .child(field("Address", "", &self.contact[3]))
                .child(
                    row()
                        .justify_end()
                        .gap(t.unit(8.))
                        .child(
                            button(t, "contact-cancel", "Cancel", Variant::Outline).on_click(
                                cx.listener(|g, _, window, cx| {
                                    let demo = &mut g.demos.input_examples;
                                    clear(&demo.contact, window, cx);
                                    demo.country = 0;
                                    demo.contact_status = None;
                                    cx.notify();
                                }),
                            ),
                        )
                        .child(
                            button(t, "contact-submit", "Submit", Variant::Primary).on_click(
                                cx.listener(|g, _, _, cx| {
                                    let demo = &mut g.demos.input_examples;
                                    demo.contact_status = Some(form_result(&demo.contact[1], cx));
                                    cx.notify();
                                }),
                            ),
                        ),
                )
                .children(feedback(t, &self.contact_status)),
            14 => column()
                .gap(t.unit(12.))
                .child(
                    button(
                        t,
                        "input-direction",
                        if self.rtl {
                            "Arabic (العربية)"
                        } else {
                            "English"
                        },
                        Variant::Outline,
                    )
                    .self_start()
                    .on_click(cx.listener(|g, _, _, cx| {
                        let demo = &mut g.demos.input_examples;
                        demo.rtl = !demo.rtl;
                        demo.rtl_key.update(cx, |state, cx| {
                            state.set_text_align(
                                if demo.rtl {
                                    TextAlign::Right
                                } else {
                                    TextAlign::Left
                                },
                                cx,
                            );
                        });
                        cx.notify();
                    })),
                )
                .child(
                    labeled(
                        div().child(if self.rtl {
                            "مفتاح API"
                        } else {
                            "API Key"
                        }),
                        if self.rtl {
                            "أدخل مفتاح حسابك."
                        } else {
                            "Enter the key for your account."
                        },
                        editor("rtl-key", &self.rtl_key),
                    )
                    .when(self.rtl, |s| s.text_right()),
                ),
            _ => unreachable!("unknown input example"),
        };
        column().w_full().max_w(t.unit(400.)).child(content)
    }

    fn search_row(&self, grouped: bool, t: Theme, window: &Window, cx: &Context<Gallery>) -> Div {
        let (id, input, status) = if grouped {
            ("grouped-search", &self.grouped_search, &self.grouped_status)
        } else {
            ("inline-search", &self.inline, &self.inline_status)
        };
        let button_id = if grouped {
            "grouped-submit"
        } else {
            "inline-submit"
        };
        let controls = row()
            .when(!grouped, |s| s.gap(t.unit(8.)))
            .child(
                field_frame(id, input, false, window, cx)
                    .flex_1()
                    .when(grouped, |s| s.rounded_r_none())
                    .child(Input::new(input)),
            )
            .child(
                button(
                    t,
                    button_id,
                    "Search",
                    if grouped {
                        Variant::Outline
                    } else {
                        Variant::Primary
                    },
                )
                .flex_shrink_0()
                .when(grouped, |s| s.rounded_l_none().border_l_0())
                .on_click(cx.listener(move |g, _, _, cx| {
                    g.demos.input_examples.search(grouped, cx);
                    cx.notify();
                })),
            );
        if grouped {
            labeled(div().child("Search"), "", controls)
        } else {
            column().child(controls)
        }
        .gap(t.unit(8.))
        .children(feedback(t, status))
    }

    fn submit(&mut self, contact: bool, cx: &App) {
        let (email, status) = if contact {
            (&self.contact[1], &mut self.contact_status)
        } else {
            (&self.account[1], &mut self.account_status)
        };
        *status = Some(form_result(email, cx));
    }

    fn search(&mut self, grouped: bool, cx: &App) {
        let (input, status) = if grouped {
            (&self.grouped_search, &mut self.grouped_status)
        } else {
            (&self.inline, &mut self.inline_status)
        };
        let value = input.read(cx).value(cx);
        *status = Some(if value.trim().is_empty() {
            "Enter a search term.".into()
        } else {
            format!("Search: {}", value.trim()).into()
        });
    }

    fn file(&self, t: Theme, cx: &Context<Gallery>) -> Button {
        button(t, "input-file", "", Variant::Outline)
            .w_full()
            .gap(t.unit(10.))
            .justify_start()
            .accessibility_label("Choose picture file")
            .child("Choose File")
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .truncate()
                    .text_color(t.muted_foreground)
                    .child(
                        self.picture
                            .clone()
                            .unwrap_or_else(|| "No file chosen".into()),
                    ),
            )
            .on_click(cx.listener(|_, _, _, cx| {
                let selection = cx.prompt_for_paths(PathPromptOptions {
                    files: true,
                    directories: false,
                    multiple: false,
                    prompt: Some("Choose a picture".into()),
                });
                cx.spawn(async move |gallery, cx| {
                    if let Ok(Ok(Some(paths))) = selection.await
                        && let Some(name) = paths.first().and_then(|path| path.file_name())
                    {
                        let name = name.to_string_lossy().into_owned();
                        let _ = gallery.update(cx, |g, cx| {
                            g.demos.input_examples.picture = Some(name.into());
                            cx.notify();
                        });
                    }
                })
                .detach();
            }))
    }

    fn country_select(&self, cx: &Context<Gallery>) -> impl IntoElement {
        let gallery = cx.entity().downgrade();
        gpui_shadcn::Select::new("input-country", COUNTRIES, self.country)
            .accessibility_label("Country")
            .w_full()
            .on_change(move |index, _, cx| {
                let _ = gallery.update(cx, |g, cx| {
                    g.demos.input_examples.country = index;
                    cx.notify();
                });
            })
    }
}

fn labeled(label: Div, description: &'static str, control: impl IntoElement) -> Div {
    column()
        .w_full()
        .child(Field::new(label, description, control))
}

fn button(t: Theme, id: &'static str, label: &'static str, variant: Variant) -> Button {
    gpui_shadcn::Button::new(id, label, variant).h(t.unit(32.))
}

fn feedback(t: Theme, status: &Option<SharedString>) -> Option<Div> {
    status
        .as_ref()
        .map(|status| div().text_color(t.muted_foreground).child(status.clone()))
}

fn clear(inputs: &[Entity<InputState>], window: &mut Window, cx: &mut App) {
    for input in inputs {
        input.update(cx, |state, cx| state.set_value("", window, cx));
    }
}

fn form_result(email: &Entity<InputState>, cx: &App) -> SharedString {
    let email = email.read(cx).value(cx);
    if email.is_empty()
        || email.split_once('@').is_some_and(|(name, host)| {
            !name.is_empty()
                && !host.is_empty()
                && !host.contains('@')
                && !email.contains(char::is_whitespace)
        })
    {
        "Form submitted.".into()
    } else {
        "Enter a valid email address.".into()
    }
}
