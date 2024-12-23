use rand::Rng;

#[derive(Debug)]
struct Item {
    value: u32,
    weight: u32,
}

fn generate_items(n:u32) -> Vec<Item> {
    let mut rng = rand::thread_rng();
    let mut items = Vec::new();

    for _ in 0..n {
        let weight = rng.gen_range(100..=1500);
        let value = rng.gen_range(100..=500);
        items.push(Item {value, weight});
    }

    items
}

fn greedy_largest_value(items: &mut Vec<Item>, max_weight: u32) -> (u32, u32, Vec<Item>) {
    items.sort_by(|a, b| b.value.cmp(&a.value));

    let mut total_value = 0;
    let mut solution_weight = 0;
    let mut solution_items = Vec::new();

    for item in items.iter() {
        if solution_weight + item.weight <= max_weight {
            solution_items.push(Item {value: item.value, weight: item.weight});
            total_value += item.value;
            solution_weight += item.weight;
        }
    }

    (total_value, solution_weight, solution_items)
}

fn greedy_smallest_size(items: &mut Vec<Item>, max_weight: u32) -> (u32, u32, Vec<Item>) {
    items.sort_by(|a, b| a.weight.cmp(&b.weight));

    let mut total_value = 0;
    let mut solution_weight = 0;
    let mut solution_items = Vec::new();

    for item in items.iter() {
        if solution_weight + item.weight <= max_weight {
            solution_items.push(Item {value: item.value, weight: item.weight});
            total_value += item.value;
            solution_weight += item.weight;
        }
    }

    (total_value, solution_weight, solution_items)
}

fn greedy_approximation(items: &mut Vec<Item>, max_weight: u32) -> (u32, u32, Vec<Item>) {
    items.sort_by(|a,b| {
        let ratio_a = a.value as f64 / a.weight as f64;
        let ratio_b = b. value as f64 / b.weight as f64;
        ratio_b.partial_cmp(&ratio_a).unwrap()
    });

    let mut total_value = 0;
    let mut solution_weight = 0;
    let mut solution_items = Vec::new();

    for item in items.iter() {
        if solution_weight + item.weight <= max_weight {
            solution_items.push(Item {value: item.value, weight: item.weight});
            total_value += item.value;
            solution_weight += item.weight;
        }
    }

    (total_value, solution_weight, solution_items)
}

fn main() {
    let mut items = generate_items(10);
    
    println!("Generated Items:");
    for (i, item) in items.iter().enumerate() {
        println!("Item {}: Value = {}, Weight = {}", i + 1, item.value, item.weight);
    }

    let max_weight = 1000;

    let (total_value, solution_weight, solution_items) = greedy_largest_value(&mut items, max_weight);

    println!("Greedy Algorithm 1: Largest Value First");
    println!("Solution Items: {:?}", solution_items);
    println!("Total Value: {}", total_value);
    println!("Total Weight: {}", solution_weight);

    let (total_value, solution_weight, solution_items) = greedy_smallest_size(&mut items, max_weight);

    println!("Greedy Algorithm 2: Smallest Size First");
    println!("Solution Items: {:?}", solution_items);
    println!("Total Value: {}", total_value);
    println!("Total Weight: {}", solution_weight);

    let (total_value, solution_weight, solution_items) = greedy_approximation(&mut items, max_weight);

    println!("Greedy Approximation Algorithm: Greatest Worth Ratio");
    println!("Solution Items: {:?}", solution_items);
    println!("Total Value: {}", total_value);
    println!("Total Weight: {}", solution_weight);

}