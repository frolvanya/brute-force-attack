use clap::Parser;
use hyper::Uri;

mod password_generator;

/// Brute Force Attack
#[derive(Parser)]
struct Args {
    /// Website url
    #[clap(short, long)]
    url: Uri,

    /// $username_field:$password_field
    #[clap(short, long)]
    fields: String,

    /// Generate all possible passwords (not efficent)
    #[clap(takes_value = false, short, long)]
    generate: bool,

    /// Make available alpha symbols
    #[clap(short, long)]
    alpha: bool,

    /// Make available numeric symbols
    #[clap(short, long)]
    numeric: bool,

    /// Make available alpha symbols
    #[clap(short, long = "special")]
    special_symbols: bool,
}

fn main() {}
