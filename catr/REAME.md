# catr

A Rust version of `cat`.

## Learned

- all of a module's variables and functions are private by default
- `eprintln!` prints to stderr
- `arg_required_else_help(true)` can be set on the `#[command]` macro from clap
- anyhow has `with_context()` to provide a simple string for context of an error
- since Rust 1.0.0, `std::error::Error` implements `From<String>` and `map_error` can therefor just receive a String
