mod histogram;
mod io_util;
mod sim;
mod stats;

use histogram::print_histogram_counts;
use io_util::{prompt_u32_default, prompt_u64_default};
use sim::run_sim;
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

    let sim = run_sim(sides, dice, rolls);

    println!("-- Face frequencies (aggregated across all dice) --");
    print_histogram_counts(
        &sim.face_counts,
        Some(&(1..=sides).map(|x| x.to_string()).collect::<Vec<_>>()),
        rolls * (dice as u64),
    );

    println!(
        "\n-- Sum of {} dice ({}..={}) --",
        dice, sim.min_sum, sim.max_sum
    );
    print_histogram_counts(
        &sim.sum_counts,
        Some(
            &(sim.min_sum..=sim.max_sum)
                .map(|x| x.to_string())
                .collect::<Vec<_>>(),
        ),
        rolls,
    );
    println!();

    let theo = expected_sum(dice, sides);
    let emp = empirical_mean_sum(&sim.sum_counts, sim.min_sum, rolls);
    println!("Theoretical mean sum: {:.4}", theo);
    println!("Empirical mean sum  : {:.4}", emp);
}
