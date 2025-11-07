//! Core simulation logic (no I/O)

use rand::Rng;

/// Result of a dice simulation
pub struct SimResult {
    /// Face counts aggregated across all dice (index 0 = Face 1)
    pub face_counts: Vec<u64>,
    /// Sum counts from `min_sum..=max_sum` (index 0 = min_sum)
    pub sum_counts: Vec<u64>,
    pub min_sum: u32,
    pub max_sum: u32,
}

/// Run a dice simulation
/// - `sides`: Number of sides on each dice
/// - `dice`: Number of dice rolled each time
/// - `rolls`: Number of total rolls to simulate
pub fn run_sim(sides: u32, dice: u32, rolls: u64) -> SimResult {
    let mut rng = rand::thread_rng();

    let mut face_counts = vec![0u64; sides as usize];
    let min_sum = dice;
    let max_sum = dice * sides;
    let mut sum_counts = vec![0u64; (max_sum - min_sum + 1) as usize];

    for _ in 0..rolls {
        let mut sum = 0u32;
        // Invoke every dice in each dice roll
        for _ in 0..dice {
            let face = rng.gen_range(1..=sides);
            sum += face;
            face_counts[(face - 1) as usize] += 1;
        }
        sum_counts[(sum - min_sum) as usize] += 1;
    }

    // Return the simulation result
    SimResult {
        face_counts,
        sum_counts,
        min_sum,
        max_sum,
    }
}
