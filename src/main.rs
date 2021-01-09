use std::env;
use std::{collections::HashMap, fs::File};
use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader},
};

fn main() {
    let them_file = "them.txt";
    let current_dir = env::current_dir().unwrap();
    let them_path = current_dir.join(them_file);

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(them_path);

    let data = dict_from_tabbed_file(file.expect("Can't open file"));

    for (key, value) in data {
        println!("\n\t{}", key);
        for entry in value {
            println!("\t\t{}", entry);
        }
    }
}

fn dict_from_tabbed_file(file: File) -> HashMap<String, Vec<String>> {
    let mut deep = 0;
    let mut last_deep = 0;
    let mut initial_deep = -1;
    let mut key: String = String::new();
    let mut data: HashMap<String, Vec<String>> = HashMap::new();

    let lines = lines_from_file(file);
    for line in lines {
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

    data
}

fn lines_from_file(file: File) -> Vec<String> {
    BufReader::new(file)
        .lines()
        .map(|l| l.expect("I can't parse the line."))
        .collect()
}
