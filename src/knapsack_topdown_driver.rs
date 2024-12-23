use std::cmp::max;
use std::time::Instant;
use rand::Rng;


fn knapsack_top_down(w:usize, weights:&Vec<usize>, profits:&Vec<usize>, n:usize, dp: &mut Vec<Vec<i64>>) -> i64{

    // If we have 0 elements remaining or knapsack is already filled, return 0
    if n==0 || w == 0 {
        dp[n][w] = 0;
        return 0;
    }

    // If already calculated result earlier, return it
    if dp[n][w] != -1 {
        return dp[n][w];
    }
 
    // If the nth element has higher weight than available capacity,
    // it can not be carried. So, return without including item
    if weights[n-1] > w {
        dp[n][w] = knapsack_top_down(w, weights, profits, n-1, dp);
        return dp[n][w];
    }

    // Else, we check by including and excluding the given item
    // And return max of it

    dp[n][w] = max(
        // If we exclude item, simply return function for n-1 items
        knapsack_top_down(w, weights, profits, n-1, dp),

        // If we include item, return profit of given item +
        // maximum value from given weight for remaining items
        profits[n-1] as i64 + knapsack_top_down(w-weights[n-1], weights, profits, n-1, dp));

    return dp[n][w];
}


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

    // Print table header for Top-Down (Memoization)
    println!("----------------------------------------------------");
    println!("0/1 Knapsack Problem using Top-down (Memoization)");
    println!("----------------------------------------------------");
    println!("N\tRun\t\tValue\t\tTime(s)");

    for &n in &test_sizes {
        let mut times = vec![];
        let mut total_value = 0;
        for run in 1..=3 { // Run 3 times for averaging
            let (weights, profits) = generate_data(n, weight_range, profit_range);
            let mut dp = vec![vec![-1; capacity + 1]; n + 1]; // Initialize with -1 for memoization

            let start = Instant::now();
            let value = knapsack_top_down(capacity, &weights, &profits, n, &mut dp); // Using the new knapsack function
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