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
    dp[n][w] // Return the maximum profit for n items and capacity w
}
