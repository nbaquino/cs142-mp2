use csv::Writer;
use std::fs;
use std::path::Path;
use std::time::Instant;

use rand::Rng;
use std::cmp::max;

fn knapsack_bottom_up(
    w: usize,
    weights: &Vec<usize>,
    profits: &Vec<usize>,
    n: usize,
    dp: &mut Vec<Vec<usize>>,
) -> (usize, usize) {
    // Initialize the first column to 0
    for i in 0..=n {
        dp[i][0] = 0;
    }
    // Initialize the first row to 0
    for j in 0..=w {
        dp[0][j] = 0;
    }
    for i in 1..=n {
        for j in 1..=w {
            // Iterate from 1 to w
            if j >= weights[i - 1] {
                // Check if the current weight can be included
                dp[i][j] = max(dp[i - 1][j], profits[i - 1] + dp[i - 1][j - weights[i - 1]]);
            } else {
                dp[i][j] = dp[i - 1][j]; // If the weight can't be included, carry forward the previous value
            }
        }
    }

    // Backtrack to get total weight
    let mut total_weight = 0;
    let mut remaining_capacity = w;
    for i in (1..=n).rev() {
        if dp[i][remaining_capacity] != dp[i - 1][remaining_capacity] {
            // This item was included
            total_weight += weights[i - 1];
            remaining_capacity -= weights[i - 1];
        }
    }

    (dp[n][w], total_weight) // Return the maximum profit and total weight
}

fn generate_data(
    n: usize,
    weight_range: (usize, usize),
    profit_range: (usize, usize),
) -> (Vec<usize>, Vec<usize>) {
    let mut rng = rand::thread_rng();
    let weights = (0..n)
        .map(|_| rng.gen_range(weight_range.0..=weight_range.1))
        .collect();
    let profits = (0..n)
        .map(|_| rng.gen_range(profit_range.0..=profit_range.1))
        .collect();
    (weights, profits)
}

fn main() {
    let capacity = 1000;
    let weight_range = (100, 1500);
    let profit_range = (100, 500);
    let test_sizes: Vec<usize> = (1000..=100000).step_by(1000).collect();

    let results_dir = "results";
    if !Path::new(results_dir).exists() {
        fs::create_dir(results_dir).expect("Failed to create results directory");
    }

    let mut wtr_bottomup = Writer::from_path(format!("{}/bottom_up.csv", results_dir)).unwrap();

    let headers = &[
        "N",
        "1st Run",
        "2nd Run",
        "3rd Run",
        "Average",
        "Weight 1",
        "Weight 2",
        "Weight 3",
        "Time 1",
        "Time 2",
        "Time 3",
        "Average Time",
    ];
    wtr_bottomup.write_record(headers).unwrap();

    // Print table header
    println!("----------------------------------------------------");
    println!("0/1 Knapsack Problem using Bottom Up DP (Tabulation)");
    println!("----------------------------------------------------");
    println!("N\tRun\t\tValue\t\tTime(s)");

    for &n in &test_sizes {
        let mut times: Vec<f64> = vec![];
        let mut values = vec![];
        let mut total_weights: Vec<usize> = vec![];

        for run in 1..=3 {
            // Run 3 times for averaging
            let (weights, profits) = generate_data(n, weight_range, profit_range);
            let mut dp = vec![vec![0; capacity + 1]; n + 1];

            let start = Instant::now();
            let (value, total_weight) =
                knapsack_bottom_up(capacity, &weights, &profits, n, &mut dp);
            let duration = start.elapsed();

            let time_taken = duration.as_secs_f64();
            times.push(time_taken);
            values.push(value);
            total_weights.push(total_weight);

            // Print each run's results
            println!("{}\t{}\t\t{}\t\t{:.6}", n, run, value, time_taken);
        }

        let avg_value = values.iter().sum::<usize>() as f64 / values.len() as f64;
        let avg_time: f64 = times.iter().sum::<f64>() / times.len() as f64;

        wtr_bottomup
            .write_record(&[
                n.to_string(),
                values[0].to_string(),
                values[1].to_string(),
                values[2].to_string(),
                avg_value.to_string(),
                total_weights[0].to_string(),
                total_weights[1].to_string(),
                total_weights[2].to_string(),
                times[0].to_string(),
                times[1].to_string(),
                times[2].to_string(),
                avg_time.to_string(),
            ])
            .unwrap();

        // Print average results with borders
        println!("----------------------------------------------------");
        println!("\tAverage\t\t{:.6}\t{:.6}", avg_value, avg_time);
        println!("----------------------------------------------------");
    }
}
