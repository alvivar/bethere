use std::collections::HashMap;
use std::{env, path::Path};
use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader},
};

fn main() {
    // Read the file

    let them_file = "them.txt";
    let current_dir = env::current_dir().unwrap();
    let them_path = current_dir.join(them_file);

    let mut data: HashMap<String, Vec<String>> = HashMap::new();
    let lines = lines_from_file(them_path);

    // Parse the file

    let mut deep = 0;
    let mut last_deep = 0;
    let mut initial_deep = -1;
    let mut key: String = String::new();

    for line in lines {
        // Ignore empty lines
        if line.trim().len() <= 0 {
            continue;
        }

        // Count the whitespaces, use them as identation

        let mut c = line.chars();
        while let Some(i) = c.next() {
            match i {
                ' ' => deep += 1,
                _ => break,
            }
        }

        if initial_deep < 0 {
            initial_deep = deep;
        }

        // A new key is created

        let trim_line = line.trim().to_string();

        // On the first one
        if key == "" {
            key = trim_line.clone();
        }

        // When the deepness decrease
        if deep < last_deep {
            key = trim_line.clone();
        }

        // When it doesn't have a parent, and it's next to a brother
        if deep == initial_deep && deep == last_deep {
            key = trim_line.clone();
        }

        // Save and repeat

        let children = data.entry(key.clone()).or_insert_with(Vec::new);
        if key != trim_line {
            children.push(trim_line);
        }

        last_deep = deep;
        deep = 0;
    }

    for (key, value) in &data {
        println!("\n\t{}", key);
        for entry in value {
            println!("\t\t{}", entry);
        }
    }
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(filename);

    let buffer = BufReader::new(file.expect("I can't create files."));

    buffer
        .lines()
        .map(|l| l.expect("I can't parse the line."))
        .collect()
}
