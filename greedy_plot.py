import matplotlib.pyplot as plt
import pandas as pd

# Data for Greedy Algorithms
largest_value_first = pd.read_csv("results/greedy_largest_value.csv")
smallest_weight_first = pd.read_csv("results/greedy_smallest_weight.csv")
greatest_worth_ratio = pd.read_csv("results/greedy_value_weight_ratio.csv")

largest_value_n = largest_value_first["N"].values
largest_value_avg_val = largest_value_first["Average Value"].values
largest_value_avg_time = largest_value_first["Average Time"].values

smallest_weight_n = smallest_weight_first["N"].values
smallest_weight_avg_val = smallest_weight_first["Average Value"].values
smallest_weight_avg_time = smallest_weight_first["Average Time"].values

worth_ratio_n = greatest_worth_ratio["N"].values
worth_ratio_avg_val = greatest_worth_ratio["Average Value"].values
worth_ratio_avg_time = greatest_worth_ratio["Average Time"].values

# Create DataFrames
df_largest_value_n = pd.DataFrame(largest_value_n)
df_largest_value_avg_val = pd.DataFrame(largest_value_avg_val)
df_largest_value_avg_time = pd.DataFrame(largest_value_avg_time)

df_smallest_weight_n = pd.DataFrame(smallest_weight_n)
df_smallest_weight_avg_val = pd.DataFrame(smallest_weight_avg_val)
df_smallest_weight_avg_time = pd.DataFrame(smallest_weight_avg_time)

df_worth_ratio_n = pd.DataFrame(worth_ratio_n)
df_worth_ratio_avg_val = pd.DataFrame(worth_ratio_avg_val)
df_worth_ratio_avg_time = pd.DataFrame(worth_ratio_avg_time)

# Create figure and axis objects
plt.figure(figsize=(10, 6))

# Plot for Average Time Comparison
plt.plot(
    df_largest_value_n,
    df_largest_value_avg_time,
    label="Largest Value First",
    linewidth=1,
    marker="o",
    markersize=2.5,
    color="red",
)

plt.plot(
    df_smallest_weight_n,
    df_smallest_weight_avg_time,
    label="Smallest Weight First",
    linewidth=1,
    marker="o",
    markersize=2.5,
    color="blue",
)

plt.plot(
    df_worth_ratio_n,
    df_worth_ratio_avg_time,
    label="Greatest Worth Ratio",
    linewidth=1,
    marker="o",
    markersize=2.5,
    color="green",
)

plt.title("Comparison of Greedy Algorithms")
plt.xlabel("Input Size (n)")
plt.ylabel("Average Time (seconds)")
plt.grid(True)
plt.legend()
plt.tight_layout()
plt.show()

# Plot for Average Value Comparison
fig, axs = plt.subplots(1, 2, figsize=(10, 6), sharex=True, sharey=True)

axs[0].plot(
    df_largest_value_n,
    df_largest_value_avg_val,
    label="Largest Value First",
    linewidth=1,
    marker="o",
    markersize=2.5,
    color="red",
)
axs[0].set_title("Largest Value First")
axs[0].set_xlabel("Input Size (n)")
axs[0].set_ylabel("Average Value")
axs[0].grid(True)

axs[1].plot(
    df_smallest_weight_n,
    df_smallest_weight_avg_val,
    label="Smallest Weight First",
    linewidth=1,
    marker="o",
    markersize=2.5,
    color="blue",
)
axs[1].set_title("Smallest Weight First")
axs[1].set_xlabel("Input Size (n)")
axs[1].grid(True)

fig.suptitle("Comparison of Greedy Algorithms", fontsize=16)
plt.tight_layout()
plt.show()

plt.plot(
    df_worth_ratio_n,
    df_worth_ratio_avg_val,
    label="Greatest Worth Ratio",
    linewidth=1,
    marker="o",
    markersize=2.5,
    color="green",
)
plt.tight_layout()
plt.show()
