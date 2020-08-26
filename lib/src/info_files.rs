// some utility functions to load info files
use crate::id::*;
use ron::de::from_reader;
use std::collections::HashMap;
use std::error::Error;
use std::{fs, fs::File};
use walkdir::WalkDir;

pub fn load_info_files<T>(path: &str)-> Result<HashMap<String, T>, Box<dyn Error>>
where
    T: for<'a> serde::Deserialize<'a> + Id + Clone
{
    let mut accm: HashMap<String, T> = HashMap::new();
    for item in WalkDir::new(path) {
        let entry = item?.clone();
        let path = entry.path();
        if fs::metadata(path)?.is_dir() {
            continue;
        }
        let info_file = File::open(path)?;
        let data: Result<T, ron::error::Error> = from_reader(info_file);
        match data {
            Ok(x)  => {
                let idxr = x.id();
                *accm.entry(idxr).or_insert(x) = x.clone();
            },
            Err(e) => {
                // just print a message and continue
                eprintln!("warn: unable to load info file: {}: {}",
                    path.display(), e);
            },
        }
    }
    Ok(accm)
}
