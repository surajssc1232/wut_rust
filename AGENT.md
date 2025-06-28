# Agent Instructions for wut_rust

## Build/Test Commands
- Build: `cargo build`
- Run: `cargo run`
- Test: `cargo test`
- Check: `cargo check`
- Lint: `cargo clippy`
- Format: `cargo fmt`

## Architecture
This is a shell prompt utility written in Rust. Main components:
- `main.rs`: Entry point, contains `get_prompt()` function for extracting shell prompts
- `prompt.rs`: Prompt cleaning utilities using regex to remove shell escape sequences
- `shell.rs`: Shell detection and tmux integration functions
- Dependencies: `regex` crate for pattern matching

## Code Style
- Use `#![allow(dead_code)]` for modules with unused functions
- Snake_case for function and variable names
- String handling: prefer `String::from_utf8_lossy()` and `.trim().to_string()`
- Error handling: use `expect()` with descriptive messages, `panic!()` for unsupported cases
- Module organization: separate concerns (shell detection, prompt handling, main logic)
- Use `match` expressions for shell type detection
- Imports: group std imports, then external crates, then local modules
