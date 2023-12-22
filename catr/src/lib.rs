use std::error::Error;
use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(trailing_var_arg = true, allow_hyphen_values = true, help = "Input file(s)", value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    #[arg(short = 'n', long = "number", help = "Number lines")]
    number_lines: bool,

    #[arg(short = 'b', long = "number-nonblank", help = "Number nonblank lines")]
    number_nonblank_lines: bool,
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(e) => eprintln!("{}: {}", filename, e),
            Ok(file) => {
                let mut last_num = 0;

                for (line_num, line_result) in file.lines().enumerate() {
                    let line = line_result?;
                    if config.number_lines {
                        println!("{:6}\t{}", line_num + 1, line);
                    } else if config.number_nonblank_lines {
                        if !line.is_empty() {
                            last_num += 1;
                            println!("{:6}\t{}", last_num, line);
                        } else {
                            println!();
                        }
                    } else {
                        println!("{}", line);
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let args = Args::parse();

    Ok(Config{
        files: args.files,
        number_lines: args.number_lines,
        number_nonblank_lines: args.number_nonblank_lines
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
