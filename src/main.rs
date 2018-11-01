extern crate rand;

use rand::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<std::error::Error>> {
    let mut rng = thread_rng();

    let file_reader = BufReader::new(File::open("words.txt")?);
    let lines: HashMap<u32, String> = file_reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut parts = line.split_whitespace();
            let result: (u32, String) = (
                parts
                    .next()
                    .expect("Bad source file. Columns must be (u32, String).")
                    .parse::<u32>()
                    .unwrap(),
                parts
                    .next()
                    .expect("Bad source file. Columns must be (u32, String).")
                    .to_string(),
            );
            result
        })
        .collect();

    let mut password = String::new();
    let mut added = 0;
    while added < 7 {
        let num = (rng.next_u32() % 55555) + 11111;
        password.push_str(match lines.get(&num) {
            Some(item) => item,
            None => continue,
        });

        added += 1;
    }

    println!("{}", password);

    Ok(())
}
