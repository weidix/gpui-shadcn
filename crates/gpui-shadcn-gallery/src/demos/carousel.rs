use std::time::Duration;

use gpui_shadcn::gpui::{prelude::FluentBuilder, *};

use super::Demos;
use crate::{
    gallery::Gallery,
    motion,
    theme::{Theme, Variant, column, row},
};

const SLIDE_COUNT: i32 = 5;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Example {
    Default,
    Sizes,
    Spacing,
    Orientation,
    Options,
    Api,
    Plugins,
    Indicators,
}

impl Example {
    const ALL: [Self; 8] = [
        Self::Default,
        Self::Sizes,
        Self::Spacing,
        Self::Orientation,
        Self::Options,
        Self::Api,
        Self::Plugins,
        Self::Indicators,
    ];

    fn title(self) -> &'static str {
        match self {
            Self::Default => "Preview",
            Self::Sizes => "Sizes",
            Self::Spacing => "Spacing",
            Self::Orientation => "Orientation",
            Self::Options => "Options",
            Self::Api => "API",
            Self::Plugins => "Plugins · Autoplay",
            Self::Indicators => "Indicators",
        }
    }

    fn looping(self) -> bool {
        matches!(self, Self::Options | Self::Plugins)
    }
}

struct CarouselState {
    focus: FocusHandle,
    position: i32,
    drag_start: Option<Point<Pixels>>,
}

pub(super) struct CarouselDemo {
    examples: [CarouselState; Example::ALL.len()],
    autoplay: Option<Task<()>>,
}

impl Demos {
    pub(crate) fn set_carousel_autoplay(&mut self, active: bool, cx: &mut Context<Gallery>) {
        self.carousel.set_autoplay(
            active
                && self.example_index(crate::catalog::Page::Carousel) == Example::Plugins as usize,
            cx,
        );
    }
}

impl CarouselDemo {
    pub(super) fn new(cx: &mut Context<Gallery>) -> Self {
        Self {
            examples: std::array::from_fn(|_| CarouselState {
                focus: cx.focus_handle().tab_stop(true),
                position: 0,
                drag_start: None,
            }),
            autoplay: None,
        }
    }

    fn set_autoplay(&mut self, active: bool, cx: &mut Context<Gallery>) {
        if !active {
            self.autoplay = None;
        } else if self.autoplay.is_none() {
            let executor = cx.background_executor().clone();
            self.autoplay = Some(cx.spawn(async move |weak, cx| {
                loop {
                    executor.timer(Duration::from_secs(2)).await;
                    if weak
                        .update(cx, |gallery, cx| {
                            gallery.demos.carousel.examples[Example::Plugins as usize].position +=
                                1;
                            cx.notify();
                        })
                        .is_err()
                    {
                        break;
                    }
                }
            }));
        }
    }

    pub(super) fn render(
        &self,
        selected: usize,
        t: Theme,
        window: &mut Window,
        cx: &mut Context<Gallery>,
    ) -> Div {
        self.example(Example::ALL[selected], t, window, cx)
    }

    fn example(
        &self,
        example: Example,
        t: Theme,
        window: &mut Window,
        cx: &mut Context<Gallery>,
    ) -> Div {
        let index = example as usize;
        let state = &self.examples[index];
        let vertical = example == Example::Orientation;
        let multi = matches!(example, Example::Sizes | Example::Spacing);
        let visible = if vertical {
            2
        } else if multi {
            if window.viewport_size().width >= px(1024.) {
                3
            } else {
                2
            }
        } else {
            1
        };
        let last = SLIDE_COUNT - visible;
        let position = if example.looping() {
            state.position
        } else {
            state.position.min(last)
        };
        let offset = motion::glide(
            (ElementId::from("carousel-position"), example.title()),
            position as f32,
            t.motion.slow,
            window,
            cx,
        );
        let fraction = 1. / visible as f32;
        let step = move |gallery: &mut Gallery, direction: i32, cx: &mut Context<Gallery>| {
            let state = &mut gallery.demos.carousel.examples[index];
            state.position = if example.looping() {
                state.position + direction
            } else {
                (state.position.min(last) + direction).clamp(0, last)
            };
            cx.notify();
        };
        let arrow = |id, label, glyph, direction| {
            gpui_shadcn::Button::icon(id, label, glyph, Variant::Outline, 32.)
                .flex_shrink_0()
                .rounded_full()
                .disabled(
                    !example.looping()
                        && if direction < 0 {
                            position <= 0
                        } else {
                            position >= last
                        },
                )
                .on_click(cx.listener(move |gallery, _, _, cx| step(gallery, direction, cx)))
        };
        let focus = state.focus.clone();
        let drag_end = || {
            cx.listener(move |gallery, event: &MouseUpEvent, _, cx| {
                let Some(start) = gallery.demos.carousel.examples[index].drag_start.take() else {
                    return;
                };
                let delta = event.position - start;
                let (along, across) = if vertical {
                    (delta.y, delta.x)
                } else {
                    (delta.x, delta.y)
                };
                if along.abs() >= px(32.) && along.abs() > across.abs() {
                    let direction = if along < px(0.) { 1 } else { -1 };
                    step(gallery, direction, cx);
                }
            })
        };
        let range = if example.looping() {
            let first = offset.floor() as i32 - 1;
            first..first + 4
        } else {
            0..SLIDE_COUNT
        };
        let track = div()
            .id("carousel-viewport")
            .relative()
            .w_full()
            .map(|track| {
                if vertical {
                    track.h(t.unit(270.))
                } else {
                    track.aspect_ratio(visible as f32)
                }
            })
            .overflow_hidden()
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |gallery, event: &MouseDownEvent, window, cx| {
                    focus.focus(window, cx);
                    gallery.demos.carousel.examples[index].drag_start = Some(event.position);
                }),
            )
            .on_mouse_up(MouseButton::Left, drag_end())
            .on_mouse_up_out(MouseButton::Left, drag_end())
            .children(range.map(|slot| {
                let number = slot.rem_euclid(SLIDE_COUNT) + 1;
                let displacement = relative((slot as f32 - offset) * fraction);
                let card = t
                    .surface()
                    .size_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(t.unit(match example {
                        Example::Spacing => 24.,
                        Example::Sizes | Example::Orientation => 30.,
                        _ => 36.,
                    }))
                    .font_weight(FontWeight::SEMIBOLD)
                    .child(number.to_string());
                div()
                    .absolute()
                    .map(|item| {
                        if vertical {
                            item.top(displacement).w_full().h(relative(fraction))
                        } else {
                            item.left(displacement).h_full().w(relative(fraction))
                        }
                    })
                    .p(t.unit(4.))
                    .when(example == Example::Spacing, |item| item.px(t.unit(6.)))
                    .child(card)
            }));
        let (previous_glyph, next_glyph) = match example {
            Example::Orientation => ("arrow-up", "arrow-down"),
            _ => ("arrow-left", "arrow-right"),
        };
        let previous = arrow("previous", "Previous slide", previous_glyph, -1);
        let next = arrow("next", "Next slide", next_glyph, 1);
        let controls = if vertical {
            column()
                .items_center()
                .child(previous)
                .child(track)
                .child(next)
        } else {
            row()
                .child(previous)
                .child(div().flex_1().min_w_0().child(track))
                .child(next)
        };
        let max_width = if multi {
            480.
        } else if vertical {
            320.
        } else {
            416.
        };
        column()
            .w_full()
            .max_w(t.unit(max_width))
            .gap(t.unit(8.))
            .child(
                controls
                    .id(("carousel", index))
                    .track_focus(&state.focus)
                    .gap(t.unit(16.))
                    .rounded(t.radius_surface())
                    .border(t.borders.width)
                    .border_color(t.background.opacity(0.))
                    .focus_visible(move |style| style.border_color(t.ring))
                    .on_key_down(cx.listener(move |gallery, event: &KeyDownEvent, _, cx| {
                        let direction = match (vertical, event.keystroke.key.as_str()) {
                            (true, "up") | (false, "left") => -1,
                            (true, "down") | (false, "right") => 1,
                            _ => return,
                        };
                        step(gallery, direction, cx);
                        cx.stop_propagation();
                    }))
                    .when(example == Example::Plugins, |element| {
                        element.on_hover(cx.listener(|gallery, hovered: &bool, _, cx| {
                            gallery.demos.carousel.set_autoplay(!hovered, cx);
                        }))
                    }),
            )
            .when(example == Example::Indicators, |element| {
                element.child(
                    row()
                        .justify_center()
                        .children((0..SLIDE_COUNT).map(|target| {
                            let active = target == position;
                            gpui_shadcn::Button::with_content(
                                ("slide-indicator", target as usize),
                                Variant::Ghost,
                            )
                            .accessibility_label(if active {
                                format!("Slide {} of {SLIDE_COUNT}, current slide", target + 1)
                            } else {
                                format!("Go to slide {}", target + 1)
                            })
                            .size(t.unit(24.))
                            .p_0()
                            .rounded_full()
                            .child(div().size(t.unit(8.)).rounded_full().bg(if active {
                                t.primary
                            } else {
                                t.muted_foreground.opacity(0.45)
                            }))
                            .on_click(cx.listener(
                                move |gallery, _, _, cx| {
                                    gallery.demos.carousel.examples[index].position = target;
                                    cx.notify();
                                },
                            ))
                        })),
                )
            })
            .when(example == Example::Api, |element| {
                element.child(
                    div()
                        .text_center()
                        .text_color(t.muted_foreground)
                        .child(format!("Slide {} of {SLIDE_COUNT}", position + 1)),
                )
            })
    }
}
