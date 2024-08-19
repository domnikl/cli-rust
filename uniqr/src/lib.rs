use anyhow::{anyhow, Result};
use clap::Parser;
use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Write},
};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(
        allow_hyphen_values = true,
        help = "Input file",
        value_name = "IN_FILE",
        default_value = "-"
    )]
    in_file: String,

    #[arg(
        allow_hyphen_values = false,
        help = "Output file",
        value_name = "OUT_FILE"
    )]
    out_file: Option<String>,

    #[arg(
        short = 'c',
        long = "count",
        help = "Show counts",
        default_value = "false"
    )]
    count: bool,
}

#[derive(Debug)]
pub struct Config {
    in_file: String,
    out_file: Option<String>,
    count: bool,
}

pub fn get_args() -> MyResult<Config> {
    let args = Args::parse();

    Ok(Config {
        in_file: args.in_file,
        out_file: args.out_file,
        count: args.count,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let mut file = open(&config.in_file).map_err(|e| anyhow!("{}: {e}", config.in_file))?;

    let mut out_file: Box<dyn Write> = match &config.out_file {
        Some(out_name) => Box::new(File::create(out_name)?),
        _ => Box::new(std::io::stdout()),
    };

    let mut print = |num: u64, text: &str| -> MyResult<()> {
        if num > 0 {
            if config.count {
                write!(out_file, "{num:>4} {text}")?;
            } else {
                write!(out_file, "{text}")?;
            }
        };
        Ok(())
    };

    let mut line = String::new();
    let mut previous = String::new();
    let mut count: u64 = 0;
    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }

        if line.trim_end() != previous.trim_end() {
            print(count, &previous)?;
            previous = line.clone();
            count = 0;
        }

        count += 1;
        line.clear();
    }

    print(count, &previous)?;

    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(std::io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
