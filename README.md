# minigrep_pack

Small example crate and binary that demonstrate a simple line-oriented search (a micro "grep" clone).

## Quick start

Build and run the binary with:

```text
cargo run -- <query> <filename>
```

Example:

```text
cargo run -- foo example.txt
```

## Library usage

Programmatic usage:

- Call `minigrep_pack::Config::new(&args)` to parse CLI-style arguments.
- Call `minigrep_pack::run(config)` to perform the search and print matching lines.

## Testing

Run the test suite with:

```text
cargo test
```

## Publishing checklist

- Ensure `cargo test` passes.
- Verify `Cargo.toml` contains correct metadata (authors, description, license, repository, readme).
- Update `version` in `Cargo.toml` for a new release.
- Run `cargo publish --dry-run` to validate before publishing.

## License

MIT
