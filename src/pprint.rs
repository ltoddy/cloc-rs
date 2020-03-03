use crate::detail::TotalDetail;

pub struct PrettyPrinter {}

impl PrettyPrinter {
    pub fn terminal(total: TotalDetail, total_text_files: usize, ignored_files: usize) {
        let TotalDetail { inner: kinds, sum } = total;

        println!();
        println!("{:>12} text files.", total_text_files);
        println!("{:>12} files ignored.", ignored_files);
        println!();

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
}
