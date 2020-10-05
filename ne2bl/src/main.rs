#![allow(unused)]

use std::io::{BufRead, BufReader};
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let dir_name: &str = &args[1];

    let ne_dir = "./";
    // let ne_dir = "D:/Anthony/Desktop/";
    let dir_path = &format!("{}{}", ne_dir, dir_name);

    let mut bib_list: String = String::from("");
    let mut num: u16 = 0;

    let paths = fs::read_dir(dir_path).unwrap();
    for path in paths {
        let file_path = path.unwrap().path();
        if file_path.extension().unwrap() == "net" {
            bib_list.push_str(&ne2bl(&file_path, num));
            num += 1;
        }
    }

    fs::write(format!("{}/ref.bib", ne_dir), bib_list).unwrap();
}

fn ne2bl(ne_path: &Path, num: u16) -> String {
    let file = fs::File::open(ne_path).unwrap();
    let reader = BufReader::new(file);
    let mut content: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line != "" {
            content.push(line);
        }
    }

    let mut bibtext: String = "".to_string();

    for line in content {
        let ne_line: Vec<&str> = line.split("}: ").collect();
        let new_key: &str = &ne_line[0][1..].to_lowercase().as_str().replace(" ", "");
        let ne_disc: &str = ne_line[1];
        match new_key {
            "referencetype" => {
                match ne_disc {
                    "Journal Article" => bibtext.push_str(&format!("@article{{{},\n", &num)),
                    "Conference Proceedings" => bibtext.push_str(&format!("@inproceedings{{{},\n", &num)),
                    "Thesis" => bibtext.push_str(&format!("@mastersthesis{{{},\n", &num)),
                    _ => println!("Something else?")
                }
            }, 
            "issue" => bibtext.push_str(&format!("  number={{{}}},\n", &ne_disc)), 
            _ => bibtext.push_str(&format!("  {}={{{}}},\n", &new_key, &ne_disc))
        }
    }
    bibtext.push_str("}\n\n");
    bibtext
}
