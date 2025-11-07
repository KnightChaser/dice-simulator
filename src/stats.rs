//! Small statistics helpers.

/// Calculate the theoretical mean of the sum of `dice` dice each with `sides` sides.
pub fn expected_sum(dice: u32, sides: u32) -> f64 {
    // Mean of one die is (sides + 1) / 2
    (dice as f64) * ((sides + 1) as f64) / 2.0
}

/// Calculate the empirical mean of the sum from the histogram `sum_counts`.
pub fn empirical_mean_sum(sum_counts: &[u64], min_sum: u32, rolls: u64) -> f64 {
    let mut acc = 0f64;
    for (i, &c) in sum_counts.iter().enumerate() {
        let sum_value = (min_sum as usize + i) as f64;
        acc += sum_value * (c as f64);
    }
    acc / (rolls as f64)
}
