use chrono::{Datelike, Utc, TimeZone};

fn main() {
    let now = Utc::now();
    let stt = now.timestamp();
    let ddl = Utc.ymd(2020, 12, 25).and_hms(16, 0, 0).timestamp();
    // let ddl = Utc.ymd(2101, 2, 12).and_hms(16, 0, 0).timestamp();
    let lft = (ddl - stt) / 3600 / 24;

    /*
    let (is_common_era, year) = now.year_ce();
    println!(
        "Current UTC date: {}-{:02}-{:02} {:?} ({})",
        year,
        now.month(),
        now.day(),
        now.weekday(),
        if is_common_era { "CE" } else { "BCE" }
    );
    */

    if lft > 1 {
        println!("{} days left. Do your best, man!", lft);
    } else if lft < 0 {
        println!("How do you do, man?");
    } else {
        println!("Only 1 day left! Fight!");
    }

    /*
    if lft > 1 {
        println!("{} days left. Do your best, man!", lft);
    } else if lft < 0 {
        println!("How do you do, man?");
    } else {
        println!("Are you still alive, old man? If so, just hold on for a while.");
        println!("Maybe the Emacs and Windows are already outdated, haha. BTW, this program was written in Rust.");
        println!("How does it feel to live for 'three centuries'?");
        println!("You were a young man in his 20s when this countdown started.");
        println!("And now I believe that you have seen everything, everything. Life is tragedy, you know.");
        println!("How do you think of your life?");
    }
    */
}
