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

    let pos_opt = file_name.rfind('.');

    if pos_opt.is_none() {
        eprintln!("unknown file extension");
        process::exit(1);
    }

    let pos = pos_opt.unwrap();

    let mut iter = file_name.chars();
    iter.by_ref().nth(pos); // eat up start values
    let extension = iter.as_str(); // get back a slice of the rest of the iterator

    match extension {
        "ass" => {
            handle_ass(file_name);
        }
        "srt" => {
            handle_srt(file_name);
        }
        _ => {
            println!("err")
        }
    }
}

fn handle_srt(file_name: &String) {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

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
            let text_pos = line.rfind(',').unwrap_or(0);
            let jp_pos = line.find(r"\N").unwrap_or(0);
            if text_pos < jp_pos {
                let line_profix = &line[..text_pos + 1];
                let line_end = &line[jp_pos + 2..];
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
