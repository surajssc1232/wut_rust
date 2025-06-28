# Huh Rust



![Demo GIF](https://media0.giphy.com/media/v1.Y2lkPTc5MGI3NjExZ29pcmlqY3RnbnFyajhlamlhY21ieDlpaW1iOXZ3cTB1em4ycm01dSZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/GRk3GLfzduq1NtfGt5/giphy.gif)


## Overview

Wut Rust is a powerful command-line interface (CLI) tool built with Rust, designed to enhance your shell experience. Leveraging the Gemini API, it provides intelligent analysis and suggestions for your previously executed commands, making your terminal more efficient and intuitive.

## Features

-   **AI-Powered Command Analysis:** Integrates with the Gemini API to provide intelligent analysis and insights into your most recently executed command.
-   **Contextual Suggestions:** Offers relevant information and next steps based on the command's output.
-   **History Integration:** Reads your shell history to provide context for the AI analysis.
-   **Fast and Reliable:** Built with Rust, ensuring high performance, memory safety, and reliability.
-   **Cross-Platform:** Designed to work seamlessly across various operating systems.

## Prerequisites

Before you begin, ensure you have the following installed on your system:

-   **Rust and Cargo:** Wut Rust is built with Rust. If you don't have Rust and Cargo (Rust's package manager and build system) installed, you can get them by running the following command in your terminal:

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

## Installation

To get a copy of Wut Rust up and running on your local machine, follow these steps:

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

    This command compiles the project in release mode, optimizing it for performance. The executable will be located in `target/release/wut_rust`.

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

After successful installation, you can use Wut Rust to analyze your previous commands.

To get an analysis of your last executed command, simply type `wut` in your terminal:

```bash
$ <your_command_here>
$ huh
```

Wut Rust will then provide an AI-powered analysis and suggestions based on the output of your last command.

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
