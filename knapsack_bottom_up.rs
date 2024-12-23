use std::cmp::max;

pub fn knapsack_bottom_up(w: usize, weights: &Vec<usize>, profits: &Vec<usize>, n: usize, dp: &mut Vec<Vec<usize>>) -> usize {
    for i in 0..n + 1 {
        dp[i][0] = 0;
    }
    for i in 0..w + 1 {
        dp[0][i] = 0;
    }
    for i in 1..n + 1 {
        for j in 1..weights[i - 1] {
            dp[i][j] = dp[i - 1][j];
        }
        for j in weights[i - 1]..w + 1 {
            dp[i][j] = max(dp[i - 1][j], profits[i - 1] + dp[i - 1][j - weights[i - 1]]);
        }
    }
    dp[n][w]
}
