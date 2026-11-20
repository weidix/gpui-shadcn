//! Date and date-range selection.

use chrono::NaiveDate;
use gpui::{prelude::FluentBuilder, *};

use crate::Theme;

/// A calendar's selected date or range, including an in-progress selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DateSelection {
    Single(Option<NaiveDate>),
    Range(Option<NaiveDate>, Option<NaiveDate>),
}

impl DateSelection {
    fn from_backend(date: gpui_base::Date) -> Self {
        match date {
            gpui_base::Date::Single(date) => Self::Single(date),
            gpui_base::Date::Range(start, end) => Self::Range(start, end),
        }
    }

    fn backend(self) -> gpui_base::Date {
        match self {
            Self::Single(date) => gpui_base::Date::Single(date),
            Self::Range(start, end) => gpui_base::Date::Range(start, end),
        }
    }

    pub fn start(self) -> Option<NaiveDate> {
        self.backend().start()
    }

    pub fn format(self, pattern: &str) -> Option<SharedString> {
        self.backend().format(pattern)
    }
}

/// A user selected a date or completed a range.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CalendarEvent(pub DateSelection);

/// The selection and current view of a calendar.
pub struct CalendarState {
    calendar: Entity<gpui_base::CalendarState>,
    _subscriptions: [Subscription; 2],
}

impl CalendarState {
    pub fn new(selection: DateSelection, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let calendar = cx.new(|cx| {
            let mut state = gpui_base::CalendarState::new(window, cx);
            state.apply_date(selection.backend());
            state
        });
        let event = cx.subscribe(&calendar, |_, _, event, cx| {
            let gpui_base::CalendarEvent::Selected(date) = *event;
            cx.emit(CalendarEvent(DateSelection::from_backend(date)));
        });
        // A range's first endpoint changes the value before selection is complete.
        let mut last_date = selection;
        let changes = cx.observe(&calendar, move |_, calendar, cx| {
            let date = DateSelection::from_backend(calendar.read(cx).date());
            if last_date != date {
                last_date = date;
                cx.notify();
            }
        });
        Self {
            calendar,
            _subscriptions: [event, changes],
        }
    }

    pub fn date(&self, cx: &App) -> DateSelection {
        DateSelection::from_backend(self.calendar.read(cx).date())
    }

    pub fn set_date(&mut self, date: DateSelection, window: &mut Window, cx: &mut Context<Self>) {
        self.calendar
            .update(cx, |state, cx| state.set_date(date.backend(), window, cx));
    }
}

impl EventEmitter<CalendarEvent> for CalendarState {}

/// A themed calendar that can be used in a surface or a date picker.
#[derive(IntoElement)]
pub struct Calendar {
    id: ElementId,
    state: Entity<CalendarState>,
    months: usize,
}

impl Calendar {
    pub fn new(id: impl Into<ElementId>, state: &Entity<CalendarState>) -> Self {
        Self {
            id: id.into(),
            state: state.clone(),
            months: 1,
        }
    }

    pub fn months(mut self, count: usize) -> Self {
        self.months = count.max(1);
        self
    }
}

impl RenderOnce for Calendar {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        use gpui_base::CalendarItemKind as Kind;

        let t = Theme::global(cx);
        let months = self.months;
        gpui_base::Calendar::new(self.id, &self.state.read(cx).calendar)
            .number_of_months(months)
            .when(months > 1, |grid| grid.w(t.unit(240. * months as f32)))
            .label(|kind, value| match kind {
                Kind::Weekday => ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"][value as usize].into(),
                Kind::Month | Kind::MonthToggle => [
                    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov",
                    "Dec",
                ]
                .get(value.saturating_sub(1) as usize)
                .copied()
                .unwrap_or("Month")
                .into(),
                Kind::Previous => "‹".into(),
                Kind::Next => "›".into(),
                _ => value.to_string().into(),
            })
            .item(move |item, state, _, _| {
                let show_selection = months == 1 || !state.is_muted();
                let active = show_selection && state.is_active();
                item.flex()
                    .items_center()
                    .justify_center()
                    .h(t.unit(32.))
                    .min_w(t.unit(32.))
                    .px(t.unit(4.))
                    .rounded(t.radius_control())
                    .text_color(if active {
                        t.primary_foreground
                    } else if state.is_muted() {
                        t.muted_foreground
                    } else {
                        t.foreground
                    })
                    .when(show_selection && state.is_in_range() && !active, |s| {
                        s.rounded_none().bg(t.muted)
                    })
                    .when(active, |s| s.bg(t.primary))
                    .when(show_selection && state.is_today() && !active, |s| {
                        s.bg(t.muted)
                    })
                    .when(!state.is_disabled() && !active, |s| {
                        s.cursor_pointer().hover(move |s| s.bg(t.muted))
                    })
                    .into_any_element()
            })
    }
}
