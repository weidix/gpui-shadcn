use gpui::*;
use std::{cell::Cell, rc::Rc, time::Instant};

#[derive(Clone, Default)]
pub struct SmoothScroll {
    handle: ScrollHandle,
    motion: Rc<Cell<Option<ScrollMotion>>>,
    frame_pending: Rc<Cell<bool>>,
    following_end: Rc<Cell<bool>>,
}

#[derive(Clone, Copy)]
struct ScrollMotion {
    from: Point<Pixels>,
    to: Point<Pixels>,
    started: Instant,
}

impl SmoothScroll {
    pub fn vertical(&self, viewport: Stateful<Div>) -> Stateful<Div> {
        self.viewport(viewport, Axis::Vertical, false)
    }

    pub fn transcript(&self, viewport: Stateful<Div>) -> Stateful<Div> {
        self.viewport(viewport, Axis::Vertical, true)
    }

    pub fn horizontal(&self, viewport: Stateful<Div>) -> Stateful<Div> {
        self.viewport(viewport, Axis::Horizontal, false)
    }

    fn viewport(&self, viewport: Stateful<Div>, axis: Axis, follow_end: bool) -> Stateful<Div> {
        let scroll = self.clone();
        // Keep GPUI's layout and clipping while taking ownership of wheel offsets.
        viewport
            .track_scroll(&self.handle)
            .overflow_hidden()
            .on_scroll(move |event, window, cx| {
                let delta = event.delta.pixel_delta(window.line_height());
                let precise = event.delta.precise();
                let amount = match axis {
                    Axis::Vertical => delta.y,
                    Axis::Horizontal if delta.x != px(0.) || precise => delta.x,
                    Axis::Horizontal => delta.y,
                };
                if amount == px(0.) {
                    return;
                }
                let current = scroll.handle.offset();
                let mut target = if precise {
                    current
                } else {
                    scroll.motion.get().map_or(current, |motion| motion.to)
                };
                match axis {
                    Axis::Vertical => target.y += amount,
                    Axis::Horizontal => target.x += amount,
                }
                target = scroll.clamped(target);
                if follow_end {
                    scroll
                        .following_end
                        .set(target.y == -scroll.handle.max_offset().y);
                }
                if target == current && scroll.motion.get().is_none() {
                    return;
                }
                if precise || cx.reduce_motion() {
                    scroll.move_to(target);
                    window.refresh();
                } else {
                    scroll.animate_to(target, window);
                }
                cx.stop_propagation();
            })
    }

    fn clamped(&self, target: Point<Pixels>) -> Point<Pixels> {
        let max = self.handle.max_offset();
        point(
            target.x.clamp(-max.x, px(0.)),
            target.y.clamp(-max.y, px(0.)),
        )
    }

    pub fn set_offset(&self, target: Point<Pixels>) {
        self.following_end.set(false);
        self.move_to(target);
    }

    fn move_to(&self, target: Point<Pixels>) {
        self.motion.set(None);
        self.handle.set_offset(target);
    }

    fn at_end(&self) -> bool {
        let position = self
            .motion
            .get()
            .map_or(self.handle.offset(), |motion| motion.to);
        self.handle.max_offset().y + position.y <= px(0.1)
    }

    pub fn following_end(&self) -> bool {
        self.following_end.get()
    }

    pub fn follow_end(&self) {
        self.following_end.set(true);
    }

    pub fn follow_content_layout(&self) -> impl IntoElement {
        let scroll = self.clone();
        canvas(
            |_, _, _| (),
            move |_, (), window, _| {
                // The transcript has measured its new scroll range in prepaint.
                if scroll.following_end() && !scroll.at_end() {
                    window.on_next_frame(move |window, cx| {
                        if scroll.following_end() {
                            scroll.scroll_to_end(window, cx);
                        }
                    });
                }
            },
        )
        .absolute()
        .size(px(0.))
    }

    fn scroll_to_end(&self, window: &mut Window, cx: &App) {
        let target = point(px(0.), -self.handle.max_offset().y);
        if cx.reduce_motion() {
            self.move_to(target);
            window.refresh();
        } else {
            self.animate_to(target, window);
        }
    }

    fn animate_to(&self, to: Point<Pixels>, window: &mut Window) {
        if self.motion.get().is_some_and(|motion| motion.to == to) {
            return;
        }
        let from = self.handle.offset();
        if from == to {
            self.motion.set(None);
            return;
        }
        self.motion.set(Some(ScrollMotion {
            from,
            to,
            started: Instant::now(),
        }));
        self.next_frame(window);
    }

    fn next_frame(&self, window: &Window) {
        if self.frame_pending.replace(true) {
            return;
        }
        let scroll = self.clone();
        window.on_next_frame(move |window, _| {
            scroll.frame_pending.set(false);
            let Some(motion) = scroll.motion.get() else {
                return;
            };
            let progress = (motion.started.elapsed().as_secs_f32() / 0.15).min(1.);
            let eased = crate::motion::ease_out_cubic(progress);
            let target = scroll.clamped(motion.to);
            scroll
                .handle
                .set_offset(motion.from + (target - motion.from) * eased);
            window.refresh();
            if progress < 1. {
                scroll.next_frame(window);
            } else {
                scroll.motion.set(None);
            }
        });
    }
}
