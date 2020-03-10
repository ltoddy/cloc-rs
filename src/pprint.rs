use std::env::current_dir;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::Duration;

use crate::detail::{LanguageDetail, SumDetail};

pub struct PrettyPrinter {}

impl PrettyPrinter {
    pub fn terminal(languages: Vec<LanguageDetail>, sum: SumDetail, elapsed: Duration) {
        println!("{:>12.4} secs", elapsed.as_secs_f64());
        println!("┌────────────────────────────────────────────────────────────────────────┐");
        println!(
            "| {:<10}{:>12}{:>12}{:>12}{:>12}{:>12} |",
            "Language", "files", "Size", "Blank", "Comment", "Code",
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
    pub fn markdown(languages: Vec<LanguageDetail>, sum: SumDetail, elapsed: Duration) {
        let mut filename = current_dir().unwrap();
        filename.push("total.md");

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
            "Language", "files", "Size", "Blank", "Comment", "Code",
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
