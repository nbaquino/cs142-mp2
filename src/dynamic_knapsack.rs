use std::cmp::max;

pub fn knapsack_bottom_up(w: usize, weights: &Vec<usize>, profits: &Vec<usize>, n: usize, dp: &mut Vec<Vec<usize>>) -> usize {
    // Initialize the first column to 0
    for i in 0..=n {
        dp[i][0] = 0;
    }
    // Initialize the first row to 0
    for j in 0..=w {
        dp[0][j] = 0;
    }
    for i in 1..=n {
        for j in 1..=w { // Iterate from 1 to w
            if j >= weights[i - 1] { // Check if the current weight can be included
                dp[i][j] = max(dp[i - 1][j], profits[i - 1] + dp[i - 1][j - weights[i - 1]]);
            } else {
                dp[i][j] = dp[i - 1][j]; // If the weight can't be included, carry forward the previous value
            }
        }
    }
    return dp[n][w] // Return the maximum profit for n items and capacity w
}

pub fn knapsack_top_down(w:usize, weights:&Vec<usize>, profits:&Vec<usize>, n:usize, dp: &mut Vec<Vec<i64>>) -> i64{

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
