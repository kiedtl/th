// TODO: syntax documentation

use std::vec::Vec;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum InfoField {
    Values(Vec<String>),
    Flag(String),
    Section(HashMap<String, InfoField>),
}

impl InfoField {
    pub fn values(&self) -> Vec<String> {
        if let InfoField::Values(v) = self {
            return v.clone();
        } else {
            // TODO: return result
            return vec![];
        }
    }

    pub fn section(&self) -> HashMap<String, InfoField> {
        if let InfoField::Section(h) = self {
            return h.clone();
        } else {
            // TODO: return result
            return HashMap::new();
        }
    }
}

#[derive(Clone, Debug)]
pub struct InfoFileData {
    // d is for data
    pub d: HashMap<String, InfoField>
}

impl InfoFileData {
    pub fn has_key(&self, key: &str) -> bool {
        if !self.d.contains_key(key) {
            return false;
        } else {
            return true;
        }
    }

    pub fn from_lines(lines: Vec<String>) -> InfoFileData {
        let mut parsed = InfoFileData {
            d: HashMap::new(),
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
                        accm.d.insert(section_name, InfoField::Section(section));
                        return l
                    },
                    _ => {
                        if split.len() > 1 {
                            // filter out empty values
                            let values = split[1..].iter()
                                .map(|v| v.to_string())
                                .filter(|v| v.len() > 0)
                                .collect::<Vec<String>>();

                            section.insert(split[0].to_string(),
                                InfoField::Values(values));
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
