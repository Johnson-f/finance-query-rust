use chrono::{Datelike, NaiveDate, NaiveTime, TimeZone, Weekday};
use chrono_tz::America::New_York;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum MarketStatus {
    Open,
    Closed,
    EarlyClose,
}

impl MarketStatus {
    pub fn as_str(&self) -> &str {
        match self {
            MarketStatus::Open => "Open",
            MarketStatus::Closed => "Closed",
            MarketStatus::EarlyClose => "Early Close",
        }
    }
}

pub struct MarketSchedule {
    year: i32,
    regular_open: NaiveTime,
    regular_close: NaiveTime,
    early_close_time: NaiveTime,
    full_holidays: HashMap<NaiveDate, String>,
    early_close_dates: HashMap<NaiveDate, String>,
}

impl MarketSchedule {
    pub fn new() -> Self {
        let year = chrono::Utc::now().year();
        let mut schedule = Self {
            year,
            regular_open: NaiveTime::from_hms_opt(9, 30, 0).unwrap(),
            regular_close: NaiveTime::from_hms_opt(16, 0, 0).unwrap(),
            early_close_time: NaiveTime::from_hms_opt(13, 0, 0).unwrap(),
            full_holidays: HashMap::new(),
            early_close_dates: HashMap::new(),
        };
        schedule.calculate_holidays();
        schedule
    }

    fn get_nth_weekday_of_month(&self, year: i32, month: u32, weekday: Weekday, n: u32) -> NaiveDate {
        let first_day = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
        let first_weekday = first_day.weekday();
        
        let days_until = (weekday.number_from_monday() as i32 - first_weekday.number_from_monday() as i32 + 7) % 7;
        first_day + chrono::Duration::days(days_until as i64 + ((n - 1) * 7) as i64)
    }

    fn get_last_monday_of_month(&self, year: i32, month: u32) -> NaiveDate {
        let next_month = if month == 12 { 1 } else { month + 1 };
        let next_year = if month == 12 { year + 1 } else { year };
        let first_day_next = NaiveDate::from_ymd_opt(next_year, next_month, 1).unwrap();
        let last_day = first_day_next - chrono::Duration::days(1);
        
        let days_back = (last_day.weekday().number_from_monday() as i32 - 1 + 7) % 7;
        last_day - chrono::Duration::days(days_back as i64)
    }

    fn get_good_friday(&self, year: i32) -> NaiveDate {
        // Butcher's Algorithm for calculating Easter
        let a = year % 19;
        let b = year / 100;
        let c = year % 100;
        let d = b / 4;
        let e = b % 4;
        let f = (b + 8) / 25;
        let g = (b - f + 1) / 3;
        let h = (19 * a + b - d - g + 15) % 30;
        let i = c / 4;
        let k = c % 4;
        let l = (32 + 2 * e + 2 * i - h - k) % 7;
        let m = (a + 11 * h + 22 * l) / 451;

        let month = (h + l - 7 * m + 114) / 31;
        let day = ((h + l - 7 * m + 114) % 31) + 1;

        // Easter Sunday
        let easter = NaiveDate::from_ymd_opt(year, month as u32, day as u32).unwrap();
        // Good Friday is two days before Easter
        easter - chrono::Duration::days(2)
    }

    fn calculate_holidays(&mut self) {
        // Fixed date holidays
        self.full_holidays.insert(
            NaiveDate::from_ymd_opt(self.year, 1, 1).unwrap(),
            "New Year's Day".to_string(),
        );
        self.full_holidays.insert(
            NaiveDate::from_ymd_opt(self.year, 6, 19).unwrap(),
            "Juneteenth".to_string(),
        );
        self.full_holidays.insert(
            NaiveDate::from_ymd_opt(self.year, 7, 4).unwrap(),
            "Independence Day".to_string(),
        );
        self.full_holidays.insert(
            NaiveDate::from_ymd_opt(self.year, 12, 25).unwrap(),
            "Christmas Day".to_string(),
        );

        // Weekday-based holidays
        self.full_holidays.insert(
            self.get_nth_weekday_of_month(self.year, 1, Weekday::Mon, 3),
            "Martin Luther King Jr. Day".to_string(),
        );
        self.full_holidays.insert(
            self.get_nth_weekday_of_month(self.year, 2, Weekday::Mon, 3),
            "Presidents Day".to_string(),
        );
        self.full_holidays.insert(
            self.get_last_monday_of_month(self.year, 5),
            "Memorial Day".to_string(),
        );
        self.full_holidays.insert(
            self.get_nth_weekday_of_month(self.year, 9, Weekday::Mon, 1),
            "Labor Day".to_string(),
        );
        self.full_holidays.insert(
            self.get_nth_weekday_of_month(self.year, 11, Weekday::Thu, 4),
            "Thanksgiving Day".to_string(),
        );
        self.full_holidays.insert(
            self.get_good_friday(self.year),
            "Good Friday".to_string(),
        );

        // Early closure dates
        self.early_close_dates.insert(
            NaiveDate::from_ymd_opt(self.year, 7, 3).unwrap(),
            "July 3rd".to_string(),
        );
        let thanksgiving = self.get_nth_weekday_of_month(self.year, 11, Weekday::Thu, 4);
        self.early_close_dates.insert(
            thanksgiving + chrono::Duration::days(1),
            "Black Friday".to_string(),
        );
        self.early_close_dates.insert(
            NaiveDate::from_ymd_opt(self.year, 12, 24).unwrap(),
            "Christmas Eve".to_string(),
        );

        // Adjust weekend holidays
        self.adjust_weekend_holidays();
    }

    fn adjust_weekend_holidays(&mut self) {
        let mut weekend_adjustments = HashMap::new();
        let mut weekend_removals = Vec::new();

        for (holiday_date, holiday_name) in &self.full_holidays.clone() {
            // Skip holidays that are already calculated to avoid weekends
            if holiday_name == "Martin Luther King Jr. Day"
                || holiday_name == "Presidents Day"
                || holiday_name == "Memorial Day"
                || holiday_name == "Labor Day"
                || holiday_name == "Thanksgiving Day"
                || holiday_name == "Good Friday"
            {
                continue;
            }

            let weekday = holiday_date.weekday();
            if weekday == Weekday::Sat {
                weekend_adjustments.insert(*holiday_date - chrono::Duration::days(1), holiday_name.clone());
                weekend_removals.push(*holiday_date);
            } else if weekday == Weekday::Sun {
                weekend_adjustments.insert(*holiday_date + chrono::Duration::days(1), holiday_name.clone());
                weekend_removals.push(*holiday_date);
            }
        }

        for date_to_remove in weekend_removals {
            self.full_holidays.remove(&date_to_remove);
        }
        self.full_holidays.extend(weekend_adjustments);
    }

    pub fn get_market_status(&self) -> (MarketStatus, Option<String>) {
        let now_et = New_York.from_utc_datetime(&chrono::Utc::now().naive_utc());
        let current_date = now_et.date_naive();
        let current_time = now_et.time();

        // Check if it's a weekend
        if now_et.weekday() == Weekday::Sat || now_et.weekday() == Weekday::Sun {
            return (MarketStatus::Closed, Some("Weekend".to_string()));
        }

        // Check if it's a holiday
        if let Some(holiday_name) = self.full_holidays.get(&current_date) {
            return (MarketStatus::Closed, Some(format!("Holiday: {}", holiday_name)));
        }

        // Check if it's an early closure day
        if let Some(early_close_name) = self.early_close_dates.get(&current_date) {
            if current_time < self.regular_open {
                return (MarketStatus::Closed, Some("Pre-market".to_string()));
            } else if current_time >= self.early_close_time {
                return (
                    MarketStatus::Closed,
                    Some(format!("Early Close: {}", early_close_name)),
                );
            } else {
                return (
                    MarketStatus::EarlyClose,
                    Some(format!("Early Close Day: {}", early_close_name)),
                );
            }
        }

        // Regular trading day logic
        if current_time < self.regular_open {
            (MarketStatus::Closed, Some("Pre-market".to_string()))
        } else if current_time >= self.regular_close {
            (MarketStatus::Closed, Some("After-hours".to_string()))
        } else {
            (MarketStatus::Open, Some("Regular trading hours".to_string()))
        }
    }
}

impl Default for MarketSchedule {
    fn default() -> Self {
        Self::new()
    }
}

