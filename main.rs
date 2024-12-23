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

    for &n in &test_sizes {
        println!("Testing for n = {}", n);

        let mut times = vec![];
        for _ in 0..3 { // Run 3 times for averaging
            let (weights, profits) = generate_data(n, weight_range, profit_range);
            let mut dp = vec![vec![0; capacity + 1]; n + 1];

            let start = Instant::now();
            knapsack_bottom_up(capacity, &weights, &profits, n, &mut dp);
            let duration = start.elapsed();

            times.push(duration.as_secs_f64());
        }

        let avg_time: f64 = times.iter().sum::<f64>() / times.len() as f64;
        println!("n = {}, Average Time = {:.6} seconds", n, avg_time);
    }
}
