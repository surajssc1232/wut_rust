# Wut Rust

## Overview

Wut Rust is a powerful and interactive command-line interface (CLI) tool built with Rust, designed to streamline your workflow by providing an intelligent shell experience. Leveraging the Gemini API, it offers advanced features such as AI-powered command suggestions, intelligent history management, and dynamic prompt customization, making your terminal more efficient and intuitive.

## Features

-   **AI-Powered Suggestions:** Integrates with the Gemini API to provide context-aware command suggestions and completions.
-   **Intelligent History:** Smartly manages your command history, allowing for easier retrieval and reuse of past commands.
-   **Customizable Prompt:** Offers flexible options to personalize your shell prompt for better readability and information display.
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
    git clone https://github.com/your-username/wut_rust.git
    cd wut_rust
    ```

    *(Replace `https://github.com/your-username/wut_rust.git` with the actual URL of your repository.)*

2.  **Build the project:**

    ```bash
    cargo build --release
    ```

    This command compiles the project in release mode, optimizing it for performance. The executable will be located in `target/release/wut_rust`.

## Usage

After successful installation, you can run Wut Rust from your terminal. 

To start the interactive shell:

```bash
./target/release/wut_rust
```

*(If you want to run it directly from the source directory without building first, you can use `cargo run`)*

```bash
cargo run
```

Once inside the Wut Rust shell, you can type commands as you normally would. The AI suggestions and history features will be active.

### Examples (Illustrative - actual commands may vary)

-   **Basic Command Execution:**
    ```
    $ ls -l
    ```

-   **AI-Powered Suggestion (e.g., after typing `git co`):**
    ```
    $ git commit -m "Initial commit" (suggestion: git checkout main)
    ```

-   **History Search (e.g., pressing `Ctrl+R` and typing `docker`):**
    ```
    (reverse-i-search)`docker`: docker ps -a
    ```

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

Your Name - your_email@example.com

Project Link: [https://github.com/your-username/wut_rust](https://github.com/your-username/wut_rust)
