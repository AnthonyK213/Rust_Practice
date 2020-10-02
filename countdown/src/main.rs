use chrono::{Datelike, Utc, TimeZone};

fn main() {
    let now = Utc::now();
    let stt = now.timestamp();
    let ddl = Utc.ymd(2020, 12, 26).and_hms(0, 0, 0).timestamp();
    let lft = (ddl - stt) / 3600 / 24;

    let (is_common_era, year) = now.year_ce();
    println!(
        "Current UTC date: {}-{:02}-{:02} {:?} ({})",
        year,
        now.month(),
        now.day(),
        now.weekday(),
        if is_common_era { "CE" } else { "BCE" }
    );

    if lft > 1 {
        println!("{} days left. Do your best, man!", lft);
    } else if lft < 0 {
        println!("How do you do, man?");
    } else {
        println!("Only 1 day left! Fight!");
    }
}
