use gpui_shadcn::gpui::{prelude::FluentBuilder, *};
use gpui_shadcn::{Input, overlay};

use super::Demos;
use crate::{
    catalog::Page,
    gallery::{Gallery, window_corner_radius},
    icon,
    theme::{Theme, Variant, column, row},
};

impl Demos {
    pub(super) fn overlays(
        &self,
        page: Page,
        t: Theme,
        window: &Window,
        cx: &mut Context<Gallery>,
    ) -> AnyElement {
        match page {
            Page::Dialog | Page::AlertDialog => self.modal(
                t,
                page == Page::AlertDialog,
                window_corner_radius(t, window),
                cx,
            ),
            Page::Sheet => self.sheet(t, window_corner_radius(t, window), cx),
            Page::Dropdown | Page::Popover => {
                let dropdown = page == Page::Dropdown;
                let entity = cx.entity().downgrade();
                let input = self.input.clone();
                overlay::Popover::new("popup-demo")
                    .offset(px(6.))
                    .trigger(gpui_shadcn::Button::new(
                        "popup-trigger",
                        if dropdown {
                            "Open menu"
                        } else {
                            "Open popover"
                        },
                        Variant::Outline,
                    ))
                    .content(move |popup, _, popup_cx| {
                        let panel = t.popup_surface().flex().flex_col();
                        if dropdown {
                            panel
                                .w(t.unit(180.))
                                .p(t.unit(4.))
                                .child(
                                    div()
                                        .px(t.unit(8.))
                                        .py(t.unit(5.))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("My account"),
                                )
                                .child(div().h(px(1.)).bg(t.border).my(t.unit(4.)))
                                .children(["Profile", "Billing", "Settings", "Sign out"].map(
                                    |label| {
                                        let entity = entity.clone();
                                        let popup = popup.clone();
                                        gpui_shadcn::Button::new(label, label, Variant::Ghost)
                                            .rounded(t.popup_item_radius())
                                            .justify_start()
                                            .px(t.unit(8.))
                                            .h(t.unit(30.))
                                            .on_click(move |_, window, cx| {
                                                popup.dismiss(window, cx);
                                                let _ = entity.update(cx, |this, cx| {
                                                    this.demos.status =
                                                        Some(format!("{label} selected.").into());
                                                    cx.notify();
                                                });
                                            })
                                    },
                                ))
                        } else {
                            panel
                                .w(t.unit(275.))
                                .p(t.unit(16.))
                                .gap(t.unit(10.))
                                .child(div().font_weight(FontWeight::SEMIBOLD).child("Profile"))
                                .child(
                                    div()
                                        .text_color(t.muted_foreground)
                                        .child("Set the email for this profile."),
                                )
                                .child(
                                    Input::frame("field-frame", popup_cx).child(Input::new(&input)),
                                )
                        }
                    })
                    .into_any_element()
            }
            Page::Tooltip => self.tooltip(t),
            Page::Toast => {
                let trigger = gpui_shadcn::Button::new("toast", "Show toast", Variant::Outline)
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.demos.toast += 1;
                        let generation = this.demos.toast;
                        this.demos.toast_visible = true;
                        cx.notify();
                        let timer = cx
                            .background_executor()
                            .timer(std::time::Duration::from_secs(4));
                        cx.spawn(async move |weak, cx| {
                            timer.await;
                            let _ = weak.update(cx, |gallery, cx| {
                                if gallery.demos.toast == generation {
                                    gallery.demos.toast_visible = false;
                                    cx.notify();
                                }
                            });
                        })
                        .detach();
                    }));
                column().child(trigger).into_any_element()
            }
            _ => unreachable!("page is not an overlay demo"),
        }
    }

    fn tooltip(&self, t: Theme) -> AnyElement {
        use gpui_shadcn::overlay::{Tooltip, TooltipSide::*};

        let example = self.example_index(Page::Tooltip);
        if example != 0 {
            let (trigger, label) = match example {
                1 => ("Save", "Save changes"),
                2 => ("Disabled", "This action is unavailable"),
                _ => ("مساعدة", "عرض المساعدة"),
            };
            let tooltip = Tooltip::new(
                ("tooltip-variant", example),
                label,
                gpui_shadcn::Button::new("tooltip-trigger", trigger, Variant::Outline)
                    .disabled(example == 2),
            );
            return if example == 1 {
                tooltip.shortcut("⌘ S")
            } else {
                tooltip
            }
            .into_any_element();
        }

        row()
            .gap(t.unit(8.))
            .children(
                [
                    ("Left", Left),
                    ("Top", Top),
                    ("Bottom", Bottom),
                    ("Right", Right),
                ]
                .map(|(label, side)| {
                    Tooltip::new(
                        (ElementId::from("tooltip-example"), label),
                        format!("{label} tooltip"),
                        gpui_shadcn::Button::new(label, label, Variant::Outline),
                    )
                    .side(side)
                }),
            )
            .into_any_element()
    }

    pub fn toast_overlay(&self, t: Theme, cx: &Context<Gallery>) -> Option<AnyElement> {
        self.toast_visible.then(|| {
            overlay::Toast::new("confirmation-toast")
                .absolute()
                .bottom(px(28.))
                .right(px(28.))
                .child(crate::motion::enter(
                    t.surface()
                        .shadow(t.shadows.lg.map(gpui_shadcn::theme::Shadow::paint).to_vec())
                        .p(t.unit(14.))
                        .flex()
                        .items_center()
                        .gap(t.unit(10.))
                        .child(icon("check", t.unit(17.), t.foreground))
                        .child(
                            column()
                                .gap(t.unit(2.))
                                .child(
                                    div()
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("Event created"),
                                )
                                .child(
                                    div()
                                        .text_color(t.muted_foreground)
                                        .child("Your event was added to the calendar."),
                                ),
                        )
                        .child(
                            gpui_shadcn::Button::new("dismiss-toast", "Dismiss", Variant::Ghost)
                                .on_click(Self::action(cx, |d| d.toast_visible = false)),
                        ),
                    ("toast-entry", self.toast as usize),
                    t.motion.slow,
                    point(px(0.), px(12.)),
                    cx,
                ))
                .into_any_element()
        })
    }

    fn modal(
        &self,
        t: Theme,
        alert: bool,
        frame_radius: Pixels,
        cx: &mut Context<Gallery>,
    ) -> AnyElement {
        let handle = if alert {
            &self.alert_dialog
        } else {
            &self.dialog
        };
        let title = if alert {
            "Are you absolutely sure?"
        } else {
            "Edit profile"
        };
        let description = if alert {
            "This action cannot be undone. This will permanently delete your example account."
        } else {
            "Make changes to your profile here. Click save when you’re done."
        };
        let modal_focus = self.modal_focus.clone();
        let trigger_focus = self.modal_trigger_focus.clone();
        let open = handle.clone();
        let cancel = handle.clone();
        let confirm = handle.clone();
        let entity = cx.entity().downgrade();
        let trigger = gpui_shadcn::Button::new(
            "open-modal",
            if alert {
                "Show confirmation"
            } else {
                "Edit profile"
            },
            Variant::Outline,
        )
        .track_focus(&self.modal_trigger_focus)
        .on_click(move |_, window, cx| {
            open.open(window, cx);
            modal_focus.focus(window, cx);
        });
        let footer = row()
            .justify_end()
            .gap(t.unit(8.))
            .child(
                gpui_shadcn::Button::new("cancel-modal", "Cancel", Variant::Outline)
                    .on_click(move |_, window, cx| cancel.close(window, cx)),
            )
            .child(
                gpui_shadcn::Button::new(
                    "confirm-modal",
                    if alert { "Continue" } else { "Save changes" },
                    if alert {
                        Variant::Destructive
                    } else {
                        Variant::Primary
                    },
                )
                .on_click(move |_, window, cx| {
                    confirm.close(window, cx);
                    let _ = entity.update(cx, |this, cx| {
                        this.demos.status = Some(
                            if alert {
                                "Example account deleted."
                            } else {
                                "Profile saved."
                            }
                            .into(),
                        );
                        cx.notify();
                    });
                }),
            );
        let frame = t
            .surface()
            .w(t.unit(410.))
            .p(t.unit(24.))
            .shadow(t.shadows.lg.map(gpui_shadcn::theme::Shadow::paint).to_vec())
            .flex()
            .flex_col()
            .gap(t.unit(16.))
            .text_color(t.foreground);
        let heading = div()
            .text_size(t.unit(t.typography.lg.size))
            .font_weight(FontWeight::SEMIBOLD)
            .child(title);
        let body = div().text_color(t.muted_foreground).child(description);
        let on_open_change = {
            let entity = cx.entity().downgrade();
            move |is_open: bool, _, window: &mut Window, cx: &mut App| {
                if !is_open {
                    trigger_focus.focus(window, cx);
                }
                let _ = entity.update(cx, |_, cx| cx.notify());
            }
        };
        let panel = if alert {
            frame
                .child(overlay::AlertDialogTitle::new().child(heading))
                .child(overlay::AlertDialogDescription::new().child(body))
                .child(footer)
        } else {
            frame
                .child(overlay::DialogTitle::new().child(heading))
                .child(overlay::DialogDescription::new().child(body))
                .child(
                    column()
                        .gap(t.unit(7.))
                        .child("Email")
                        .child(Input::frame("field-frame", cx).child(Input::new(&self.input))),
                )
                .child(footer)
        };
        let panel = crate::motion::enter(
            panel,
            "modal-entry",
            t.motion.normal,
            point(px(0.), px(9.)),
            cx,
        );
        if alert {
            column()
                .child(trigger)
                .child(
                    overlay::AlertDialog::new(cx)
                        .focus_handle(self.modal_focus.clone())
                        .handle(handle.clone())
                        .on_open_change(on_open_change)
                        .backdrop(
                            overlay::AlertDialogBackdrop::new()
                                .absolute()
                                .inset_0()
                                .rounded(frame_radius)
                                .bg(t.overlay),
                        )
                        .popup(overlay::AlertDialogPopup::new().child(panel)),
                )
                .into_any_element()
        } else {
            column()
                .child(trigger)
                .child(
                    overlay::Dialog::new(cx)
                        .focus_handle(self.modal_focus.clone())
                        .handle(handle.clone())
                        .on_open_change(on_open_change)
                        .backdrop(
                            overlay::DialogBackdrop::new()
                                .absolute()
                                .inset_0()
                                .rounded(frame_radius)
                                .bg(t.overlay),
                        )
                        .popup(
                            overlay::DialogPopup::new().child(
                                div()
                                    .id("dialog-content")
                                    .role(Role::Group)
                                    .aria_label(title)
                                    .child(panel),
                            ),
                        ),
                )
                .into_any_element()
        }
    }

    fn sheet(&self, t: Theme, frame_radius: Pixels, cx: &mut Context<Gallery>) -> AnyElement {
        let handle = self.sheet.clone();
        let modal_focus = self.modal_focus.clone();
        let trigger_focus = self.modal_trigger_focus.clone();
        let open = handle.clone();
        let close = handle.clone();
        let entity = cx.entity().downgrade();
        let trigger = gpui_shadcn::Button::new("sheet-trigger", "Open sheet", Variant::Outline)
            .track_focus(&self.modal_trigger_focus)
            .on_click(move |_, window, cx| {
                open.open(window, cx);
                modal_focus.focus(window, cx);
                let _ = entity.update(cx, |_, cx| cx.notify());
            });
        let entity = cx.entity().downgrade();
        let dismiss = move |window: &mut Window, cx: &mut App| {
            close.close(window, cx);
            trigger_focus.focus(window, cx);
            let _ = entity.update(cx, |_, cx| cx.notify());
        };
        let close_button = dismiss.clone();
        column()
            .child(trigger)
            .when(handle.is_open(), |this| {
                this.child(
                    deferred(
                        overlay::Sheet::new(cx)
                            .focus_handle(self.modal_focus.clone())
                            .overlay(
                                div()
                                    .absolute()
                                    .inset_0()
                                    .rounded(frame_radius)
                                    .bg(t.overlay),
                            )
                            .request_close(dismiss)
                            .surface(
                                column()
                                    .absolute()
                                    .top_0()
                                    .right_0()
                                    .bottom_0()
                                    .w(t.unit(360.))
                                    .bg(t.background)
                                    .rounded_tr(frame_radius)
                                    .rounded_br(frame_radius)
                                    .border_l(t.borders.width)
                                    .border_color(t.border)
                                    .p(t.unit(24.))
                                    .gap(t.unit(14.))
                                    .occlude()
                                    .child(
                                        row()
                                            .justify_between()
                                            .child(
                                                div()
                                                    .text_size(t.unit(t.typography.lg.size))
                                                    .font_weight(FontWeight::SEMIBOLD)
                                                    .child("Edit profile"),
                                            )
                                            .child(
                                                gpui_shadcn::Button::new(
                                                    "close-sheet",
                                                    "Close",
                                                    Variant::Ghost,
                                                )
                                                .on_click(move |_, window, cx| {
                                                    close_button(window, cx)
                                                }),
                                            ),
                                    )
                                    .child(
                                        div().text_color(t.muted_foreground).child(
                                            "Make changes to your profile in this side panel.",
                                        ),
                                    )
                                    .child(
                                        Input::frame("field-frame", cx)
                                            .child(Input::new(&self.input)),
                                    )
                                    .with_animation(
                                        "sheet-entry",
                                        Animation::new(std::time::Duration::from_millis(
                                            t.motion.slow,
                                        ))
                                        .with_easing(
                                            move |progress| t.motion.easing.sample(progress),
                                        ),
                                        move |panel, progress| {
                                            panel.right(t.unit(-360. * (1. - progress)))
                                        },
                                    ),
                            ),
                    )
                    .with_priority(10),
                )
            })
            .into_any_element()
    }
}
