use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("must input a file_name");
        process::exit(1);
    }

    let file_name = args.last().unwrap();

    let extension_pos_opt = file_name.rfind('.');

    if extension_pos_opt.is_none() {
        eprintln!("unknown file extension");
        process::exit(1);
    }

    let pos = extension_pos_opt.unwrap();

    let extension = &file_name[pos..]; // get back a slice of the rest of the iterator

    match extension.into() {
        ".ass" => {
            handle_ass(file_name);
        }
        ".srt" => {
            handle_srt(file_name);
        }
        _ => {
            println!("unsupport file extension: {}",extension)
        }
    }
}

fn handle_srt(file_name: &String) {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    // remove 3th line for each section
    let mut count = 0;
    for line_rs in reader.lines() {
        let line = line_rs.unwrap();
        if count != 2 {
            println!("{}", line);
        }

        count = count + 1;

        if line.is_empty() {
            count = 0;
        }
    }
}

fn handle_ass(file_name: &String) {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    let mut event_section = false;
    for line_rs in reader.lines() {
        let line = line_rs.unwrap();

        if event_section && line.starts_with("Dialogue") {
            let remove_mark = r"\N";
            // everything after the 9th comma is treated as the subtitle text
            let text_pos = find_ninth_comma(&line).unwrap_or(0);
            let remove_pos = line.find(remove_mark).unwrap_or(0);
            if text_pos < remove_pos {
                let line_profix = &line[..text_pos + 1];
                let line_end = &line[remove_pos + remove_mark.len()..];
                print!("{}", line_profix);
                println!("{}", line_end);
            } else {
                println!("{}", line)
            }
        } else {
            println!("{}", line)
        }

        if "[Events]" == line {
            event_section = true;
        }
    }
}

fn find_ninth_comma(s: &str) -> Option<usize> {
    let mut count = 0;
    for (i, c) in s.char_indices() {
        if c == ',' {
            count += 1;
            if count == 9 {
                return Some(i);
            }
        }
    }
    None
}
