use std::io::{self, BufReader, prelude::*};
use std::fs::File;
use std::env;

fn read (input: &mut String) {
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(input)
        .expect("Failed to read from stdin");
}

fn main() {
    let command_line_args: Vec<String> = env::args().collect();
    if command_line_args.len() != 2 {
        panic!("Need to specify the name of the file.")
    }

    let file_name = &command_line_args[1];
    let file = File::options().read(true).write(true).open(file_name).expect("Cannot open the file.");
    let reader = BufReader::new(file);

    print!("Search for a number: ");
    let mut number = String::new();
    read(&mut number);
    let number:u32 = number.trim().parse().expect("Not a valid number.");

    let mut counter:u8 = 0;

    for line in reader.lines() {
        let line = line.unwrap().parse::<u32>().unwrap();
        if number == line {
            counter+=1;
        }
    }

    if counter > 0 {
        println!("The number {} was found {} time(s).", number, counter);
    } else {
        println!("The number {} wasn't found.", number);
    }
}