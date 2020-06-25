use std::env::current_dir;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::Duration;

use crate::detail::{LanguageDetail, SumDetail};
use crate::engine::Report;

pub(crate) struct PrettyPrinter {}

impl PrettyPrinter {
    pub(crate) fn terminal(report: Report, elapsed: Duration) {
        let Report { languages, sum } = report;

        println!("{:>12.4} secs", elapsed.as_secs_f64());
        println!("┌────────────────────────────────────────────────────────────────────────┐");
        println!(
            "| {:<10}{:>12}{:>12}{:>12}{:>12}{:>12} |",
            "Language", "files", "size", "blank", "comment", "code",
        );
        println!("├────────────────────────────────────────────────────────────────────────┤");

        for detail in languages {
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
            sum.files,
            bytes_to_size(sum.bytes as f64),
            sum.blank,
            sum.comment,
            sum.code,
        );
        println!("└────────────────────────────────────────────────────────────────────────┘");
    }

    // TODO
    pub(crate) fn markdown(report: Report, elapsed: Duration) {
        let Report { languages, sum } = report;

        let mut filename = current_dir().expect("current working directory value is invalid");
        filename.push("cloc-report.md");

        let mut file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .append(false)
            .open(filename)
            .unwrap();
        let mut template = format!("{:.4} secs\n\n", elapsed.as_secs_f64());

        template.push_str(&format!(
            "| {:<10} | {:>12} | {:>12} | {:>12} | {:>12} | {:>12} |\n",
            "Language", "files", "size", "blank", "comment", "code",
        ));
        template.push_str(&format!(
            "|-{}-|-{}-|-{}-|-{}-|-{}-|-{}-|\n",
            "-".repeat(10),
            "-".repeat(12),
            "-".repeat(12),
            "-".repeat(12),
            "-".repeat(12),
            "-".repeat(12),
        ));
        for detail in languages {
            template.push_str(&format!(
                "| {:<10} | {:>12} | {:>12} | {:>12} | {:>12} | {:>12} |\n",
                detail.language,
                detail.files,
                bytes_to_size(detail.bytes as f64),
                detail.blank,
                detail.comment,
                detail.code
            ));
        }
        template.push_str(&format!(
            "| {:<10} | {:>12} | {:>12} | {:>12} | {:>12} | {:>12} |\n",
            "Sum",
            sum.files,
            bytes_to_size(sum.bytes as f64),
            sum.blank,
            sum.comment,
            sum.code
        ));

        file.write_all(template.as_bytes()).unwrap();
    }

    // TODO
    pub(crate) fn _html(_languages: Vec<LanguageDetail>, _sum: SumDetail, _elapsed: Duration) {}
}

const SIZES: [&str; 9] = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];

fn bytes_to_size(bytes: f64) -> String {
    let k = 1024_f64;
    if bytes <= 1_f64 {
        return format!("{:.2} B", bytes);
    }
    let i = (bytes.ln() / k.ln()) as i32;
    format!("{:.2} {}", bytes / k.powi(i), SIZES[i as usize])
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_bytes_to_size() {
        let one_kb = 1024f64;

        assert_eq!(bytes_to_size(one_kb), "1.00 KB")
    }
}
