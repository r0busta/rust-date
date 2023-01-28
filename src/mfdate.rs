use chrono::{prelude::NaiveDate, Datelike, NaiveDateTime};

pub struct MfDate {
    base: NaiveDate,
}

pub struct MfDateIterator {
    current: MfDate,
    end: MfDate,
}

impl Iterator for MfDateIterator {
    type Item = MfDate;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.day_before_or_eq(self.end) {
            let current = self.current.clone();
            self.current = self.current.add_months_reset_day(1);
            return Some(current);
        }

        None
    }
}

impl MfDate {
    pub fn year(&self) -> i32 {
        self.base.year()
    }

    pub fn month(&self) -> u32 {
        self.base.month()
    }

    pub fn next_before_eq(&self, arg: MfDate) -> MfDateIterator {
        MfDateIterator {
            current: self.clone(),
            end: arg,
        }
    }

    pub fn year_month_eq(&self, arg: MfDate) -> bool {
        return self.year() == arg.year() && self.month() == arg.month();
    }

    pub fn day_before_or_eq(&self, arg: MfDate) -> bool {
        if self.year_month_eq(arg) {
            return true;
        }

        return self.day_before(arg);
    }

    pub fn day_before(&self, arg: MfDate) -> bool {
        if self.year() < arg.year() {
            return true;
        }

        return self.year() == arg.year() && self.month() < arg.month();
    }

    pub fn add_months_reset_day(&self, months: i32) -> MfDate {
        let mut new_date = self.base.clone();
        new_date = new_date.with_day(1).unwrap();
        if months > 0 {
            new_date = new_date + chrono::Months::new(months as u32);
        } else {
            new_date = new_date - chrono::Months::new(months.abs() as u32);
        }

        MfDate { base: new_date }
    }

    pub fn from_date_time(time: NaiveDateTime) -> MfDate {
        MfDate {
            base: NaiveDate::from_ymd_opt(time.year(), time.month(), time.day()).unwrap(),
        }
    }

    pub fn from_date(year: i32, month: u32, day: u32) -> MfDate {
        MfDate {
            base: NaiveDate::from_ymd_opt(year, month, day).unwrap(),
        }
    }

    pub fn format(&self, format: &str) -> String {
        self.base.format(format).to_string()
    }
}

impl Copy for MfDate {}

impl Clone for MfDate {
    fn clone(&self) -> Self {
        *self
    }
}

impl PartialEq for MfDate {
    fn eq(&self, other: &Self) -> bool {
        self.base == other.base
    }
}

impl Eq for MfDate {}

impl std::fmt::Display for MfDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format("%b%e, %Y"))
    }
}

impl std::fmt::Debug for MfDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
