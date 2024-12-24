use std::time::Instant;
mod greedy_knapsack;
use greedy_knapsack::*;

fn main() {
    // Parameters
    let capacity: u32 = 1000;
    let value_range: (u32, u32) = (100, 500);
    let weight_range: (u32, u32) = (100, 1500);
    let test_cases: Vec<u32> = vec![100, 1000, 10000, 100000];


    // Greedy Algorithm 1
    println!("------------------------------------------------------------------");
    println!("0/1 Knapsack Problem using Greedy Algorithm: Largest Value First");
    println!("------------------------------------------------------------------");
    println!("N\tRun\tValue\tWeight\tTime(s)");

    for &n in &test_cases {
        let mut times: Vec<f64> = vec![];
        let mut total_value: u32 = 0;
        
        for run in 1..=3 {
            let mut items = generate_items(n, weight_range, value_range);

            let start_1 = Instant::now();
            let (value_1, weight_1) = greedy_largest_value(&mut items, capacity);
            let duration_1 = start_1.elapsed();
            
            let time_taken_1 = duration_1.as_secs_f64();
            times.push(time_taken_1);
            total_value += value_1;
            
            println!("{}\t{}\t{}\t{}\t{:.6}", n, run, value_1, weight_1, time_taken_1);   
        }

        let avg_value = total_value as f64 / times.len() as f64;
        let avg_time: f64 = times.iter().sum::<f64>() / times.len() as f64;
        println!("----------------------------------------");
        println!("Average\t\t{:.6}\t{:.6}", avg_value, avg_time);
        println!("----------------------------------------");
    }

    // Greedy Algorithm 2
    println!("--------------------------------------------------------------------");
    println!("0/1 Knapsack Problem using Greedy Algorithm: Smallest Weight First");
    println!("--------------------------------------------------------------------");
    println!("N\tRun\tValue\tWeight\tTime(s)");

    for &n in &test_cases {
        let mut times: Vec<f64> = vec![];
        let mut total_value: u32 = 0;
        
        for run in 1..=3 {
            let mut items = generate_items(n, weight_range, value_range);

            let start_1 = Instant::now();
            let (value_1, weight_1) = greedy_smallest_weight(&mut items, capacity);
            let duration_1 = start_1.elapsed();
            
            let time_taken_1 = duration_1.as_secs_f64();
            times.push(time_taken_1);
            total_value += value_1;
            
            println!("{}\t{}\t{}\t{}\t{:.6}", n, run, value_1, weight_1, time_taken_1);   
        }

        let avg_value = total_value as f64 / times.len() as f64;
        let avg_time: f64 = times.iter().sum::<f64>() / times.len() as f64;
        println!("----------------------------------------");
        println!("Average\t\t{:.6}\t{:.6}", avg_value, avg_time);
        println!("----------------------------------------");
    }

    // Greedy Algorithm 3
    println!("-------------------------------------------------------------------");
    println!("0/1 Knapsack Problem using Greedy Algorithm: Greatest Worth Ratio");
    println!("-------------------------------------------------------------------");
    println!("N\tRun\tValue\tWeight\tTime(s)");

    for &n in &test_cases {
        let mut times: Vec<f64> = vec![];
        let mut total_value: u32 = 0;
        
        for run in 1..=3 {
            let mut items = generate_items(n, weight_range, value_range);

            let start_1 = Instant::now();
            let (value_1, weight_1) = greedy_value_weight_ratio(&mut items, capacity);
            let duration_1 = start_1.elapsed();
            
            let time_taken_1 = duration_1.as_secs_f64();
            times.push(time_taken_1);
            total_value += value_1;
            
            println!("{}\t{}\t{}\t{}\t{:.6}", n, run, value_1, weight_1, time_taken_1);   
        }

        let avg_value = total_value as f64 / times.len() as f64;
        let avg_time: f64 = times.iter().sum::<f64>() / times.len() as f64;
        println!("----------------------------------------");
        println!("Average\t\t{:.6}\t{:.6}", avg_value, avg_time);
        println!("----------------------------------------");
    }
}