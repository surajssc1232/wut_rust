## Commands

- Build: `cargo build`
- Run: `cargo run`
- Test: `cargo test`
- Lint: `cargo clippy`
- Format: `cargo fmt`

## Code Style

- Formatting: Use `rustfmt` for consistent code style.
- Imports: Group standard library, crate, and module imports separately.
- Types: Use static typing and leverage Rust's type system.
- Naming Conventions: Follow Rust's naming conventions (e.g., `snake_case` for variables and functions, `PascalCase` for types).
- Error Handling: Use `Result` and `Option` for error handling. Avoid using `unwrap()` in production code.
- Dependencies: Manage dependencies in `Cargo.toml`.
- Use `serde` for serialization and deserialization.
- Use `clap` for command-line argument parsing.
- Use `tokio` for asynchronous operations.
