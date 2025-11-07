//! Tiny ASCII histogram printer

/// Print a horizontal bar chart for `counts`.
/// If `labels_opt` is `Some`, those are printed; otherwise indices are used.
/// `total` is used to compute percentage
pub fn print_histogram_counts(counts: &[u64], labels_opt: Option<&[String]>, total: u64) {
    let max_count: u64 = counts.iter().copied().max().unwrap_or(1);
    let bar_max: usize = 50;

    for (i, &c) in counts.iter().enumerate() {
        let label = match labels_opt {
            Some(labels) => labels[i].as_str(),
            None => {
                // NOTE:
                // If label is not provided...
                ""
            }
        };

        let ratio = (c as f64) / (total as f64);
        let bar_len = if max_count == 0 {
            0
        } else {
            ((c as f64) / (max_count as f64) * (bar_max as f64)).round() as usize
        };
        let bar = "█".repeat(bar_len);

        if labels_opt.is_some() {
            println!("{:>3}: {:>10} ({:>6.2}%) {}", label, c, ratio * 100.0, bar);
        } else {
            // NOTE:
            // If label is not provided...
            println!("k={:>2}: {:>10} ({:>6.2}%) {}", i, c, ratio * 100.0, bar);
        }
    }
}
