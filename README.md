# Huh 

![Demo Gif](assets/demo.gif)

## Overview

Huh is a powerful command-line interface (CLI) tool built with Rust, designed to enhance your shell experience. Leveraging the Gemini API, it provides intelligent analysis and suggestions for your previously executed commands, making your terminal more efficient and intuitive.

## Features

-   **AI-Powered Command Analysis:** Integrates with the Gemini API to provide intelligent analysis and insights into your most recently executed command.
-   **Contextual Suggestions:** Offers relevant information and next steps based on the command's output.
-   **File Creation & Editing:** Create new files or edit existing ones using AI assistance with natural language instructions.
-   **GitHub-Style Diffs:** See exactly what changes are made with colorized diff output before files are modified.
-   **History Integration:** Reads your shell history to provide context for the AI analysis.
-   **Fast and Reliable:** Built with Rust, ensuring high performance, memory safety, and reliability.
-   **Cross-Platform:** Designed to work seamlessly across various operating systems.

## Prerequisites

Before you begin, ensure you have the following installed on your system:

-   **Rust and Cargo:** Huh is built with Rust. If you don't have Rust and Cargo (Rust's package manager and build system) installed, you can get them by running the following command in your terminal:

    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

    Follow the on-screen instructions. After installation, you might need to restart your terminal or run `source $HOME/.cargo/env` to update your PATH.

-   **Gemini API Key:** To utilize the AI-powered features, you will need an API key from the Google Gemini API. Please refer to the official Google AI documentation on how to obtain one.

    Once you have your API key, you will need to configure it as an environment variable. For example:

    ```bash
    export GEMINI_API_KEY="YOUR_API_KEY_HERE"
    ```

    It's recommended to add this line to your shell's configuration file (e.g., `.bashrc`, `.zshrc`, `config.fish`) to make it persistent.

-   **Tmux:** Huh is designed to be used within a `tmux` session. It is not possible to read the output of `huh` without `tmux` as it relies on `tmux` panes for displaying information. Please ensure `tmux` is installed and you are running `huh` within a `tmux` session. You can install `tmux` using your system's package manager (e.g., `sudo apt-get install tmux` on Debian/Ubuntu, `sudo yum install tmux` on CentOS/RHEL, `brew install tmux` on macOS).

## Installation

To get a copy of Huh up and running on your local machine, follow these steps:

1.  **Clone the repository:**

    ```bash
    git clone https://github.com/surajssc1232/wut_rust.git
    cd wut_rust
    ```

    *(Replace `https://github.com/surajssc1232/wut_rust.git` with the actual URL of your repository.)*

2.  **Build the project:**

    ```bash
    cargo build --release
    ```

    This command compiles the project in release mode, optimizing it for performance. The executable will be located in `target/release/huh`.

3.  **Install the executable:**

    ```bash
    cargo install --path .
    ```

    This command installs the `huh` executable to your Cargo bin directory (usually `~/.cargo/bin`), making it available in your system's PATH.

Alternatively, you can install directly from crates.io:

```bash
cargo install huh
```

## Usage

After successful installation, you can use Huh in several ways:

### Command Analysis
To get an analysis of your last executed command, simply type `huh` in your terminal:

```bash
$ <your_command_here>
$ huh
```

### Query Mode
You can also ask Huh questions directly:

```bash
$ huh "How do I list all files in a directory?"
```

### File Reading and Query
Read a file and ask questions about it:

```bash
$ huh @myfile.txt "What does this file do?"
```

### File Writing and Editing
Use the new write mode to create or edit files with AI assistance:

```bash
# Create a new file
$ huh -w @newfile.py "Create a Python script that calculates fibonacci numbers"

# Edit an existing file
$ huh -w @existing.js "Add error handling to this JavaScript code"
```

When editing existing files, Huh will show you a **concise diff summary** with **colored output** showing exactly what changes were made:

```diff
▲ Changes for config.json:
─────────────────────────────────────────────────────────────
  3 additions (+), 1 deletions (-)

Key changes:
  - "port": 5432          ← Red (deletions)
  + "port": 5432,         ← Blue (additions)
  + "debug": true         ← Blue (additions)
─────────────────────────────────────────────────────────────
```

**For files with many changes:**
```diff
▲ Changes for large_file.js:
─────────────────────────────────────────────────────────────
  42 additions (+), 15 deletions (-)

Key changes:
  + function validateInput(data) {      ← Blue (additions)
  + function processData(input) {       ← Blue (additions)
  - var oldFunction = function() {      ← Red (deletions)
  + const newFunction = () => {         ← Blue (additions)
  - console.log('old way');             ← Red (deletions)
  ... and 52 more changes
─────────────────────────────────────────────────────────────
```

Huh will provide intelligent analysis, suggestions, and file modifications based on your requests.

## Contributing

Contributions are what make the open-source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".

1.  Fork the Project
2.  Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3.  Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4.  Push to the Branch (`git push origin feature/AmazingFeature`)
5.  Open a Pull Request

## License

Distributed under the MIT License. See `LICENSE` for more information.

## Contact


Project Link: [https://github.com/surajssc1232/wut_rust](https://github.com/surajssc1232/wut_rust)
