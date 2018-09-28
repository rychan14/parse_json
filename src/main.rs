#[macro_use]
extern crate serde_json;

use std::collections::HashMap;
use std::fs::{self, read_dir, File};
use std::io::BufReader;
use std::io::prelude::*;

fn remove_file (path: &str) {
    if fs::metadata(path).is_ok() {
        fs::remove_file(path).expect("failed to delete file")
    }
}

fn write_file (mut file: File, content: &[u8]) -> std::io::Result<()> {
    file.write_all(content)
}

fn main() {
    let input_path = "./";
    let output_path = "./";
    let filter_path = ".properties";
    read_dir(input_path).unwrap()
        .map(|file| file.unwrap().path())
        .filter(|file| file.to_str().unwrap().contains(filter_path))
        .for_each(|path_buf| {
            let path = path_buf.as_path().to_str().unwrap();
            let file = BufReader::new(File::open(path).unwrap());
            let mut translation_map = HashMap::new();
            file.lines()
                .filter(|line| !line.as_ref().unwrap().is_empty())
                .for_each(|line| {
                    let line = line.unwrap();
                    let l: Vec<&str> = line.split("=").collect();
                    let key = l[0].to_owned();
                    let value = l[1..]
                        .join("=")
                        .replace("{", "${");
                    translation_map.insert(key, value);
                });

            // remove file if it already exists
            let locale = path
                .replace(".properties", "")
                .replace("common_", "")
                .replace("common", "en")
                .replace("./", "");
            let new_path = format!("{}.js", locale);
            let output_path_with_locale = format!("{}{}", output_path, new_path);
            remove_file(&output_path_with_locale);

            // write hashmap to new file
            let new_file = File::create(&output_path_with_locale).unwrap();
            let json = json!(translation_map).to_string();
            let module = format!("export const {} = {}", locale, json);
            write_file(new_file, &module.as_bytes()).expect("failed to write");
        });
}
