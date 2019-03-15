extern crate clap;

use clap::App;
use std::io;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let yaml = clap::load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let filename_pattern = matches.value_of("FILE_PATTERN").unwrap();

    if !filename_pattern.contains("{}") {
        eprintln!("FILE_PATTERN must contain placeholder text: {{}}");
        std::process::exit(1);
    }

    let file_size = matches.value_of("MAX_SIZE").unwrap_or("2M");
    let file_size_number_part = file_size[0..file_size.len()-1].parse::<usize>().expect("could not parse number");
    let file_size: usize = match file_size.chars().last().unwrap() {
        'K' => {
            file_size_number_part * 1000
        }
        'M' => {
            file_size_number_part * 1000 * 1000
        }
        'G' => {
            file_size_number_part * 1000 * 1000 * 1000
        }
        s => {
            eprintln!("Unsupported number suffix \"{}\"", s);
            std::process::exit(1);
        }
    };

    let mut first_file = File::create(filename_pattern.replace("{}", "1"))?;
    let mut second_file = File::create(filename_pattern.replace("{}", "2"))?;
    let mut current_file_size = 0usize;
    let mut current_file: &mut File = &mut first_file;
    let mut using_first_file = true;

    let stdin = io::stdin();
    let mut stdin_handle = stdin.lock();

    loop {
        let mut input = String::new();
        let bytes_read = stdin_handle.read_line(&mut input)?;
        if bytes_read == 0 {
            break;
        }

        current_file_size += bytes_read;
        current_file.write_all(input.as_bytes())?;

        if current_file_size > file_size {
            if using_first_file {
                using_first_file = false;
                current_file = &mut second_file;
            } else {
                using_first_file = true;
                current_file = &mut first_file;
            }
        }
    }

    Ok(())
}
