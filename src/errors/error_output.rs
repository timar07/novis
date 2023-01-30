use colored::*;
use super::DebugInfo;

pub fn print_error(name: &'static str, msg: String, info: DebugInfo) {
    eprintln!(
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

// TODO: Implement
#[allow(dead_code)]
fn print_multiline_snippet(_: DebugInfo) {

}

fn print_snippet(info: DebugInfo) {
    let snippet_prefix = format!("    {} | ", info.line)
        .bright_black();

    if let Some(line) = info.src.lines().nth(info.line-1) {
        eprintln!(
            "{}{}",
            snippet_prefix,
            line
        );

        eprintln!(
            "{}{}{}",
            " ".repeat(snippet_prefix.len() + info.col-1),
            "~".repeat(info.len-1).red(),
            "^".red()
        )
    }
}
