use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(
        trailing_var_arg = true,
        allow_hyphen_values = true,
        help = "Input file(s)",
        value_name = "FILE",
        default_value = "-"
    )]
    files: Vec<String>,

    #[arg(
        short = 'n',
        long = "lines",
        help = "Number of lines",
        default_value = "10"
    )]
    lines: String,

    #[arg(
        short = 'c',
        long = "bytes",
        help = "Number of bytes",
        conflicts_with = "lines"
    )]
    bytes: Option<String>,
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let args = Args::parse();
    let lines =
        parse_positive_int(&args.lines).map_err(|e| format!("illegal line count -- {}", e))?;

    let bytes = match args.bytes {
        Some(bytes) => {
            Some(parse_positive_int(&bytes).map_err(|e| format!("illegal byte count -- {}", e))?)
        }
        None => None,
    };

    Ok(Config {
        files: args.files,
        lines,
        bytes,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let num_files = config.files.len();

    for (file_num, filename) in config.files.iter().enumerate() {
        match open(&filename) {
            Err(e) => eprintln!("{}: {}", filename, e),
            Ok(mut file) => {
                if num_files > 1 {
                    println!(
                        "{}==> {} <==",
                        if file_num > 0 { "\n" } else { "" },
                        filename
                    );
                }

                if let Some(num_bytes) = config.bytes {
                    let mut handle = file.take(num_bytes as u64);
                    let mut buffer = vec![0; num_bytes];
                    let bytes_read = handle.read(&mut buffer)?;
                    print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
                } else {
                    let mut line = String::new();
                    for _ in 0..config.lines {
                        let bytes = file.read_line(&mut line)?;

                        if bytes == 0 {
                            break;
                        }

                        print!("{}", line);
                        line.clear();
                    }
                }
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

/// Examples:
///
/// ```rust
/// let res = headr::parse_positive_int("3");
/// assert!(res.is_ok());
/// ```
///
pub fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse::<usize>() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(val.into()),
    }
}
