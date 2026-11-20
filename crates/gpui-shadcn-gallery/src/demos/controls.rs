use super::{Demos, text_link};
use crate::{
    catalog::Page,
    gallery::Gallery,
    icon,
    theme::{Theme, Variant, column, row},
};
use gpui_shadcn::gpui::{prelude::FluentBuilder, *};
use gpui_shadcn::{
    CheckboxState,
    form::{self, OtpInput, RadioGroup},
    input,
};

impl Demos {
    pub(super) fn controls(
        &self,
        page: Page,
        t: Theme,
        window: &Window,
        cx: &mut Context<Gallery>,
    ) -> AnyElement {
        use Page::*;
        let variants = [
            ("Default", Variant::Primary),
            ("Secondary", Variant::Secondary),
            ("Outline", Variant::Outline),
            ("Ghost", Variant::Ghost),
            ("Destructive", Variant::Destructive),
            ("Link", Variant::Link),
        ];
        match page {
            Button => {
                let selected = self.example_index(page);
                match selected {
                    0..=5 => {
                        let (label, variant) = variants[selected];
                        gpui_shadcn::Button::new(label, label, variant)
                            .on_click(Self::message(cx, format!("{label} button clicked.")))
                            .into_any_element()
                    }
                    6 => row()
                        .gap(t.unit(12.))
                        .child(
                            gpui_shadcn::Button::new("small", "Small", Variant::Outline)
                                .h(t.unit(32.))
                                .px(t.unit(10.))
                                .on_click(Self::message(cx, "Small button clicked.")),
                        )
                        .child(
                            gpui_shadcn::Button::new("regular", "Default", Variant::Outline)
                                .on_click(Self::message(cx, "Default button clicked.")),
                        )
                        .child(
                            gpui_shadcn::Button::new("large", "Large", Variant::Outline)
                                .h(t.unit(40.))
                                .px(t.unit(18.))
                                .on_click(Self::message(cx, "Large button clicked.")),
                        )
                        .into_any_element(),
                    7 => gpui_shadcn::Button::new("disabled", "Disabled", Variant::Outline)
                        .disabled(true)
                        .into_any_element(),
                    8 => row()
                        .gap(t.unit(12.))
                        .child(
                            gpui_shadcn::Button::icon(
                                "icon-search",
                                "Search",
                                "search",
                                Variant::Outline,
                                32.,
                            )
                            .on_click(Self::message(cx, "Search icon clicked.")),
                        )
                        .child(
                            gpui_shadcn::Button::icon(
                                "icon-next",
                                "Next",
                                "arrow-right",
                                Variant::Primary,
                                36.,
                            )
                            .on_click(Self::message(cx, "Next icon clicked.")),
                        )
                        .child(
                            gpui_shadcn::Button::icon(
                                "icon-settings",
                                "Settings",
                                "sliders-horizontal",
                                Variant::Ghost,
                                40.,
                            )
                            .on_click(Self::message(cx, "Settings icon clicked.")),
                        )
                        .child(
                            gpui_shadcn::Button::icon(
                                "icon-disabled",
                                "Remove (disabled)",
                                "x",
                                Variant::Outline,
                                36.,
                            )
                            .disabled(true),
                        )
                        .into_any_element(),
                    9 => {
                        gpui_shadcn::Button::new("continue-with-icon", "Continue", Variant::Outline)
                            .child(icon("arrow-right", t.unit(16.), t.foreground))
                            .on_click(Self::message(cx, "Continue clicked."))
                            .into_any_element()
                    }
                    _ => unreachable!("unknown button example"),
                }
            }
            ButtonGroup => {
                let labels = ["Archive", "Report", "Snooze"];
                let radius = t.radius_control();
                row()
                    .id("button-group")
                    .role(Role::Group)
                    .aria_label("Message actions")
                    .children(labels.into_iter().enumerate().map(|(i, label)| {
                        gpui_shadcn::Button::new(label, label, Variant::Outline)
                            .rounded_none()
                            .when(i == 0, |button| button.rounded_l(radius))
                            .when(i > 0, |button| button.border_l_0())
                            .when(i == labels.len() - 1, |button| button.rounded_r(radius))
                            .on_click(Self::message(cx, format!("{label} selected.")))
                    }))
                    .into_any_element()
            }
            Toggle => gpui_shadcn::Toggle::new("bold")
                .pressed(self.toggled[0])
                .child("Bold")
                .on_change(Self::change(cx, |d, pressed| d.toggled[0] = pressed))
                .into_any_element(),
            ToggleGroup => {
                let example = self.example_index(page);
                let items = match example {
                    6 => ["Left", "Center", "Right"],
                    7 => ["عريض", "مائل", "تسطير"],
                    _ => ["Bold", "Italic", "Underline"],
                };
                form::ToggleGroup::new("formatting")
                    .axis(if example == 4 {
                        Axis::Vertical
                    } else {
                        Axis::Horizontal
                    })
                    .when(example == 3, |group| group.gap(t.unit(12.)))
                    .when(example == 7, |group| group.flex_row_reverse())
                    .children(items.into_iter().enumerate().map(|(i, label)| {
                        gpui_shadcn::Toggle::new(("format", i))
                            .pressed(self.toggled[i])
                            .disabled(example == 5 && i == 1)
                            .when(matches!(example, 0 | 3 | 4 | 5 | 6 | 7), |toggle| {
                                toggle.border_0()
                            })
                            .when(example == 2, |toggle| {
                                let height = [28., 36., 44.][i];
                                toggle.h(t.unit(height))
                            })
                            .child(label)
                            .on_change(Self::change(cx, move |d, pressed| {
                                if example == 6 {
                                    d.toggled = [false; 3];
                                }
                                d.toggled[i] = pressed;
                            }))
                    }))
                    .into_any_element()
            }
            Field => form::Field::new(
                div().child("Email"),
                "We’ll use this address to send you account updates.",
                gpui_shadcn::Input::frame("field-frame", cx).child(input::Input::new(&self.input)),
            )
            .max_w(px(320.))
            .into_any_element(),
            Textarea => {
                let example = self.example_index(page);
                let state = &self.textarea[example];
                let editor = input::Textarea::field("message", state, example == 3, window, cx);
                let control = match example {
                    1 => form::Field::new(
                        div().child("Message"),
                        "Your message will be sent to the support team.",
                        editor,
                    )
                    .into_any_element(),
                    2 => form::Field::new(div().child("Message"), "", editor)
                        .opacity(t.states.disabled)
                        .into_any_element(),
                    3 => column()
                        .gap(t.unit(8.))
                        .child(form::Field::new(div().child("Message"), "", editor))
                        .child(div().text_color(t.destructive).child("Enter a message."))
                        .into_any_element(),
                    4 => column()
                        .gap(t.unit(8.))
                        .child(editor)
                        .child(
                            gpui_shadcn::Button::new(
                                "send-message",
                                "Send message",
                                Variant::Primary,
                            )
                            .self_end()
                            .on_click(cx.listener(
                                |gallery, _, _, cx| {
                                    let text = gallery.demos.textarea[4].read(cx).value(cx);
                                    gallery.demos.status = Some(if text.trim().is_empty() {
                                        "Enter a message.".into()
                                    } else {
                                        "Message sent.".into()
                                    });
                                    cx.notify();
                                },
                            )),
                        )
                        .into_any_element(),
                    5 => form::Field::new(div().child("الرسالة"), "", editor)
                        .text_right()
                        .into_any_element(),
                    _ => editor.into_any_element(),
                };
                div()
                    .w_full()
                    .max_w(t.unit(320.))
                    .child(control)
                    .into_any_element()
            }
            Checkbox => gpui_shadcn::Checkbox::new("terms")
                .checked(self.checked)
                .on_change(Self::change(cx, |d, state| {
                    d.checked = matches!(state, CheckboxState::Checked)
                }))
                .child(
                    column()
                        .gap(t.unit(3.))
                        .child(
                            div()
                                .font_weight(FontWeight::MEDIUM)
                                .child("Accept terms and conditions"),
                        )
                        .child(
                            div()
                                .text_color(t.muted_foreground)
                                .child("You agree to our Terms of Service and Privacy Policy."),
                        ),
                )
                .into_any_element(),
            Radio => RadioGroup::new("density-radio")
                .axis(Axis::Vertical)
                .flex()
                .flex_col()
                .gap(t.unit(10.))
                .children(
                    ["Default", "Comfortable", "Compact"]
                        .into_iter()
                        .enumerate()
                        .map(|(i, label)| {
                            gpui_shadcn::Radio::new(label)
                                .checked(self.radio == i)
                                .on_change(Self::change(cx, move |d, _| d.radio = i))
                                .child(label)
                        }),
                )
                .into_any_element(),
            Switch => gpui_shadcn::Switch::new("airplane")
                .checked(self.switched)
                .on_change(Self::change(cx, |d, next| d.switched = next))
                .child("Airplane Mode")
                .into_any_element(),
            Slider => {
                let value = self.slider.read(cx).value(cx);
                column()
                    .w_full()
                    .max_w(px(320.))
                    .gap(t.unit(12.))
                    .child(
                        row()
                            .justify_between()
                            .child("Volume")
                            .child(format!("{value:.0}%")),
                    )
                    .child(form::Slider::new("volume-slider", &self.slider, "Volume"))
                    .into_any_element()
            }
            Select => {
                let gallery = cx.entity().downgrade();
                gpui_shadcn::Select::new(
                    "fruit-select",
                    ["Apple", "Banana", "Blueberry", "Grapes", "Pineapple"],
                    self.choice,
                )
                .accessibility_label("Fruit")
                .w(t.unit(200.))
                .on_change(move |index, _, cx| {
                    let _ = gallery.update(cx, |g, cx| {
                        g.demos.choice = index;
                        cx.notify();
                    });
                })
                .into_any_element()
            }
            InputOtp => {
                let entity = self.otp.clone();
                column()
                    .items_center()
                    .gap(t.unit(12.))
                    .child(OtpInput::new(&entity))
                    .child(
                        text_link("clear-otp", "Clear code").on_click(move |_, window, cx| {
                            entity.update(cx, |state, cx| state.set_value("", window, cx));
                        }),
                    )
                    .into_any_element()
            }
            _ => unreachable!("page is not a control"),
        }
    }
}
