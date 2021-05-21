// Author: AnthonyK213
#![allow(unused)]

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use std::collections::HashMap;
use std::env;
use std::iter::FromIterator;

fn main() {
    // Map the string to integer.
    let table = [
        'f', 'Z', 'o', 'd', 'R', '9', 'X', 'Q', 'D', 'S', 'U', 'm', '2', '1', 'y', 'C', 'k', 'r',
        '6', 'z', 'B', 'q', 'i', 'v', 'e', 'Y', 'a', 'h', '8', 'b', 't', '4', 'x', 's', 'W', 'p',
        'H', 'n', 'J', 'E', '7', 'j', 'L', '5', 'V', 'G', '3', 'g', 'u', 'M', 'T', 'K', 'N', 'P',
        'A', 'w', 'c', 'F',
    ];
    let mut map = HashMap::new();
    let mut k: u64 = 0;
    for c in table.iter() {
        map.insert(c, k);
        k += 1;
    }

    let num_a: u64 = 100618342136696320;
    let num_b: u64 = 177451812;

    let args: Vec<String> = env::args().collect();
    let option: &str = &args[1];
    let prm = &args[2];

    match option {
        "b" => {
            if prm.len() < 13 {
                println!("Input a valid url, moron!");
            } else {
                let mut av_str = String::from("av");
                match &prm.find("/BV") {
                    Some(index) => {
                        let bv_str = (&prm[(index + 3)..(index + 13)]).to_string();
                        av_str += &b2a(&bv_str, &map, num_a, num_b).to_string();
                        println!("{}", av_str);
                        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                        ctx.set_contents(av_str.to_owned()).unwrap();
                    }
                    None => println!("Input a valid url, moron!"),
                };
            }
        }
        "a" => match prm.parse::<u64>() {
            Ok(av_num) => {
                let mut bv_url = String::from("http://www.bilibili.com/video/BV");
                let bv_num = a2b(av_num, &table, num_a, num_b);
                bv_url += &bv_num.to_string();
                println!("BV{}", bv_num);
                let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                ctx.set_contents(bv_url.to_owned()).unwrap();
            }
            Err(err) => println!("Error: {:?}", err),
        },
        _ => println!("Gimme a correct option, bitch!"),
    };
}

fn b2a(bv: &str, map: &HashMap<&char, u64>, num_a: u64, num_b: u64) -> u64 {
    let power = [6, 2, 4, 8, 5, 9, 3, 7, 1, 0];

    let bv_vec: Vec<char> = bv.chars().collect();

    let mut s: u64 = 0;
    for i in 0..10 {
        s += map.get(&bv_vec[i]).unwrap() * 58_u64.pow(power[i])
    }
    (s - num_a) ^ num_b
}

fn a2b(av: u64, map: &[char], num_a: u64, num_b: u64) -> String {
    let pos = [5, 3, 7, 0, 4, 2, 6, 1, 8, 9];
    let mut lst: [char; 10] = ['0'; 10];

    let mut s = (av ^ num_b) + num_a;

    for i in 0..10 {
        lst[pos[i]] = map[(s / 58_u64.pow((9 - i) as u32)) as usize];
        s = s % 58_u64.pow((9 - i) as u32);
    }
    String::from_iter(&lst)
}
