pub mod colors {
    pub const RESET: &str = "\x1b[0m";
    pub const BOLD: &str = "\x1b[1m";
    pub const ITALIC: &str = "\x1b[3m";
    pub const BLUE: &str = "\x1b[34m";
    pub const YELLOW: &str = "\x1b[33m";
    pub const RED: &str = "\x1b[31m";
    pub const CYAN: &str = "\x1b[36m";
}

pub mod api {
    pub const GEMINI_MODEL_FAST: &str = "gemini-2.5-flash-lite-preview-06-17";
    pub const GEMINI_MODEL_STANDARD: &str = "gemini-2.0-flash";
}

pub mod formatting {
    pub const MAX_LINE_WIDTH: usize = 100;
} 