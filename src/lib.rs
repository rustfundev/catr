use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Files to print
    #[arg(default_values_t = vec![String::from("-")])]
    files: Vec<String>,

    /// Number the non-blank output lines, starting at 1
    #[arg(
        group = "input",
        short = 'b',
        long = "number-nonblank",
        default_value_t = false
    )]
    number_nonblank_lines: bool,

    /// Number the output lines, starting at 1
    #[arg(group = "input", short = 'n', long = "number", default_value_t = false)]
    number_lines: bool,
}

pub fn run(args: Args) -> MyResult<()> {
    for filename in args.files.iter() {
        match open_file(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(fh) => read_file(fh, args.number_lines, args.number_nonblank_lines)?,
        }
    }
    Ok(())
}

pub fn get_args() -> MyResult<Args> {
    Ok(Args::parse())
}

fn open_file(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn read_file(
    fh: Box<dyn BufRead>,
    number_lines: bool,
    number_nonblank_lines: bool,
) -> MyResult<()> {
    let mut count = 1u8;

    let handle_blank_line = |line: &String, count: &mut u8| {
        if number_lines {
            println!("     {}\t{}", count, line);
            *count = *count + 1;
        } else if number_nonblank_lines {
            println!("{}", line);
        }
    };

    let handle_nonblank_line = |line: &String, count: &mut u8| {
        if number_lines || number_nonblank_lines {
            println!("     {}\t{}", count, line);
            *count = *count + 1;
        }
    };

    fh.lines().for_each(|x| match x {
        Err(err) => eprintln!("Failed to read file line: {}", err),
        Ok(line) => {
            match line.trim().is_empty() {
                true => handle_blank_line(&line, &mut count),
                false => handle_nonblank_line(&line, &mut count),
            };

            if !number_lines && !number_nonblank_lines {
                println!("{}", line);
            }
        }
    });
    Ok(())
}
