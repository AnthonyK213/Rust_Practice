use std::fs;
use std::path::Path;
use std::io::{BufRead, BufReader};

pub struct BibNE {
    content: Vec<String>
}

impl BibNE {
    pub fn new(path: &Path) -> Self {
        let file = fs::File::open(path).unwrap();
        let reader = BufReader::new(file);
        let mut content: Vec<String> = Vec::new();
        for line in reader.lines() {
            let line = line.unwrap();
            if line != "" {
                content.push(line);
            }
        }
        Self { content }
    }

    fn title(&self) -> String {
        let mut tt_str = "--".to_string();

        for line in &self.content {
            let ne_line: Vec<&str> = line.split(": ").collect();

            if line != "" && ne_line.len() > 1 {
                let key: &str = &ne_line[0];
                let val: &str = &ne_line[1]
                    .chars()
                    .map(|x| match x {
                        ' ' | '(' | ')' | ',' | '{' | '}' => '_',
                        _ => x
                    }).collect::<String>();
                match key {
                    "{Title}" => {
                        tt_str = val.to_string();
                        break;
                    },
                    _ => {}
                };
            }
        }
        tt_str
    }

    pub fn gen(&self) -> String {
        let mut bib_txt: String = "".to_string();

        for line in &self.content {
            let ne_line: Vec<&str> = line.split("}: ").collect();

            if line != "" && ne_line.len() > 1 {
                let new_key: &str = &ne_line[0][1..]
                    .to_lowercase()
                    .replace(" ", "")
                    .replace(",", "");
                let new_val: &str = ne_line[1];
                let ttl = self.title();
                match new_key {
                    "referencetype" => {
                        match new_val {
                            "Journal Article"
                                => bib_txt.push_str(
                                    &format!("@article{{{},\n", &ttl)),
                            "Conference Proceedings"
                                => bib_txt.push_str(
                                    &format!("@inproceedings{{{},\n", &ttl)),
                            "Thesis"
                                => bib_txt.push_str(
                                    &format!("@mastersthesis{{{},\n", &ttl)),
                            _ => {
                                bib_txt.push_str(
                                    &format!("//@{}{{{},\n", new_val, &ttl));
                                println!(
                                    "Unknown type of article: \"{}\" => {}",
                                    new_val, &ttl);
                            }
                        }
                    },
                    "issue"
                        => bib_txt.push_str(
                            &format!("  number={{{}}},\n", &new_val)), 
                    "abstract" => {},
                    _ => bib_txt.push_str(
                        &format!("  {}={{{}}},\n", &new_key, &new_val))
                }
            }
        }
        bib_txt + ("}\n\n")
    }
}

pub fn bl2st(bl_path: &Path) -> String {
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
