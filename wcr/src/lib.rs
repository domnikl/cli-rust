use clap::{Arg, ArgAction, Command, Parser};
use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

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
        short = 'c',
        long = "bytes",
        help = "Show byte count",
        default_value = "false"
    )]
    bytes: bool,

    #[arg(
        short = 'm',
        long = "chars",
        help = "Show character count",
        default_value = "false",
        conflicts_with = "bytes"
    )]
    chars: bool,

    #[arg(
        short = 'l',
        long = "lines",
        help = "Show line count",
        default_value = "false"
    )]
    lines: bool,

    #[arg(
        short = 'w',
        long = "words",
        help = "Show word count",
        default_value = "false"
    )]
    words: bool,
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

pub fn get_args() -> MyResult<Config> {
    let args = Args::parse();
    let mut words = args.words;
    let mut lines = args.lines;
    let mut bytes = args.bytes;
    let chars = args.chars;

    if [lines, words, bytes, chars].iter().all(|v| v == &false) {
        lines = true;
        words = true;
        bytes = true;
    }

    Ok(Config {
        files: args.files,
        lines,
        words,
        bytes,
        chars,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in &config.files {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                if let Ok(info) = count(file) {
                    println!(
                        "{}{}{}{} {}",
                        format_field(info.num_lines, config.lines),
                        format_field(info.num_words, config.words),
                        format_field(info.num_bytes, config.bytes),
                        format_field(info.num_chars, config.chars),
                        filename
                    );
                }
            }
        }
    }

    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(std::io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;
    let mut buf = String::new();

    loop {
        let line_bytes = file.read_line(&mut buf)?;
        if line_bytes == 0 {
            break;
        }

        num_bytes += line_bytes;
        num_lines += 1;
        num_words += buf.split_whitespace().count();
        num_chars += buf.chars().count();
        buf.clear();
    }

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

/// Examples:
///
/// ```rust
/// let res = wcr::format_field(10, true);
/// assert_eq!("      10", res);
///
/// let res = wcr::format_field(10, false);
/// assert_eq!("", res);
/// ```
///
pub fn format_field(value: usize, show: bool) -> String {
    if show {
        format!("{:>8}", value)
    } else {
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::{count, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };

        assert_eq!(info.unwrap(), expected);
    }
}
