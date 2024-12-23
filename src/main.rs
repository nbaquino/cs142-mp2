mod knapsack_bottom_up;
use std::time::Instant;
use rand::Rng;
use knapsack_bottom_up::knapsack_bottom_up;


fn generate_data(n: usize, weight_range: (usize, usize), profit_range: (usize, usize)) -> (Vec<usize>, Vec<usize>) {
    let mut rng = rand::thread_rng();
    let weights = (0..n).map(|_| rng.gen_range(weight_range.0..=weight_range.1)).collect();
    let profits = (0..n).map(|_| rng.gen_range(profit_range.0..=profit_range.1)).collect();
    (weights, profits)
}

fn main() {
    let capacity = 1000;
    let weight_range = (100, 1500);
    let profit_range = (100, 500);

    let test_sizes = vec![100, 1000, 5000, 10000, 25000, 50000, 75000, 100000];

    // Print table header
    println!("----------------------------------------------------");
    println!("0/1 Knapsack Problem using Bottom Up DP (Tabulation)");
    println!("----------------------------------------------------");
    println!("N\tRun\t\tValue\t\tTime(s)");

    for &n in &test_sizes {
        let mut times = vec![];
        let mut total_value = 0;
        for run in 1..=3 { // Run 3 times for averaging
            let (weights, profits) = generate_data(n, weight_range, profit_range);
            let mut dp = vec![vec![0; capacity + 1]; n + 1];

            let start = Instant::now();
            let value = knapsack_bottom_up(capacity, &weights, &profits, n, &mut dp);
            let duration = start.elapsed();

            let time_taken = duration.as_secs_f64();
            times.push(time_taken);
            total_value += value;
            // Print each run's results
            println!("{}\t{}\t\t{}\t\t{:.6}", n, run, value, time_taken);
        }

        let avg_value = total_value as f64 / times.len() as f64;
        let avg_time: f64 = times.iter().sum::<f64>() / times.len() as f64;
        // Print average results with borders
        println!("----------------------------------------------------");
        println!("\tAverage\t\t{:.6}\t{:.6}", avg_value, avg_time);
        println!("----------------------------------------------------");
    }
}
