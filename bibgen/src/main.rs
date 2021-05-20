mod class;
use class::*;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let dir_name: &str = &args[1];

    let ne_dir = "./";
    let dir_path = &format!("{}{}", ne_dir, dir_name);

    let mut bib_list: String = String::from("");

    let paths = fs::read_dir(dir_path).unwrap();
    for path in paths {
        let file_path = path.unwrap().path();
        let file_extn = file_path.extension();
        if let Some(extn) = file_extn {
            match extn.to_str().unwrap() {
                "net" => {
                    let bib_ne = BibNE::new(&file_path);
                    bib_list.push_str(&bib_ne.gen());
                }
                "bib" => {
                    bib_list.push_str(&bl2st(&file_path));
                }
                _ => {}
            }
        }
    }

    // Is this correct?
    fs::write(format!("{}ref.bib", ne_dir), bib_list).unwrap();
}
