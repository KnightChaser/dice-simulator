// src/main.rs
use rand::Rng;
use std::io::{self, Write};

fn main() {
    println!("=== Dice Roller Simulation ===");
    let sides = prompt_u32_default("Enter number of sides on the die (default 6): ", 6);
    let dice = prompt_u32_default("Number of dice per roll (default 1): ", 1);
    let rolls = prompt_u64_default("Number of rolls to simulate (default 10_000): ", 10_000);

    if sides < 2 || dice < 1 || rolls == 0 {
        eprintln!("Invalid input values. Please ensure sides >= 2, dice >= 1, and rolls > 0.");
        return;
    }

    println!("\nRolling {dice}d{sides} for {rolls} times...\n");

    // Face frequency across all dice
    let mut face_counts = vec![0u64; sides as usize];

    // Sum-of-dice histogram
    let min_sum: u32 = dice;
    let max_sum: u32 = dice * sides;
    let mut sum_counts = vec![0u64; (max_sum - min_sum + 1) as usize];

    let mut rng = rand::thread_rng();

    // Repeat the rolling process
    for _ in 0..rolls {
        let mut sum = 0u32;
        // Roll every dice sequentially and sum up
        for _ in 0..dice {
            let face = rng.gen_range(1..=sides);
            sum += face;
            face_counts[(face - 1) as usize] += 1;
        }
        sum_counts[(sum - min_sum) as usize] += 1;
    }

    let total_faces = rolls * (dice as u64);
    println!("-- Face frequencies (aggregated across all dice) --");
    print_histogram_counts(&face_counts, Some(&labels_1_to_n(sides)), total_faces);

    println!("\n-- Sum of {} dice ({}..={}) --", dice, min_sum, max_sum);
    print_histogram_counts(&sum_counts, None, rolls);
    println!();

    // Quick stats
    let mean_sum = expected_sum(dice, sides);
    let empirical_mean = empirical_mean_sum(&sum_counts, min_sum, rolls);
    println!("Theoretical mean sum: {:.4}", mean_sum);
    println!("Empirical mean sum  : {:.4}", empirical_mean);
}

fn prompt_u32_default(msg: &str, default: u32) -> u32 {
    print!("{msg}");
    io::stdout().flush().ok();
    let mut s = String::new();

    if io::stdin().read_line(&mut s).is_ok() {
        // Proceed only if read_line was successful
        let t = s.trim();
        if t.is_empty() {
            // If trimmed input is empty, return default
            return default;
        }
        if let Ok(v) = t.parse::<u32>() {
            // If parsing is successful, return the parsed value
            return v;
        }
    }
    default
}

fn prompt_u64_default(msg: &str, default: u64) -> u64 {
    print!("{msg}");
    io::stdout().flush().ok();
    let mut s = String::new();

    if io::stdin().read_line(&mut s).is_ok() {
        // Proceed only if read_line was successful
        let t = s.trim();
        if t.is_empty() {
            // If trimmed input is empty, return default
            return default;
        }
        if let Ok(v) = t.parse::<u64>() {
            // If parsing is successful, return the parsed value
            return v;
        }
    }
    default
}

fn labels_1_to_n(n: u32) -> Vec<String> {
    (1..=n).map(|x| x.to_string()).collect()
}

fn print_histogram_counts(counts: &[u64], labels_opt: Option<&[String]>, total: u64) {
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

fn expected_sum(dice: u32, sides: u32) -> f64 {
    // Mean of one die is (sides + 1) / 2
    (dice as f64) * ((sides + 1) as f64) / 2.0
}

fn empirical_mean_sum(sum_counts: &[u64], min_sum: u32, rolls: u64) -> f64 {
    let mut acc = 0f64;
    for (i, &c) in sum_counts.iter().enumerate() {
        let sum_value = (min_sum as usize + i) as f64;
        acc += sum_value * (c as f64);
    }
    acc / (rolls as f64)
}
