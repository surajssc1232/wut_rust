
## Build, Lint, and Test

- Build: `cargo build`
- Lint: `cargo check`
- Test: `cargo test`
- Run a single test: `cargo test --test <test_name>`

## Code Style

- Formatting: Use `rustfmt` for automatic formatting.
- Imports: Group imports by standard library, external crates, and internal modules.
- Types: Use static types and leverage Rust's type system.
- Naming Conventions:
    - `snake_case` for variables and function names.
    - `PascalCase` for structs, enums, and traits.
- Error Handling: Prefer `Result` and the `?` operator for error propagation. Avoid using `unwrap()` or `expect()` in production code.
- Modules: Organize code into logical modules.
- Concurrency: Use `tokio` for asynchronous operations.
