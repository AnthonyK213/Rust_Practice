use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let dir_name: &str = &args[1];

    let ne_dir = "./";
    let dir_path = &format!("{}{}", ne_dir, dir_name);

    let mut bib_list: String = String::from("");
    let mut num: u16 = 1;

    let paths = fs::read_dir(dir_path).unwrap();
    for path in paths {
        let file_path = path.unwrap().path();
        let file_extn = file_path.extension();
        if let Some(extn) = file_extn {
            match extn.to_str().unwrap() {
                "net" => {
                    bib_list.push_str(&ne2bl(&file_path, num));
                    num += 1;
                }
                "bib" => {
                    bib_list.push_str(&bl2st(&file_path));
                    num += 1;
                }
                _ => {}
            }
        }
    }

    // Is this correct?
    fs::write(format!("{}ref.bib", ne_dir), bib_list).unwrap();
}

// Hashmap may be better. Hitherto, pattern matching is fine.
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
    let mut title_str = "";

    for line in &content {
        let ne_line: Vec<&str> = line.split("}: ").collect();

        if line != "" && ne_line.len() > 1 {
            let new_key: &str = &ne_line[0][1..].to_lowercase().as_str().replace(" ", "");
            let ne_disc: &str = ne_line[1];
            match new_key {
                "title" => {
                    title_str = ne_disc;
                    break;
                }
                _ => {}
            }
        }
    }

    for line in &content {
        let ne_line: Vec<&str> = line.split("}: ").collect();

        if line != "" && ne_line.len() > 1 {
            let new_key: &str = &ne_line[0][1..].to_lowercase().as_str().replace(" ", "");
            let ne_disc: &str = ne_line[1];
            match new_key {
                "referencetype" => match ne_disc {
                    "Journal Article" => bibtext.push_str(&format!("@article{{{},\n", title_str)),
                    "Conference Proceedings" => {
                        bibtext.push_str(&format!("@inproceedings{{{},\n", title_str))
                    }
                    "Thesis" => bibtext.push_str(&format!("@mastersthesis{{{},\n", title_str)),
                    _ => {
                        bibtext.push_str(&format!("//@{}{{{},\n", ne_disc, &num));
                        println!("Unknown type of article: \"{}\" => {}", ne_disc, &num);
                    }
                },
                "issue" => bibtext.push_str(&format!("  number={{{}}},\n", &ne_disc)),
                "abstract" => {}
                _ => bibtext.push_str(&format!("  {}={{{}}},\n", &new_key, &ne_disc)),
            }
        }
    }
    bibtext + ("}\n\n")
}

fn bl2st(bl_path: &Path) -> String {
    let file = fs::File::open(bl_path).unwrap();
    let reader = BufReader::new(file);
    let mut content: String = String::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line != "" {
            content += &line;
            content += "\n";
        }
    }
    content + "\n"
}
