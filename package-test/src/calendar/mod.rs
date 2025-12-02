//! Calendar data module for fetching earnings, dividend, and split events

mod calendar;

pub use calendar::{
    // Functions
    get_earnings_calendar,
    get_dividend_calendar,
    get_split_info,
    get_full_calendar,
    get_calendars_for_symbols,
    get_calendar_raw,
    // Types
    EarningsEvent,
    DividendEvent,
    SplitEvent,
    CalendarData,
};
