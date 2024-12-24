use rand::Rng;

pub fn generate_items(n:u32, weight_range: (u32, u32), value_range: (u32, u32)) -> Vec<(u32, u32)> {
    let mut rng = rand::thread_rng();

    (0..n)
        .map(|_| {
            let weight = rng.gen_range(weight_range.0..=weight_range.1);
            let value = rng.gen_range(value_range.0..=value_range.1);
            (value, weight)
        })
        .collect()
}

pub fn greedy_largest_value(items: &mut Vec<(u32, u32)>, max_weight: u32) -> (u32, u32) {
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

pub fn greedy_smallest_weight(items: &mut Vec<(u32, u32)>, max_weight: u32) -> (u32, u32) {
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

pub fn greedy_value_weight_ratio(items: &mut Vec<(u32, u32)>, max_weight: u32) -> (u32, u32) {
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
