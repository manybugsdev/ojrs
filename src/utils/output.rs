use colored::Colorize;

pub fn success(msg: &str) {
    println!("{} {}", "✓".green().bold(), msg);
}

pub fn error(msg: &str) {
    println!("{} {}", "✗".red().bold(), msg);
}

pub fn info(msg: &str) {
    println!("{} {}", "ℹ".blue().bold(), msg);
}

pub fn warning(msg: &str) {
    println!("{} {}", "⚠".yellow().bold(), msg);
}
