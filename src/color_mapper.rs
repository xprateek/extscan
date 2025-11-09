use colored::Color;

pub fn get_color(ext: &str) -> Color {
    match ext {
        // Programming languages and formats
        "json" => Color::BrightYellow,
        "d" => Color::BrightBlue,
        "toml" => Color::BrightRed,
        "txt" => Color::White,
        "el" => Color::BrightBlue,          
        "yml" | "yaml" => Color::BrightBlue,
        "js" => Color::BrightYellow,
        "lock" => Color::White,
        "coffee" => Color::BrightRed,       
        "ls" => Color::BrightMagenta,       
        "lua" => Color::Blue,
        "ini" => Color::BrightYellow,
        "md" | "markdown" | "mkd" => Color::BrightGreen,
        "xml" => Color::BrightGreen,
        "hs" => Color::Blue,
        "rs" => Color::BrightRed,
        "java" => Color::BrightRed,
        "bash" | "sh" => Color::Green,
        "py" => Color::BrightYellow,
        "rb" | "ru" | "gemspec" => Color::BrightRed,
        "clj" | "cljs" => Color::BrightGreen,
        "ex" | "exs" => Color::BrightRed,
        "swift" => Color::BrightRed,
        "scala" => Color::Red,
        "kt" | "kts" => Color::Magenta,
        "dart" => Color::BrightBlue,
        "php" => Color::Magenta,
        "go" => Color::Blue,
        "c" | "cc" | "cpp" | "cxx" | "h" | "hpp" | "hh" => Color::Blue,
        "ts" | "tsx" | "jsx" => Color::BrightBlue,
        "jsonc" => Color::BrightYellow,
        "cmake" | "makefile" => Color::White,
        "bat" | "cmd" => Color::White,
        "sql" => Color::BrightBlue,
        
        // Images and media
        "png" | "apng" => Color::BrightMagenta,
        "jpg" | "jpeg" | "bmp" | "gif" | "heic" | "avif" | "webp" => Color::BrightMagenta,
        "ico" => Color::BrightMagenta,
        "svg" => Color::BrightMagenta,
        "mkv" | "mp4" | "mov" | "avi" | "flv" | "wmv" | "webm" => Color::BrightMagenta,
        "mp3" | "wav" | "flac" | "m3u" | "m3u8" | "ogg" | "opus" => Color::BrightMagenta,

        // Archives and compressed files
        "zip" | "tar" | "gz" | "tgz" | "bz2" | "tbz" | "xz" | "rar" | "7z" => Color::White,

        // Config and data files
        "env" | "cfg" | "conf" | "config" => Color::BrightRed,
        "log" => Color::White,

        // Documents and office files
        "pdf" => Color::BrightRed,
        "doc" | "docx" | "odt" => Color::BrightGreen,
        "xls" | "xlsx" => Color::Green,
        "ppt" | "pptx" => Color::BrightMagenta,
        "rtf" => Color::White,

        // Scripts and shell files
        "zsh" | "zshrc" | "bashrc" | "bash_profile" | "profile" | "fish" => Color::Green,
        "cshrc" | "tcsh" => Color::Green,

        // Misc / Unknown files fallback
        _ => Color::White,
    }
}
