use colored::*;
use super::DebugInfo;

pub fn print_error(name: &'static str, msg: String, info: DebugInfo) {
    println!(
        "{}{} {} at line {} col {}",
        name.red(),
        ":".red(),
        msg,
        info.line,
        info.col
    );

    print_snippet(info);
}

fn print_snippet(info: DebugInfo) {
    let snippet_prefix = format!("    {} | ", info.line);

    println!(
        "{}{}",
        snippet_prefix,
        info.src.lines().nth(info.line-1).unwrap()
    );

    println!(
        "{}{}{}",
        " ".repeat(snippet_prefix.len() + info.col-1),
        "~".repeat(info.len-1).red(),
        "^".red()
    )
}
