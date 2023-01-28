use chrono::TimeZone;
use rust_date::mfdate::MfDate;

#[test]
fn mfdate_from_date_time() {
    {
        let time = chrono::NaiveDateTime::default();
        let date = MfDate::from_date_time(time);

        assert_eq!(date.year(), 1970);
        assert_eq!(date.month(), 1);
    }

    {
        let time = chrono::Utc.with_ymd_and_hms(2021, 1, 1, 0, 0, 1).unwrap();
        let date_time = chrono::NaiveDateTime::from_timestamp_opt(time.timestamp(), 0).unwrap();
        let date = MfDate::from_date_time(date_time);

        assert_eq!(date.year(), 2021);
        assert_eq!(date.month(), 1);
    }
}

#[test]
fn mfdate_copy() {
    {
        let date = MfDate::from_date(2021, 8, 31);
        let copy = date;

        assert_eq!(date, copy);
    }
}

#[test]
fn mfdate_clone() {
    {
        let date = MfDate::from_date(2021, 8, 31);
        let clone = date.clone();

        assert_eq!(date, clone);
    }
}

#[test]
fn mf_display() {
    {
        let date = MfDate::from_date(2021, 8, 1);
        let string = format!("{}", date);

        assert_eq!(string, "Aug 1, 2021");
    }

    {
        let time = chrono::Utc.with_ymd_and_hms(2021, 10, 4, 0, 0, 1).unwrap();
        let date_time = chrono::NaiveDateTime::from_timestamp_opt(time.timestamp(), 0).unwrap();
        let date = MfDate::from_date_time(date_time);
        let string = format!("{}", date);

        assert_eq!(string, "Oct 4, 2021");
    }

    {
        let time = chrono::Utc
            .with_ymd_and_hms(2021, 10, 4, 23, 59, 59)
            .unwrap();
        let date_time = chrono::NaiveDateTime::from_timestamp_opt(time.timestamp(), 0).unwrap();
        let date = MfDate::from_date_time(date_time);
        let string = format!("{}", date);

        assert_eq!(string, "Oct 4, 2021");
    }
}

#[test]
fn mf_format() {
    {
        let date = MfDate::from_date(2021, 8, 1);
        let string = format!("{}", date.format("%D"));

        assert_eq!(string, "08/01/21");
    }
}

#[test]
fn mfdate_eq() {
    {
        // the same day
        let date = MfDate::from_date(2021, 8, 31);
        let arg = MfDate::from_date(2021, 8, 31);

        assert_eq!(date, arg);
    }

    {
        // one day after
        let date = MfDate::from_date(2021, 9, 1);
        let arg = MfDate::from_date(2021, 8, 31);

        assert_ne!(date, arg);
    }

    {
        // one day before and an earlier month
        let date = MfDate::from_date(2021, 7, 31);
        let arg = MfDate::from_date(2021, 8, 1);

        assert_ne!(date, arg);
    }

    {
        // one month before
        let date = MfDate::from_date(2021, 7, 31);
        let arg = MfDate::from_date(2021, 8, 31);

        assert_ne!(date, arg);
    }

    {
        // one year before
        let date = MfDate::from_date(2020, 8, 1);
        let arg = MfDate::from_date(2021, 8, 1);

        assert_ne!(date, arg);
    }
}

#[test]
fn mfdate_year_month_equal() {
    {
        let time = chrono::NaiveDateTime::default();
        let date = MfDate::from_date_time(time);
        let arg = MfDate::from_date_time(time);

        assert_eq!(date.year_month_eq(arg), true);
    }

    {
        let time = chrono::Utc.with_ymd_and_hms(2021, 1, 1, 0, 0, 1).unwrap();
        let date_time = chrono::NaiveDateTime::from_timestamp_opt(time.timestamp(), 0).unwrap();
        let date = MfDate::from_date_time(date_time);
        let arg = MfDate::from_date_time(date_time);

        assert_eq!(date.year_month_eq(arg), true);
    }

    {
        let date = MfDate::from_date(0, 1, 1);
        let arg = MfDate::from_date(0, 1, 1);

        assert_eq!(date.year_month_eq(arg), true);
    }

    {
        let date = MfDate::from_date(-48, 1, 15);
        let arg = MfDate::from_date(-48, 1, 15);

        assert_eq!(date.year_month_eq(arg), true);
    }

    {
        let date = MfDate::from_date(2021, 8, 31);
        let arg = MfDate::from_date(2021, 8, 31);

        assert_eq!(date.year_month_eq(arg), true);
    }

    {
        let date = MfDate::from_date(2021, 9, 30);
        let arg = MfDate::from_date(2021, 8, 31);

        assert_eq!(date.year_month_eq(arg), false);
    }
}

#[test]
fn mfdate_day_before_or_eq() {
    {
        // the same day
        let date = MfDate::from_date(2021, 8, 31);
        let arg = date;

        assert_eq!(date.day_before_or_eq(arg), true);
    }

    {
        // the same day
        let date = MfDate::from_date(2021, 8, 31);
        let arg = MfDate::from_date(2021, 8, 31);

        assert_eq!(date.day_before_or_eq(arg), true);
    }

    {
        // one day before
        let date = MfDate::from_date(2021, 8, 30);
        let arg = MfDate::from_date(2021, 8, 31);

        assert_eq!(date.day_before_or_eq(arg), true);
    }

    {
        // one day after
        let date = MfDate::from_date(2021, 9, 1);
        let arg = MfDate::from_date(2021, 8, 31);

        assert_eq!(date.day_before_or_eq(arg), false);
    }

    {
        // one day before, past month
        let date = MfDate::from_date(2021, 7, 31);
        let arg = MfDate::from_date(2021, 8, 1);

        assert_eq!(date.day_before_or_eq(arg), true);
    }

    {
        // one year before
        let date = MfDate::from_date(2020, 8, 1);
        let arg = MfDate::from_date(2021, 8, 1);

        assert_eq!(date.day_before_or_eq(arg), true);
    }

    {
        // one day after
        let date = MfDate::from_date(2021, 9, 1);
        let arg = MfDate::from_date(2021, 8, 31);

        assert_eq!(date.day_before_or_eq(arg), false);
    }
}

#[test]
fn mfdate_day_before() {
    {
        // one day after
        let date = MfDate::from_date(2021, 9, 1);
        let arg = MfDate::from_date(2021, 8, 31);

        assert_eq!(date.day_before(arg), false);
    }

    {
        // the same day
        let date = MfDate::from_date(2021, 8, 31);
        let arg = MfDate::from_date(2021, 8, 31);

        assert_eq!(date.day_before(arg), false);
    }

    {
        // one day before and a prev month
        let date = MfDate::from_date(2021, 7, 31);
        let arg = MfDate::from_date(2021, 8, 31);

        assert_eq!(date.day_before(arg), true);
    }

    {
        // one month before
        let date = MfDate::from_date(2021, 7, 31);
        let arg = MfDate::from_date(2021, 8, 1);

        assert_eq!(date.day_before(arg), true);
    }

    {
        // one year before
        let date = MfDate::from_date(2020, 8, 1);
        let arg = MfDate::from_date(2021, 8, 1);

        assert_eq!(date.day_before(arg), true);
    }
}

#[test]
fn mfdate_add_months_reset_day() {
    {
        let date = MfDate::from_date(2020, 8, 1);
        let arg = 0;
        let want = MfDate::from_date(2020, 8, 1);

        assert_eq!(date.add_months_reset_day(arg), want);
    }

    {
        let date = MfDate::from_date(2020, 8, 1);
        let arg = 1;
        let want = MfDate::from_date(2020, 9, 1);

        assert_eq!(date.add_months_reset_day(arg), want);
    }

    {
        let date = MfDate::from_date(2020, 8, 1);
        let arg = 2;
        let want = MfDate::from_date(2020, 10, 1);

        assert_eq!(date.add_months_reset_day(arg), want);
    }

    {
        let date = MfDate::from_date(2020, 8, 31);
        let arg = 1;
        let want = MfDate::from_date(2020, 9, 1);

        assert_eq!(date.add_months_reset_day(arg), want);
    }

    {
        let date = MfDate::from_date(2020, 8, 31);
        let arg = -1;
        let want = MfDate::from_date(2020, 7, 1);

        assert_eq!(date.add_months_reset_day(arg), want);
    }

    {
        let date = MfDate::from_date(2020, 1, 31);
        let arg = -12;
        let want = MfDate::from_date(2019, 1, 1);

        assert_eq!(date.add_months_reset_day(arg), want);
    }

    {
        let date = MfDate::from_date(2020, 1, 31);
        let arg = -13;
        let want = MfDate::from_date(2018, 12, 1);

        assert_eq!(date.add_months_reset_day(arg), want);
    }
}
