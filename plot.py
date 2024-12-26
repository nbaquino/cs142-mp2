import matplotlib.pyplot as plt
import pandas as pd

# Data for Dynamic Programming Bottom Up (Tabulation)
dp_bottom_up_data = {
    'N': [5, 10, 100, 1000, 5000, 10000, 25000, 50000, 75000, 100000],
    'Average Value': [765.67, 866.33, 2237.67, 3467.33, 4272.0, 4393.67, 4464.33, 4482.0, 4597.0, 4766.67],
    'Average Time (s)': [0.000312, 0.000529, 0.005562, 0.055401, 0.275933, 0.607086, 1.674860, 3.100319, 4.113612, 5.322388]
}

# Data for Dynamic Programming Top Down (Memoization)
dp_top_down_data = {
    'N': [5, 10, 100, 1000, 5000, 10000, 25000, 50000, 75000, 100000],
    'Average Value': [389.0, 888.33, 2371.0, 3662.0, 4224.33, 4438.33, 4465.67, 4501.33, 4709.33, 4779.0],
    'Average Time (s)': [0.000002, 0.000005, 0.002549, 0.068912, 0.488070, 1.322124, 3.062313, 7.685362, 9.468768, 15.502679]
}

# Create DataFrames
df_dp_bottom_up = pd.DataFrame(dp_bottom_up_data)
df_dp_top_down = pd.DataFrame(dp_top_down_data)

# Create figure and axis objects
plt.figure(figsize=(10, 6))

# Plot for DP Bottom Up (Tabulation)
plt.plot(df_dp_bottom_up['N'], df_dp_bottom_up['Average Time (s)'], label='DP Bottom Up (Tabulation)', marker='o', color='blue')

# Plot for DP Top Down (Memoization)
plt.plot(df_dp_top_down['N'], df_dp_top_down['Average Time (s)'], label='DP Top Down (Memoization)', marker='o', color='red')

# Add labels and title
plt.title('Comparison of Dynamic Programming Bottom Up vs Top Down')
plt.xlabel('N')
plt.ylabel('Average Time (s)')
plt.grid(True)

# Add legend
plt.legend()

# Display the plot
plt.tight_layout()
plt.show()
