// TODO: syntax documentation

use std::vec::Vec;
use std::collections::HashMap;

#[derive(Clone, Debug)]
enum InfoField {
    Values(Vec<String>),
    Flag(String),
    Section(String, HashMap<String, InfoField>),
}

#[derive(Clone, Debug)]
pub struct InfoFileData {
    data: Vec<InfoField>,
}

impl InfoFileData {
    pub fn from_lines(lines: Vec<String>) -> InfoFileData {
        let mut parsed = InfoFileData {
            data: Vec::new(),
        };

        fn parse_section(
            accm: &mut InfoFileData, section_name: String, lines: &[String]
        ) -> usize {
            let mut section = HashMap::new();
            for mut l in 0..lines.len() {
                let line = lines[l].trim().to_string();
                let split: Vec<&str> = line.split(" ").collect();

                match split[0] {
                    "section" => {
                        l = parse_section(accm, split[1].to_string(),
                            &lines[(l + 1)..]);
                    },
                    "end_section" => {
                        accm.data.push(InfoField::Section(section_name, section));
                        return l
                    },
                    _ => {
                        if split.len() > 1 {
                            section.insert(split[0].to_string(),
                                InfoField::Values(split[1..].iter()
                                    .map(|v| v.to_string())
                                    .filter(|v| v.len() > 0).collect::<Vec<String>>()));
                        } else {
                            section.insert(split[0].to_owned(),
                                InfoField::Flag(split[0].to_string()));
                        }
                    },
                }
            }

            0
        }

        for mut l in 0..lines.len() {
            let line = lines[l].trim().to_string();
            if line.len() == 0 {
                continue;
            }

            // comments
            if line.chars().collect::<Vec<char>>()[0] == '#' {
                continue;
            }

            let split: Vec<String> = line.split(' ')
                .map(|v| v.to_string()).collect();

            // sections
            if split[0] == "section" {
                // parse section and jump to the end
                // of section
                l = parse_section(&mut parsed, split[1].clone(),
                    &lines[(l + 1)..]);
            }
        }

        parsed
    }
}
