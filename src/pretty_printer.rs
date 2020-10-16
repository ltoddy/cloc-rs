use std::time::Duration;

use crate::reporter::Report;
use crate::util::bytes_to_size;

pub fn pretty_print(report: Report, elapsed: Duration) {
    let Report { sections, summary } = report;

    println!("{:>12.4} secs", elapsed.as_secs_f64());
    println!("┌───────────────────────────────────────────────────────────────────────────────────────┐");
    println!(
        "│ {:<25}{:>12}{:>12}{:>12}{:>12}{:>12} │",
        "Language", "files", "size", "blank", "comment", "code",
    );
    println!("├───────────────────────────────────────────────────────────────────────────────────────┤");

    for detail in sections {
        println!(
            "│ {:<25}{:>12}{:>12}{:>12}{:>12}{:>12} │",
            detail.language,
            detail.files,
            bytes_to_size(detail.bytes as f64),
            detail.blank,
            detail.comment,
            detail.code,
        );
    }

    println!("├───────────────────────────────────────────────────────────────────────────────────────┤");
    println!(
        "│ {:<25}{:>12}{:>12}{:>12}{:>12}{:>12} │",
        "Sum",
        summary.files,
        bytes_to_size(summary.bytes as f64),
        summary.blank,
        summary.comment,
        summary.code,
    );
    println!("└───────────────────────────────────────────────────────────────────────────────────────┘");
}
