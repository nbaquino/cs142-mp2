use std::time::Instant;
use rand::Rng;

fn generate_items(n:u32, weight_range: (u32, u32), value_range: (u32, u32)) -> Vec<(u32, u32)> {
    let mut rng = rand::thread_rng();

    (0..n)
        .map(|_| {
            let weight = rng.gen_range(weight_range.0..=weight_range.1);
            let value = rng.gen_range(value_range.0..=value_range.1);
            (value, weight)
        })
        .collect()
}

fn greedy_largest_value(items: &mut Vec<(u32, u32)>, max_weight: u32) -> (u32, u32) {
    items.sort_by(|a, b| b.0.cmp(&a.0));

    let mut total_value = 0;
    let mut total_weight = 0;

    for item in items.iter() {
        if total_weight + item.1 <= max_weight {
            total_value += item.0;
            total_weight += item.1;
        }
    }

    (total_value, total_weight)
}

fn greedy_smallest_weight(items: &mut Vec<(u32, u32)>, max_weight: u32) -> (u32, u32) {
    items.sort_by(|a, b| a.1.cmp(&b.1));

    let mut total_value = 0;
    let mut total_weight = 0;

    for item in items.iter() {
        if total_weight + item.1 <= max_weight {
            total_value += item.0;
            total_weight += item.1;
        }
    }

    (total_value, total_weight)
}

fn greedy_value_weight_ratio(items: &mut Vec<(u32, u32)>, max_weight: u32) -> (u32, u32) {
    items.sort_by(|a, b| {
        let ratio_a = a.0 as f64 / a.1 as f64;
        let ratio_b = b.0 as f64 / b.1 as f64;
        ratio_b.partial_cmp(&ratio_a).unwrap()
    });

    let mut total_value = 0;
    let mut total_weight = 0;

    for item in items.iter() {
        if total_weight + item.1 <= max_weight {
            total_value += item.0;
            total_weight += item.1;
        }
    }

    (total_value, total_weight)
}
fn main() {
    let capacity: u32 = 1000;
    let value_range: (u32, u32) = (100, 500);
    let weight_range: (u32, u32) = (100, 1500);
    let test_cases: Vec<u32> = vec![100, 1000, 10000, 100000];

    println!("------------------------------------------------------------------");
    println!("0/1 Knapsack Problem using Greedy Algorithm 1: Largest Value First");
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

    println!("--------------------------------------------------------------------");
    println!("0/1 Knapsack Problem using Greedy Algorithm 2: Smallest Weight First");
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

    println!("-------------------------------------------------------------------");
    println!("0/1 Knapsack Problem using Greedy Algorithm 3: Greatest Worth Ratio");
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
