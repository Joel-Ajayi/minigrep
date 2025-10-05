//! minigrep binary
//!
//! Lightweight example binary that uses the `minigrep_pack` library crate.
//!
//! Quick publishing checklist:
//! - Run `cargo test` and ensure all tests pass.
//! - Update `Cargo.toml` with the correct authors, description, license, and repository fields.
//! - Ensure `README.md` exists and is referenced via `package.readme` in Cargo.toml.
//! - Bump version in Cargo.toml if releasing a new version.
//! - Run `cargo publish --dry-run` to validate before actual publish.
//!
//! To run tests locally:
//! ```text
//! cargo test
//! ```
//!
//! The binary relies on the library crate `minigrep_pack` for argument parsing and core behavior.

#[warn(unused_imports)]
#[allow(unused)]
use std::env;
use std::process;

use minigrep_pack;

/// Entry point for the minigrep_pack binary.
///
/// This function parses CLI arguments using `minigrep_pack::Config::new` and runs the library's
/// `run` function. Errors are printed to stderr and cause a non-zero exit code.
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = minigrep_pack::Config::new(&args).unwrap_or_else(|err: &str| {
        eprintln!("Problems parsing args: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep_pack::run(config) {
        eprintln!("App Error: {}", e);
        process::exit(1);
    };
}

// Minimal tests to ensure that `Config::new` can parse a basic args vector.
// This provides a quick smoke-test useful for CI and publishing validation.
#[cfg(test)]
mod tests {
    // Use the library crate that the binary depends on.
    use super::*;

    #[test]
    fn config_new_parses_basic_args() {
        // A minimal argument vector: program name, query, filename
        let args = vec![
            "minigrep".to_string(),
            "needle".to_string(),
            "haystack.txt".to_string(),
        ];

        // We only assert that parsing succeeds; concrete field checks belong in the library's tests.
        assert!(
            minigrep_pack::Config::new(&args).is_ok(),
            "Config::new should succeed on valid args"
        );
    }
}
