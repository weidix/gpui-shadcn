use super::Demos;
use crate::{
    catalog::Page,
    gallery::Gallery,
    icon, motion,
    theme::{Theme, Variant, column, row},
};
use gpui_shadcn::gpui::{prelude::FluentBuilder, *};
use gpui_shadcn::{
    display::{Avatar, AvatarImage},
    motion::MotionReveal,
    overlay::Popover,
};

const CHAT: [(&str, [&str; 2]); 4] = [
    (
        "I'm building a chat for our app and the scroll behavior is driving me nuts. Every time the AI streams a reply, the whole thread jumps around.",
        [
            "Keep the viewport pinned to the latest reply while the reader is already at the bottom. As new text arrives, they can keep reading without moving the scroll position themselves.",
            "As soon as the reader scrolls up, leave their position alone. Show a down arrow when the latest text is out of view so they can resume following when they're ready.",
        ],
    ),
    (
        "How can a new turn keep some of the last reply in view?",
        [
            "Use the new question as a scroll anchor so it settles near the top of the transcript.",
            "Leave a little room above the question for the previous reply, then stream the new answer below it.",
        ],
    ),
    (
        "What if the reader wants to revisit an older reply?",
        [
            "Let the new reply continue offscreen without moving them away from what they're reading.",
            "The return button takes them to the bottom and lets the scroller follow new content again.",
        ],
    ),
    (
        "Does the scroll button work with a keyboard?",
        [
            "Yes. Tab to the scroll button and press Enter to return to the latest reply.",
            "At the bottom the button disappears because the latest reply is already in view.",
        ],
    ),
];

fn message(t: Theme, outgoing: bool, avatar: bool, content: Div) -> Div {
    row()
        .flex_shrink_0()
        .items_end()
        .gap(t.unit(8.))
        .when(outgoing, |s| s.flex_row_reverse())
        .when(avatar, |s| {
            s.child(
                Avatar::new()
                    .size(t.unit(32.))
                    .flex_shrink_0()
                    .rounded_full()
                    .overflow_hidden()
                    .image(
                        AvatarImage::new(if outgoing {
                            "avatars/10.png"
                        } else {
                            "avatars/02.png"
                        })
                        .size_full()
                        .rounded_full(),
                    ),
            )
        })
        .child(content.min_w_0().max_w(relative(0.84)))
}

fn message_bubble(t: Theme, outgoing: bool, text: impl Into<SharedString>) -> Div {
    div()
        .px(t.unit(12.))
        .py(t.unit(8.))
        .rounded(t.radius())
        .bg(if outgoing { t.primary } else { t.muted })
        .text_color(if outgoing {
            t.primary_foreground
        } else {
            t.foreground
        })
        .child(text.into())
}

fn reply_length(paragraphs: &[&str]) -> usize {
    paragraphs.iter().map(|paragraph| paragraph.len()).sum()
}

fn bubble(t: Theme, outgoing: bool, message: &'static str) -> Div {
    row().w_full().when(outgoing, |s| s.justify_end()).child(
        div()
            .max_w(relative(0.85))
            .px(t.unit(12.))
            .py(t.unit(8.))
            .rounded(t.radius())
            .bg(if outgoing { t.primary } else { t.muted })
            .text_color(if outgoing {
                t.primary_foreground
            } else {
                t.foreground
            })
            .child(message),
    )
}

fn marker(t: Theme, label: &'static str) -> Div {
    row()
        .w_full()
        .gap(t.unit(8.))
        .text_color(t.muted_foreground)
        .text_size(t.unit(t.typography.xs.size))
        .child(div().flex_1().h(px(1.)).bg(t.border))
        .child(label)
        .child(div().flex_1().h(px(1.)).bg(t.border))
}

impl Demos {
    pub(super) fn messages(
        &self,
        page: Page,
        t: Theme,
        window: &mut Window,
        cx: &mut Context<Gallery>,
    ) -> AnyElement {
        let content = match page {
            Page::Attachment => {
                let attached = self.extra.attachment;
                t.surface().p(t.unit(10.)).child(
                    row()
                        .justify_between()
                        .gap(t.unit(10.))
                        .child(icon("inbox", t.unit(18.), t.foreground))
                        .child(
                            column()
                                .flex_1()
                                .child(div().font_weight(FontWeight::MEDIUM).child(if attached {
                                    "release-notes.pdf"
                                } else {
                                    "No attachment"
                                }))
                                .when(attached, |s| {
                                    s.child(
                                        div().text_color(t.muted_foreground).child("PDF · 248 KB"),
                                    )
                                }),
                        )
                        .child(
                            gpui_shadcn::Button::new(
                                "change-attachment",
                                if attached { "Remove" } else { "Restore" },
                                Variant::Ghost,
                            )
                            .on_click(Self::action(cx, |d| {
                                d.extra.attachment = !d.extra.attachment
                            })),
                        ),
                )
            }
            Page::Bubble => column()
                .gap(t.unit(8.))
                .child(bubble(t, false, "Ready to review the changes?"))
                .child(bubble(t, true, "Yes, send me the preview.")),
            Page::Marker => column()
                .gap(t.unit(8.))
                .child(bubble(t, false, "See you tomorrow."))
                .child(marker(t, "Today"))
                .child(bubble(t, true, "The preview is ready.")),
            Page::Message => column()
                .gap(t.unit(24.))
                .py(t.unit(32.))
                .child(message(
                    t,
                    true,
                    true,
                    message_bubble(t, true, "Deploying to prod real quick."),
                ))
                .child(message(
                    t,
                    false,
                    true,
                    message_bubble(t, false, "It's 4:55 PM. On a Friday."),
                ))
                .child(
                    column()
                        .items_end()
                        .gap(t.unit(4.))
                        .child(
                            message(
                                t,
                                true,
                                true,
                                message_bubble(t, true, "It's a one-line change."),
                            )
                            .w_full(),
                        )
                        .child(
                            div()
                                .pr(t.unit(40.))
                                .text_size(t.unit(t.typography.xs.size))
                                .text_color(t.muted_foreground)
                                .child("Delivered"),
                        ),
                )
                .child(message(
                    t,
                    false,
                    true,
                    column()
                        .items_start()
                        .gap(t.unit(4.))
                        .child(message_bubble(
                            t,
                            false,
                            "It's always a one-line change 😭.",
                        ))
                        .child(
                            message_bubble(t, false, "Alright, let me take a look.")
                                .relative()
                                .mb(t.unit(12.))
                                .child(
                                    div()
                                        .absolute()
                                        .right(t.unit(8.))
                                        .bottom(t.unit(-13.))
                                        .px(t.unit(5.))
                                        .rounded_full()
                                        .border(t.borders.width)
                                        .border_color(t.border)
                                        .bg(t.background)
                                        .text_size(t.unit(t.typography.caption.size))
                                        .child("👍"),
                                ),
                        ),
                ))
                .child(
                    div()
                        .text_size(t.unit(t.typography.compact.size))
                        .text_color(t.muted_foreground)
                        .child("Oliver is typing..."),
                ),
            Page::MessageScroller => self.message_scroller(t, cx),
            Page::Trace => t.surface().p(t.unit(10.)).child(
                column()
                    .gap(t.unit(4.))
                    .child(
                        div()
                            .font_weight(FontWeight::SEMIBOLD)
                            .p(t.unit(6.))
                            .child("Build completed"),
                    )
                    .children(
                        [
                            ("Load project", "Read configuration and dependencies."),
                            ("Compile", "Compiled the workspace successfully."),
                            ("Run checks", "All checks passed."),
                        ]
                        .into_iter()
                        .enumerate()
                        .map(|(i, (title, body))| {
                            let active = self.extra.trace_step == Some(i);
                            let progress = motion::glide(
                                (ElementId::from("trace-step"), title),
                                f32::from(active),
                                t.motion.normal,
                                window,
                                cx,
                            );
                            column()
                                .child(
                                    gpui_shadcn::Button::new(title, title, Variant::Ghost)
                                        .w_full()
                                        .justify_between()
                                        .child(
                                            icon("chevron-down", t.unit(14.), t.muted_foreground)
                                                .with_transformation(Transformation::rotate(
                                                    percentage(progress * 0.5),
                                                )),
                                        )
                                        .on_click(Self::action(cx, move |d| {
                                            d.extra.trace_step = if active { None } else { Some(i) }
                                        })),
                                )
                                .child(MotionReveal::new(
                                    ("trace-body", i),
                                    progress,
                                    div()
                                        .px(t.unit(12.))
                                        .py(t.unit(5.))
                                        .opacity(progress)
                                        .text_color(t.muted_foreground)
                                        .child(body)
                                        .into_any_element(),
                                ))
                        }),
                    ),
            ),
            _ => unreachable!(),
        };
        let width = if matches!(page, Page::Message | Page::MessageScroller) {
            384.
        } else {
            360.
        };
        content.w_full().max_w(t.unit(width)).into_any_element()
    }

    fn message_scroller(&self, t: Theme, cx: &mut Context<Gallery>) -> Div {
        let turn = self.extra.message_chunk;
        let scroll = &self.extra.messages_scroll;
        let streaming = turn.is_some_and(|(index, count)| count < reply_length(&CHAT[index].1));
        let next = turn.map_or(0, |(index, _)| index + 1);
        let prompt = CHAT.get(next).map(|entry| entry.0);
        let transcript = scroll.transcript(column().id("messages-scroll"))
            .size_full()
            .p(t.unit(16.))
            .pb(t.unit(32.))
            .gap(t.unit(14.))
            .when(turn.is_none(), |s| {
                s.justify_center().items_center().text_center()
                    .child(div().size(t.unit(40.)).rounded(t.radius_control()).bg(t.muted)
                        .flex().items_center().justify_center().child(icon("message-circle-dashed", t.unit(20.), t.foreground)))
                    .child(div().font_weight(FontWeight::SEMIBOLD).text_size(t.unit(t.typography.lg.size)).child("Morning, shadcn!"))
                    .child(div().max_w(t.unit(260.)).text_color(t.muted_foreground)
                        .child("What are we working on today? Press send to start a new conversation"))
            })
            .when_some(turn, |s, (index, count)| {
                s.children(CHAT[..=index].iter().enumerate().flat_map(|(i, (question, paragraphs))| {
                    let mut visible = if i == index { count } else { reply_length(paragraphs) };
                    let question = message(t, true, false, message_bubble(t, true, *question));
                    let answer = if visible == 0 {
                        div().text_color(t.muted_foreground).child("Thinking…")
                    } else {
                        column().gap(t.unit(12.)).children(paragraphs.iter().filter_map(|paragraph| {
                            let rendered = visible.min(paragraph.len());
                            visible -= rendered;
                            (rendered > 0).then(|| div().child(&paragraph[..rendered]))
                        }))
                    };
                    [question, answer.flex_shrink_0()]
                }))
            });

        let send =
            gpui_shadcn::Button::icon("send-chat", "Send", "arrow-up", Variant::Primary, 30.)
                .disabled(streaming || prompt.is_none())
                .on_click(cx.listener(move |g, _, _, cx| {
                    if g.demos
                        .extra
                        .message_chunk
                        .is_some_and(|(index, count)| count < reply_length(&CHAT[index].1))
                        || next >= CHAT.len()
                    {
                        return;
                    }
                    g.demos.extra.message_chunk = Some((next, 0));
                    g.demos.extra.messages_scroll.follow_end();
                    cx.notify();
                    let executor = cx.background_executor().clone();
                    cx.spawn(async move |weak, cx| {
                        executor.timer(std::time::Duration::from_millis(350)).await;
                        let mut count = 0;
                        let chunks = CHAT[next].1.into_iter().flat_map(|paragraph| {
                            paragraph
                                .split_inclusive(' ')
                                .collect::<Vec<_>>()
                                .chunks(3)
                                .map(|words| words.iter().map(|word| word.len()).sum::<usize>())
                                .collect::<Vec<_>>()
                        });
                        for length in chunks {
                            executor.timer(std::time::Duration::from_millis(75)).await;
                            count += length;
                            if weak
                                .update(cx, |gallery, cx| {
                                    let extra = &mut gallery.demos.extra;
                                    extra.message_chunk = Some((next, count));
                                    cx.notify();
                                })
                                .is_err()
                            {
                                break;
                            }
                        }
                    })
                    .detach();
                }));
        let add = Popover::new("chat-add")
            .anchor(Anchor::BottomLeft)
            .offset(t.unit(4.))
            .trigger(gpui_shadcn::Button::icon(
                "chat-add-trigger",
                "Add files",
                "plus",
                Variant::Outline,
                30.,
            ))
            .content(move |_, _, _| {
                t.popup_surface().w(t.unit(176.)).p(t.unit(4.)).child(
                    column().children(
                        [
                            "Add Photos & Files",
                            "Create Image",
                            "Deep Research",
                            "Web Search",
                        ]
                        .map(|label| {
                            gpui_shadcn::Button::new(label, label, Variant::Ghost)
                                .rounded(t.popup_item_radius())
                                .justify_start()
                                .disabled(true)
                        }),
                    ),
                )
            });
        column()
            .gap(t.unit(12.))
            .child(
                t.surface().h(t.unit(560.)).overflow_hidden().child(
                    column()
                        .size_full()
                        .child(
                            row()
                                .items_start()
                                .justify_between()
                                .p(t.unit(16.))
                                .border_b(t.borders.width)
                                .border_color(t.border)
                                .child(
                                    column()
                                        .gap(t.unit(4.))
                                        .child(
                                            div()
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .child("New Chat"),
                                        )
                                        .child(
                                            div()
                                                .text_color(t.muted_foreground)
                                                .child("How can I help you today?"),
                                        ),
                                )
                                .child(
                                    gpui_shadcn::Button::icon(
                                        "new-chat",
                                        "Reset conversation",
                                        "rotate-cw",
                                        Variant::Outline,
                                        32.,
                                    )
                                    .disabled(streaming)
                                    .on_click(Self::action(
                                        cx,
                                        |d| {
                                            d.extra.message_chunk = None;
                                            d.extra
                                                .messages_scroll
                                                .set_offset(point(px(0.), px(0.)));
                                        },
                                    )),
                                ),
                        )
                        .child(
                            div()
                                .relative()
                                .flex_1()
                                .min_h_0()
                                .child(transcript)
                                .child(scroll.follow_content_layout())
                                .when(turn.is_some() && !scroll.following_end(), |s| {
                                    s.child(
                                        div()
                                            .absolute()
                                            .left_0()
                                            .right_0()
                                            .bottom(t.unit(8.))
                                            .flex()
                                            .justify_center()
                                            .child(
                                                gpui_shadcn::Button::icon(
                                                    "latest-message",
                                                    "Scroll to latest",
                                                    "arrow-down",
                                                    Variant::Outline,
                                                    32.,
                                                )
                                                .rounded_full()
                                                .bg(t.background)
                                                .shadow(
                                                    t.shadows
                                                        .sm
                                                        .map(gpui_shadcn::theme::Shadow::paint)
                                                        .to_vec(),
                                                )
                                                .on_click(cx.listener(|g, _, _, cx| {
                                                    g.demos.extra.messages_scroll.follow_end();
                                                    cx.notify();
                                                })),
                                            ),
                                    )
                                }),
                        )
                        .child(
                            div().p(t.unit(16.)).child(
                                column()
                                    .rounded(t.radius_control())
                                    .border(t.borders.width)
                                    .border_color(t.input)
                                    .child(
                                        div().h(t.unit(54.)).px(t.unit(12.)).pt(t.unit(9.)).child(
                                            div()
                                                .line_clamp(2)
                                                .when(streaming || prompt.is_none(), |s| {
                                                    s.text_color(t.muted_foreground)
                                                })
                                                .child(prompt.unwrap_or(
                                                    "No messages queued. Reset the conversation.",
                                                )),
                                        ),
                                    )
                                    .child(
                                        row()
                                            .items_center()
                                            .justify_between()
                                            .px(t.unit(8.))
                                            .pb(t.unit(8.))
                                            .child(add)
                                            .child(send),
                                    ),
                            ),
                        ),
                ),
            )
            .child(
                div()
                    .text_center()
                    .text_size(t.unit(t.typography.xs.size))
                    .text_color(t.muted_foreground)
                    .child("Demo is read only. Press send to send messages."),
            )
    }
}
