# OpenCode Configuration for Huh (Rust CLI Tool)

## Build/Test/Lint Commands
- **Build**: `cargo build` or `cargo build --release`
- **Run**: `cargo run` or `cargo run -- [args]`
- **Test**: `cargo test` 
- **Test single**: `cargo test test_name`
- **Check**: `cargo check` (fast compilation check)
- **Format**: `cargo fmt`
- **Lint**: `cargo clippy`
- **Install locally**: `cargo install --path .`

## Usage Examples
- **Command analysis**: `huh` (analyzes last command)
- **Query mode**: `huh "question here"`
- **File query**: `huh @file.txt "what does this do?"`
- **File write/edit**: `huh -w @file.rs "add error handling"`

## Code Style Guidelines
- **Imports**: Group std, external crates, then local modules with blank lines between
- **Naming**: snake_case for functions/variables, PascalCase for types/structs, SCREAMING_SNAKE for constants
- **Error handling**: Use `Result<T, String>` for errors, `.map_err()` for error transformation
- **Async**: Use tokio runtime, async/await syntax, prefer structured concurrency with `select!`
- **String handling**: Use `String::from()` for literals, `to_string()` for conversions
- **Structs**: Derive common traits (Serialize, Deserialize, etc.), use builder pattern for complex initialization
- **CLI**: Use clap with derive macros, provide helpful descriptions and defaults
- **HTTP**: Use reqwest with proper error handling and JSON serialization
- **Regex**: Compile once, reuse instances, use raw strings for patterns
- **Terminal output**: Use ANSI escape codes directly, flush stdout after printing