use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(trailing_var_arg = true, allow_hyphen_values = true, required = true)]
    text: Vec<String>,

    #[arg(short = 'n', long = None, help = "Do not print newline")]
    no_newline: bool
}

fn main() {
    let args = Args::parse();

    print!("{}{}", args.text.join(" "), if args.no_newline { "" } else { "\n" });
}
