use crate::reporter::Report;
use crate::util::bytes_to_size;

pub fn pretty_print(report: Report) -> () {
    let Report { sections, summary } = report;

    println!("┌────────────────────────────────────────────────────────────────────────┐");
    println!(
        "| {:<10}{:>12}{:>12}{:>12}{:>12}{:>12} |",
        "Language", "files", "size", "blank", "comment", "code",
    );
    println!("├────────────────────────────────────────────────────────────────────────┤");

    for detail in sections {
        println!(
            "| {:<10}{:>12}{:>12}{:>12}{:>12}{:>12} |",
            detail.language,
            detail.files,
            bytes_to_size(detail.bytes as f64),
            detail.blank,
            detail.comment,
            detail.code,
        );
    }

    println!("├────────────────────────────────────────────────────────────────────────┤");
    println!(
        "| {:<10}{:>12}{:>12}{:>12}{:>12}{:>12} |",
        "Sum",
        summary.files,
        bytes_to_size(summary.bytes as f64),
        summary.blank,
        summary.comment,
        summary.code,
    );
    println!("└────────────────────────────────────────────────────────────────────────┘");
}
