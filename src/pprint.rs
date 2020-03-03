use std::env::current_dir;
use std::fs::OpenOptions;
use std::time::Duration;

use crate::detail::TotalDetail;
use std::io::Write;

pub struct PrettyPrinter {}

impl PrettyPrinter {
    pub fn terminal(total: TotalDetail, total_text_files: usize, ignored_files: usize, elapsed: Duration) {
        let TotalDetail { kinds, sum } = total;

        println!();
        println!("{:>12} text files.", total_text_files);
        println!("{:>12} files ignored.", ignored_files);
        println!();

        println!("{:>12.4} secs", elapsed.as_secs_f64());
        println!("┌────────────────────────────────────────────────────┐");
        println!("| {:<14}{:>12}{:>12}{:>12} |", "Language", "Code", "Comment", "Blank");
        println!("├────────────────────────────────────────────────────┤");

        for detail in kinds.values() {
            println!(
                "| {:<14}{:>12}{:>12}{:>12} |",
                detail.language, detail.code, detail.comment, detail.blank,
            );
        }

        println!("├────────────────────────────────────────────────────┤");
        println!("| {:<14}{:>12}{:>12}{:>12} |", "Sum", sum.code, sum.comment, sum.blank);
        println!("└────────────────────────────────────────────────────┘");
    }

    // TODO
    pub fn markdown(total: TotalDetail, total_text_files: usize, ignored_files: usize, elapsed: Duration) {
        let TotalDetail { kinds, sum } = total;
        let mut filename = current_dir().unwrap();
        filename.push("total.md");

        let mut file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .append(false)
            .open(filename)
            .unwrap();
        let mut template = format!(
            "{} text files.\n\n{} files ignored.\n\n{:.4} secs\n\n",
            total_text_files,
            ignored_files,
            elapsed.as_secs_f64()
        );

        template.push_str("| Language      |        Code |     Comment |       Blank |\n");
        template.push_str("----------------|-------------|-------------|--------------\n");
        for detail in kinds.values() {
            template.push_str(&format!(
                "| {:<13} | {:>11} | {:>11} | {:>11} |\n",
                detail.language, detail.code, detail.comment, detail.blank
            ));
        }
        template.push_str(&format!(
            "| {:<13} | {:>11} | {:>11} | {:>11} |\n",
            "Sum", sum.code, sum.comment, sum.blank
        ));

        file.write_all(template.as_bytes()).unwrap();
    }
}
