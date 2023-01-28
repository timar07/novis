use colored::*;
use super::DebugInfo;

pub fn print_error(name: &'static str, msg: String, info: DebugInfo) {
    println!(
        "{}{} {} ({}:{}:{})",
        name.red(),
        ":".red(),
        msg,
        info.fname,
        info.line,
        info.col
    );

    print_snippet(info);
}

fn print_snippet(info: DebugInfo) {
    let snippet_prefix = format!("    {} | ", info.line)
        .bright_black();

    if let Some(line) = info.src.lines().nth(info.line-1) {
        println!(
            "{}{}",
            snippet_prefix,
            line
        );

        println!(
            "{}{}{}",
            " ".repeat(snippet_prefix.len() + info.col-1),
            "~".repeat(info.len-1).red(),
            "^".red()
        )
    }
}
