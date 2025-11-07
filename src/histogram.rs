//! Tiny ASCII histogram printer

/// Print a horizontal bar chart for `counts`.
/// If `labels_opt` is `Some`, those are printed; otherwise indices are used.
/// `total` is used to compute percentages.
pub fn print_histogram_counts(counts: &[u64], labels_opt: Option<&[String]>, total: u64) {
    let max_count = counts.iter().copied().max().unwrap_or(1);
    let bar_max = 50usize;

    for (i, &c) in counts.iter().enumerate() {
        let label: String = labels_opt
            .and_then(|ls| ls.get(i).cloned())
            .unwrap_or_else(|| i.to_string());

        let percent = (c as f64) / (total as f64) * 100.0;
        let bar_len = if max_count == 0 {
            0
        } else {
            ((c as f64) / (max_count as f64) * (bar_max as f64)).round() as usize
        };
        let bar = "█".repeat(bar_len);
        println!("{:>3}: {:>10} ({:>6.2}%) {}", label, c, percent, bar);
    }
}
