use std::time::Instant;
use csv::Writer;
mod greedy_knapsack;
use greedy_knapsack::*;

fn main() {
    // Parameters
    let capacity: u32 = 1000;
    let value_range: (u32, u32) = (100, 500);
    let weight_range: (u32, u32) = (100, 1500);
    let test_cases: Vec<u32> = (1000..=100000).step_by(1000).collect();

    let mut wtr_largest_value = Writer::from_path("greedy_largest_value.csv").unwrap();
    let mut wtr_smallest_weight = Writer::from_path("greedy_smallest_weight.csv").unwrap();
    let mut wtr_value_weight_ratio = Writer::from_path("greedy_value_weight_ratio.csv").unwrap();

    let headers = &["N", "1st Run", "2nd Run", "3rd Run", "Average", "Value 1", "Value 2", "Value 3"];
    wtr_largest_value.write_record(headers).unwrap();
    wtr_smallest_weight.write_record(headers).unwrap();
    wtr_value_weight_ratio.write_record(headers).unwrap();

    // Greedy Algorithm 1: Largest Value First
    println!("------------------------------------------------------------------");
    println!("0/1 Knapsack Problem using Greedy Algorithm: Largest Value First");
    println!("------------------------------------------------------------------");

    for &n in &test_cases {
        let mut times: Vec<f64> = vec![];
        let mut values: Vec<u32> = vec![];
        let mut weights: Vec<u32> = vec![];

        for run in 1..=3 {
            let mut items = generate_items(n, weight_range, value_range);

            let start_1 = Instant::now();
            let (value_1, weight_1) = greedy_largest_value(&mut items, capacity);
            let duration_1 = start_1.elapsed();

            let time_taken_1 = duration_1.as_secs_f64();
            times.push(time_taken_1);
            values.push(value_1);
            weights.push(weight_1);

            println!("{}-{}:\t{}\t{}\t{}", n, run, value_1, weight_1, time_taken_1);
        }

        let avg_value = values.iter().sum::<u32>() as f64 / values.len() as f64;
        let avg_time: f64 = times.iter().sum::<f64>() / times.len() as f64;

        wtr_largest_value.write_record(&[
            n.to_string(),
            values[0].to_string(),
            values[1].to_string(),
            values[2].to_string(),
            avg_value.to_string(),
            values[0].to_string(),
            values[1].to_string(),
            values[2].to_string(),
        ]).unwrap();

        println!("Average value: {:.6}, Average time: {:.6}", avg_value, avg_time);
        println!("----------------------------------------");
    }

    // Greedy Algorithm 2: Smallest Weight First
    println!("--------------------------------------------------------------------");
    println!("0/1 Knapsack Problem using Greedy Algorithm: Smallest Weight First");
    println!("--------------------------------------------------------------------");

    for &n in &test_cases {
        let mut times: Vec<f64> = vec![];
        let mut values: Vec<u32> = vec![];
        let mut weights: Vec<u32> = vec![];

        for run in 1..=3 {
            let mut items = generate_items(n, weight_range, value_range);

            let start_1 = Instant::now();
            let (value_1, weight_1) = greedy_smallest_weight(&mut items, capacity);
            let duration_1 = start_1.elapsed();

            let time_taken_1 = duration_1.as_secs_f64();
            times.push(time_taken_1);
            values.push(value_1);
            weights.push(weight_1);

            println!("{}-{}:\t{}\t{}\t{}", n, run, value_1, weight_1, time_taken_1);
        }

        let avg_value = values.iter().sum::<u32>() as f64 / values.len() as f64;
        let avg_time: f64 = times.iter().sum::<f64>() / times.len() as f64;

        wtr_smallest_weight.write_record(&[
            n.to_string(),
            values[0].to_string(),
            values[1].to_string(),
            values[2].to_string(),
            avg_value.to_string(),
            values[0].to_string(),
            values[1].to_string(),
            values[2].to_string(),
        ]).unwrap();

        println!("Average value: {:.6}, Average time: {:.6}", avg_value, avg_time);
        println!("----------------------------------------");
    }

    // Greedy Algorithm 3: Greatest Worth Ratio
    println!("-------------------------------------------------------------------");
    println!("0/1 Knapsack Problem using Greedy Algorithm: Greatest Worth Ratio");
    println!("-------------------------------------------------------------------");

    for &n in &test_cases {
        let mut times: Vec<f64> = vec![];
        let mut values: Vec<u32> = vec![];
        let mut weights: Vec<u32> = vec![];

        for run in 1..=3 {
            let mut items = generate_items(n, weight_range, value_range);

            let start_1 = Instant::now();
            let (value_1, weight_1) = greedy_value_weight_ratio(&mut items, capacity);
            let duration_1 = start_1.elapsed();

            let time_taken_1 = duration_1.as_secs_f64();
            times.push(time_taken_1);
            values.push(value_1);
            weights.push(weight_1);

            println!("{}-{}:\t{}\t{}\t{}", n, run, value_1, weight_1, time_taken_1);
        }

        let avg_value = values.iter().sum::<u32>() as f64 / values.len() as f64;
        let avg_time: f64 = times.iter().sum::<f64>() / times.len() as f64;

        wtr_value_weight_ratio.write_record(&[
            n.to_string(),
            values[0].to_string(),
            values[1].to_string(),
            values[2].to_string(),
            avg_value.to_string(),
            values[0].to_string(),
            values[1].to_string(),
            values[2].to_string(),
        ]).unwrap();

        println!("Average value: {:.6}, Average time: {:.6}", avg_value, avg_time);
        println!("----------------------------------------");
    }

    wtr_largest_value.flush().unwrap();
    wtr_smallest_weight.flush().unwrap();
    wtr_value_weight_ratio.flush().unwrap();
}
