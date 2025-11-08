use colored::Color;

pub fn get_color(ext: &str) -> Color {
    let code_ext = [
        "rs", "c", "cpp", "h", "hpp", "py", "js", "ts", "java", "cs", "sh", "bash", "php", // #TODO add more as needed
    ];
    let non_code_ext = [
        "png", "jpg", "jpeg", "gif", "svg", "xml", "json", "md", "html", "css", "txt", // #TODO add more as needed
    ];

    if code_ext.contains(&ext) {
        Color::BrightBlue
    } else if non_code_ext.contains(&ext) {
        Color::BrightGreen
    } else {
        Color::White
    }
}
