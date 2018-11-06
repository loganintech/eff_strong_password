extern crate rand;

use rand::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

//This should always have at least one element. The program will crash if it doesn (because of index out of bound)
const SPECIAL_CHARS: [[&'static str; 6]; 6] = [
    ["!", "@", "#", "%", "^", "&"],
    ["*", "+", "-", "=", "(", ")"],
    ["[", "]", "{", "}", "<", ">"],
    [".", ",", "'", ":", ";", "?"],
    ["~", "|", "_", "/", "\\", " "],
    ["1", "2", "3", "4", "5", "6"],
];

fn main() -> Result<(), Box<std::error::Error>> {
    let mut rng = thread_rng();

    let file_reader = BufReader::new(File::open("words.txt")?);
    let lines: HashMap<u32, String> = file_reader
        .lines()
        .map(|line| {
            let line = line.expect("Trouble loading line in file.");
            let mut parts = line.split_whitespace();

            (
                parts
                    .next()
                    .expect("Bad source file. Columns must be (u32, String)")
                    .parse::<u32>()
                    .expect("The first column of the source file must be a positive number."),
                parts
                    .next()
                    .expect("Bad source file. Columns must be (u32, String).")
                    .to_string(),
            )
        })
        .collect();

    let first_arg = std::env::args().skip(1).next();
    let place_beginning = if let Some(beginning) = &first_arg {
        beginning == "beginning"
    } else {
        false
    };
    let place_end = if let Some(end) = &first_arg {
        end == "end"
    } else {
        false
    };

    let first_roll: usize = (rng.next_u32() % SPECIAL_CHARS.len() as u32) as usize;
    let second_roll: usize = (rng.next_u32() % SPECIAL_CHARS[0].len() as u32) as usize;

    let mut password = String::new();
    let mut added = 0;
    while added < 7 {
        let num = (rng.next_u32() % 55555) + 11111;

        let next_word = match lines.get(&num) {
            Some(item) => item,
            None => continue,
        };

        if place_beginning {
            password.push_str(SPECIAL_CHARS[first_roll][second_roll]);
        }

        password.push_str(next_word);

        if place_end {
            password.push_str(SPECIAL_CHARS[first_roll][second_roll]);
        }

        added += 1;
    }

    println!("{}", password);

    Ok(())
}
