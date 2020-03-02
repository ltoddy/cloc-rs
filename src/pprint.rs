use crate::detail::TotalDetail;

pub struct PrettyPrinter {}

impl PrettyPrinter {
    pub fn terminal(total: TotalDetail) {
        println!("┌────────────────────────────────────────────────────┐");
        println!("| {:<14}{:>12}{:>12}{:>12} |", "Language", "Code", "Comment", "Blank");
        println!("├────────────────────────────────────────────────────┤");

        for detail in total.inner.values() {
            println!(
                "| {:<14}{:>12}{:>12}{:>12} |",
                detail.language, detail.code, detail.comment, detail.blank,
            );
        }

        println!("└────────────────────────────────────────────────────┘");
    }
}
