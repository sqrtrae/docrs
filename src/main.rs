//! A command line utility for opening rust crate documentation on
//! https://docs.rs using your default web browser.

use clap::Parser;
use regex::Regex;
use webbrowser;

/// A command line utility for opening rust crate documentation on
/// https://docs.rs using your default web browser.
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    /// Name of the rust crate
    crate_name: String,
}

fn is_valid_crate_name(crate_name: &str) -> bool {
    // only performs a simple regex check
    let re = Regex::new(r"^[a-zA-Z][a-zA-Z0-9\-_]*$").unwrap();
    re.is_match(crate_name)
}

fn open_docs(crate_name: &str) {
    let url = format!("https://docs.rs/{crate_name}");
    if webbrowser::open(&url).is_err() {
        println!(
            "Unable to open {crate_name} documentation on https://docs.rs"
        );
    }
}

fn main() {
    let args = Args::parse();

    if is_valid_crate_name(&args.crate_name) {
        open_docs(&args.crate_name);
    } else {
        println!("Provided crate name contains invalid characters.")
    }
}

#[cfg(test)]
mod tests {
    use super::is_valid_crate_name;

    #[test]
    fn validate_crate_names() {
        assert_eq!(is_valid_crate_name("clap"), true);
        assert_eq!(is_valid_crate_name("proc-macro2"), true);
        assert_eq!(is_valid_crate_name("serde_json"), true);

        assert_eq!(is_valid_crate_name("7minutes"), false);
        assert_eq!(is_valid_crate_name("what-the-%&#$?"), false);
    }
}
