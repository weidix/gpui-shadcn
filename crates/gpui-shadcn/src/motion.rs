use gpui::*;
use std::time::Duration;

use crate::Theme;

pub fn enter(
    panel: Div,
    id: impl Into<ElementId>,
    duration_ms: u64,
    offset: Point<Pixels>,
    cx: &App,
) -> AnyElement {
    if cx.reduce_motion() {
        return panel.into_any_element();
    }
    let easing = Theme::global(cx).motion.easing;
    panel
        .relative()
        .with_animation(
            id,
            Animation::new(Duration::from_millis(duration_ms))
                .with_easing(move |progress| easing.sample(progress)),
            move |panel, progress| {
                panel
                    .left(offset.x * (1. - progress))
                    .top(offset.y * (1. - progress))
                    .opacity(0.4 + 0.6 * progress)
            },
        )
        .into_any_element()
}

/// Fades a popup in while keeping its measured position and dimensions stable.
pub fn fade_in(panel: Div, id: impl Into<ElementId>, duration_ms: u64, cx: &App) -> AnyElement {
    if cx.reduce_motion() {
        return panel.into_any_element();
    }
    let easing = Theme::global(cx).motion.easing;
    panel
        .with_animation(
            id,
            Animation::new(Duration::from_millis(duration_ms))
                .with_easing(move |progress| easing.sample(progress)),
            |panel, progress| panel.opacity(progress),
        )
        .into_any_element()
}

pub fn glide(
    id: impl Into<ElementId>,
    target: f32,
    duration_ms: u64,
    window: &mut Window,
    cx: &mut App,
) -> f32 {
    let easing = Theme::global(cx).motion.easing;
    gpui_base::transition(
        id.into(),
        target,
        gpui_base::Transition::new(Duration::from_millis(duration_ms))
            .ease(move |progress| easing.sample(progress)),
        window,
        cx,
    )
}

/// Cubic ease-out for component animations.
pub fn ease_out_cubic(progress: f32) -> f32 {
    1. - (1. - progress.clamp(0., 1.)).powi(3)
}

element!(
    /// Reveals an element's height as progress moves from zero to one.
    MotionReveal(gpui_base::MotionReveal)
);
constructor!(MotionReveal(id: impl Into<ElementId>, progress: f32, child: AnyElement));
