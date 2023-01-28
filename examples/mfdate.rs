use rust_date::mfdate::MfDate;

fn main() {
    let start = MfDate::from_date(2021, 8, 31);
    let end = MfDate::from_date(2022, 2, 28);

    // Create a range of months.
    let mut months = vec![];

    let mut date = start.add_months_reset_day(0);
    while date.day_before_or_eq(end) {
        months.push(date);

        date = date.add_months_reset_day(1);
    }
    println!("{:?}", months);

    // Create a range of months using an iterator.
    let mut months = vec![];
    for date in start.add_months_reset_day(0).next_before_eq(end) {
        months.push(date);
    }
    println!("{:?}", months)
}
