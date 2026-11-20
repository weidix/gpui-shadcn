//! Overlay composition and open-state handles.

use gpui::*;
use std::time::Duration;

use crate::Button;

pub use crate::tooltip::{Tooltip, TooltipSide};

element!(Popover(gpui_base::Popover) + Styled);
constructor!(Popover(id: impl Into<ElementId>));

/// A handle for closing a popover from its content.
#[derive(Clone)]
pub struct PopoverHandle(WeakEntity<gpui_base::PopoverState>);

impl PopoverHandle {
    pub fn dismiss(&self, window: &mut Window, cx: &mut App) {
        let _ = self.0.update(cx, |state, cx| state.dismiss(window, cx));
    }
}

impl Popover {
    pub fn trigger(self, trigger: Button) -> Self {
        Self(self.0.trigger_with(move |open, _, _| {
            let selected = trigger.is_selected();
            trigger.selected(selected || open).into_any_element()
        }))
    }

    pub fn content<E: IntoElement>(
        self,
        build: impl FnOnce(PopoverHandle, &mut Window, &mut App) -> E + 'static,
    ) -> Self {
        Self(
            self.0
                .content(move |_, window, cx| build(PopoverHandle(cx.weak_entity()), window, cx)),
        )
    }

    builders! {
        anchor(anchor: impl Into<Anchor>);
        offset(offset: Pixels);
        open(open: bool);
        default_open(open: bool);
        track_focus(focus: &FocusHandle);
        overlay_closable(closable: bool);
        on_open_change(handler: impl Fn(&bool, &mut Window, &mut App) + 'static);
    }
}

element!(HoverCard(gpui_base::HoverCard));
constructor!(HoverCard(id: impl Into<ElementId>));

impl HoverCard {
    pub fn content(
        self,
        build: impl FnOnce(&mut Window, &mut App) -> Stateful<Div> + 'static,
    ) -> Self {
        Self(self.0.content(move |_, window, cx| build(window, cx)))
    }

    builders! {
        trigger(trigger: impl IntoElement);
        anchor(anchor: impl Into<Anchor>);
        open_delay(duration: Duration);
        close_delay(duration: Duration);
    }
}

/// A dialog's open state, shared with its trigger and content.
#[derive(Clone)]
pub struct DialogHandle(gpui_base::DialogHandle);

impl DialogHandle {
    pub fn new(open: bool) -> Self {
        Self(gpui_base::DialogHandle::new(open))
    }

    pub fn is_open(&self) -> bool {
        self.0.is_open()
    }

    pub fn open(&self, window: &mut Window, cx: &mut App) {
        self.0.open(window, cx);
    }

    pub fn close(&self, window: &mut Window, cx: &mut App) {
        self.0.close(window, cx);
    }
}

/// The interaction that changed a dialog's open state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DialogChangeReason {
    TriggerPress,
    BackdropPress,
    Cancel,
    Confirm,
    Imperative,
}

impl DialogChangeReason {
    fn from_backend(reason: gpui_base::DialogChangeReason) -> Self {
        use gpui_base::DialogChangeReason as Reason;

        match reason {
            Reason::TriggerPress => Self::TriggerPress,
            Reason::BackdropPress => Self::BackdropPress,
            Reason::Cancel => Self::Cancel,
            Reason::Confirm => Self::Confirm,
            Reason::Imperative => Self::Imperative,
        }
    }
}

macro_rules! dialog {
    ($name:ident) => {
        element!($name(gpui_base::$name) + Styled + Parent);
        constructor!($name(cx: &mut App));

        impl $name {
            pub fn handle(self, handle: DialogHandle) -> Self {
                Self(self.0.handle(handle.0))
            }

            pub fn on_open_change(
                self,
                handler: impl Fn(bool, DialogChangeReason, &mut Window, &mut App) + 'static,
            ) -> Self {
                Self(self.0.on_open_change(move |open, reason, window, cx| {
                    handler(open, DialogChangeReason::from_backend(reason), window, cx);
                }))
            }

            builders! {
                open(open: bool);
                focus_handle(focus: FocusHandle);
                backdrop(backdrop: impl IntoElement);
                popup(popup: impl IntoElement);
                close_on_escape(enabled: bool);
                request_close(handler: impl Fn(bool, &mut Window, &mut App) + 'static);
            }
        }
    };
}

dialog!(Dialog);
dialog!(AlertDialog);

impl Dialog {
    builders! {
        close_on_backdrop_press(enabled: bool);
    }
}

element!(DialogTitle(gpui_base::DialogTitle) + Styled + Parent);
element!(DialogDescription(gpui_base::DialogDescription) + Styled + Parent);
element!(DialogBackdrop(gpui_base::DialogBackdrop) + Styled + Parent);
element!(DialogPopup(gpui_base::DialogPopup) + Styled + Parent);
element!(AlertDialogTitle(gpui_base::AlertDialogTitle) + Styled + Parent);
element!(AlertDialogDescription(gpui_base::AlertDialogDescription) + Styled + Parent);
element!(AlertDialogBackdrop(gpui_base::AlertDialogBackdrop) + Styled + Parent);
element!(AlertDialogPopup(gpui_base::AlertDialogPopup) + Styled + Parent);
constructor!(DialogTitle());
constructor!(DialogDescription());
constructor!(DialogBackdrop());
constructor!(DialogPopup());
constructor!(AlertDialogTitle());
constructor!(AlertDialogDescription());
constructor!(AlertDialogBackdrop());
constructor!(AlertDialogPopup());

element!(Sheet(gpui_base::Sheet) + Styled);
constructor!(Sheet(cx: &mut App));

impl Sheet {
    builders! {
        overlay(overlay: impl IntoElement);
        surface(surface: impl IntoElement);
        focus_handle(focus: FocusHandle);
        request_close(handler: impl Fn(&mut Window, &mut App) + 'static);
    }
}

element!(Toast(gpui_base::Toast) + Styled + Parent + Interactive);
constructor!(Toast(id: impl Into<ElementId>));

element!(Positioner(gpui_base::Positioner) + Parent);

impl Positioner {
    pub fn corner(anchor: Anchor, position: Point<Pixels>) -> Self {
        Self(gpui_base::Positioner::corner(anchor, position))
    }

    pub fn side(trigger: Bounds<Pixels>) -> Self {
        Self(gpui_base::Positioner::side(trigger))
    }

    builders! {
        occlude();
        offset(offset: Pixels);
        margin(margin: Pixels);
    }
}
