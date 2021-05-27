use device_query::{DeviceQuery, DeviceState, Keycode};
use std::env;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use std::{thread, time};

fn main() {
    let device_state = DeviceState::new();
    let sleep_time = time::Duration::from_millis(50);
    let path = env::current_exe().unwrap();
    let log_path = path.parent().unwrap().join("qia.log");
    let file = File::open(&log_path).unwrap();
    let reader = BufReader::new(file);
    let mut content: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line != "" {
            content.push(line);
        }
    }
    let mut n = 0;
    let place = [
        "西1f",
        "西嘬面",
        "学自选",
        "西自选",
        "学2f",
        "四食堂",
        "西3f",
    ];
    loop {
        let keys: Vec<Keycode> = device_state.get_keys();
        let index = n % place.len();
        if keys.is_empty() {
            println!("哪儿恰? {}?", place[index]);
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            thread::sleep(sleep_time);
            n += 1;
        } else {
            if keys[0] == Keycode::Q {
                let last_place = content.last().unwrap();
                let p_res = place[index];
                let res: &str = if p_res == last_place.as_str() {
                    place[(index + 1) % place.len()]
                } else {
                    place[index]
                };
                println!("{}", res);
                let mut log_file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .create(true)
                    .open(&log_path)
                    .unwrap();
                if let Err(e) = log_file.write(format!("{}\n", res).as_bytes()) {
                    eprintln!("Couldn't write to file: {}", e);
                };
                break;
            }
        }
    }
}

// fn coc_test(a: u32, b: u32) -> u32 {
//     a.pow(b)
// }
