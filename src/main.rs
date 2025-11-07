// src/main.rs
use rand::Rng;

mod histogram;
mod io_util;
mod stats;

use histogram::print_histogram_counts;
use io_util::{prompt_u32_default, prompt_u64_default};
use stats::{empirical_mean_sum, expected_sum};

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

fn labels_1_to_n(n: u32) -> Vec<String> {
    (1..=n).map(|x| x.to_string()).collect()
}
