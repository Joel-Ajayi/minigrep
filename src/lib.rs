//! minigrep library
//!
//! Small example library used by the `minigrep` binary. Provides argument parsing and
//! simple line-oriented search (case-sensitive and case-insensitive).
//!
//! Quick publishing checklist:
//! - Ensure `cargo test` passes.
//! - Update Cargo.toml: authors, description, license, repository, readme, and version.
//! - Ensure README.md exists and is referenced in Cargo.toml.
//! - Run `cargo publish --dry-run` to validate before publishing.
//!
//! Usage (library):
//! - Call `Config::new(&args)` to parse CLI args.
//! - Call `run(config)` to execute a search and print matching lines.
//!
#[warn(dead_code)]
use std::env;
use std::error::Error;
use std::fs;

/// Run the search using the provided configuration.
///
/// Reads the file specified in `config.filename`, performs the appropriate search
/// (case-sensitive or case-insensitive) and prints matching lines to stdout.
/// Errors from reading the file are propagated to the caller.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }
    Ok(())
}

/// Configuration for a minigrep run.
///
/// Contains the search query, the filename to search, and whether the search is
/// case-sensitive.
pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    /// Create a new `Config` from CLI-style arguments.
    ///
    /// Expects at least three arguments: program name, query, filename.
    /// The `CASE_INSENSITIVE` environment variable toggles case sensitivity:
    /// if it is present the search will be case-insensitive.
    ///
    /// Returns `Err` with a short message when args are not sufficient.
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough params");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

/// Perform a case-sensitive search for `query` in `contents`.
///
/// Returns a vector of line slices that contain the query.
fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .map(|line| line.trim())
        .collect()
}

/// Perform a case-insensitive search for `query` in `contents`.
///
/// Returns a vector of line slices that match the query case-insensitively.
fn search_case_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .map(|line| line.trim())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
            Rust:
            safe, fast, production
            Forget Guy";

        assert_eq!(vec!["safe, fast, production"], search(query, contents))
    }

    #[test]
    fn case_insensitive_search_finds_matches() {
        let query = "rUsT";
        let contents = "\
            Rust:
            safe, fast, production
            Forget Guy";
        // search_case_insensitive should find the "Rust:" line
        assert_eq!(vec!["Rust:"], search_case_insensitive(query, contents));
    }

    #[test]
    fn config_new_parses_basic_args() {
        let args = vec![
            "minigrep".to_string(),
            "needle".to_string(),
            "haystack.txt".to_string(),
        ];

        assert!(
            Config::new(&args).is_ok(),
            "Config::new should succeed on valid args"
        );
    }
}
