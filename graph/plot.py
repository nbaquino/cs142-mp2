import matplotlib.pyplot as plt
import pandas as pd

# Data for Dynamic Programming Bottom Up (Tabulation)
dp_bottom_up_data = {
    "N": [0, 1000, 5000, 10000, 25000, 50000, 75000, 100000],
    "Average Value": [
        0,
        3467.33,
        4272.0,
        4393.67,
        4464.33,
        4482.0,
        4597.0,
        4766.67,
    ],
    "Average Time (s)": [
        0,
        0.055401,
        0.275933,
        0.607086,
        1.674860,
        3.100319,
        4.113612,
        5.322388,
    ],
}

# Data for Dynamic Programming Top Down (Memoization)
dp_top_down_data = {
    "N": [0, 1000, 5000, 10000, 25000, 50000, 75000, 100000],
    "Average Value": [
        0,
        3662.0,
        4224.33,
        4438.33,
        4465.67,
        4501.33,
        4709.33,
        4779.0,
    ],
    "Average Time (s)": [
        0,
        0.068912,
        0.488070,
        1.322124,
        3.062313,
        7.685362,
        9.468768,
        15.502679,
    ],
}

# Create DataFrames
df_dp_bottom_up = pd.DataFrame(dp_bottom_up_data)
df_dp_top_down = pd.DataFrame(dp_top_down_data)

# Create figure and axis objects
plt.figure(figsize=(10, 6))

# Plot for DP Bottom Up (Tabulation)
plt.plot(
    df_dp_bottom_up["N"],
    df_dp_bottom_up["Average Time (s)"],
    label="DP Bottom Up (Tabulation)",
    marker="o",
    color="blue",
)

# Plot for DP Top Down (Memoization)
plt.plot(
    df_dp_top_down["N"],
    df_dp_top_down["Average Time (s)"],
    label="DP Top Down (Memoization)",
    marker="o",
    color="red",
)

plt.title("Comparison of Dynamic Programming Bottom Up vs Top Down")
plt.xlabel("Input Size (n)")
plt.ylabel("Average Time (seconds)")
plt.grid(True)
plt.legend()
plt.tight_layout()
plt.show()


# Data for Greedy Algo
greedy_largest_value_data = {
    "N": [0, 1000, 5000, 10000, 25000, 50000, 75000, 100000],
    "Average Value": [0, 1658.33, 2155, 1831.67, 1166.67, 1166.67, 1166.67, 1666.67],
    "Average Time (s)": [
        0,
        0.0003817,
        0.0021263,
        0.0049836,
        0.0104713,
        0.0244677,
        0.0303481,
        0.0407343,
    ],
}
greedy_smallest_weight_data = {
    "N": [0, 1000, 5000, 10000, 25000, 50000, 75000, 100000],
    "Average Value": [0, 2628.67, 2830.33, 2843.33, 2965, 2879.33, 2883.67, 3267.33],
    "Average Time (s)": [
        0,
        0.0005565,
        0.0023681,
        0.0051399,
        0.0137738,
        0.0247884,
        0.0375170,
        0.0477979,
    ],
}
greedy_worth_ratio_data = {
    "N": [0, 1000, 5000, 10000, 25000, 50000, 75000, 100000],
    "Average Value": [0, 3536, 4182.67, 4242, 4362.67, 4427.67, 4417, 4430.67],
    "Average Time (s)": [
        0,
        0.0007265,
        0.0031461,
        0.0079791,
        0.0210178,
        0.0437989,
        0.0658092,
        0.0901923,
    ],
}

# Create DataFrames
df_greedy_largest_value = pd.DataFrame(greedy_largest_value_data)
df_greedy_smallest_weight = pd.DataFrame(greedy_smallest_weight_data)
df_greedy_worth_ratio = pd.DataFrame(greedy_worth_ratio_data)

# Create figure and axis objects
plt.figure(figsize=(10, 6))

# Plot data
plt.plot(
    df_greedy_largest_value["N"],
    df_greedy_largest_value["Average Time (s)"],
    label="Largest Value First",
    marker="o",
    color="blue",
)
plt.plot(
    df_greedy_smallest_weight["N"],
    df_greedy_smallest_weight["Average Time (s)"],
    label="Smallest Weight First",
    marker="o",
    color="red",
)
plt.plot(
    df_greedy_worth_ratio["N"],
    df_greedy_worth_ratio["Average Time (s)"],
    label="Greatest Worth Ratio",
    marker="o",
    color="green",
)

plt.title("Comparison of Greedy Algorithms")
plt.xlabel("Input Size (n)")
plt.ylabel("Average Time (seconds)")
plt.grid(True)
plt.legend()
plt.tight_layout()
plt.show()


# Create figure and axis objects
plt.figure(figsize=(10, 6))

# Plot for DP Bottom Up (Tabulation)
plt.plot(
    df_dp_bottom_up["N"],
    df_dp_bottom_up["Average Time (s)"],
    label="DP Bottom Up (Tabulation)",
    marker="o",
    color="blue",
)

# Plot for DP Top Down (Memoization)
plt.plot(
    df_dp_top_down["N"],
    df_dp_top_down["Average Time (s)"],
    label="DP Top Down (Memoization)",
    marker="o",
    color="red",
)

# Plot for Greedy Algorithms
plt.plot(
    df_greedy_largest_value["N"],
    df_greedy_largest_value["Average Time (s)"],
    label="Largest Value First",
    marker="o",
    color="green",
)
plt.plot(
    df_greedy_smallest_weight["N"],
    df_greedy_smallest_weight["Average Time (s)"],
    label="Smallest Weight First",
    marker="o",
    color="yellow",
)
plt.plot(
    df_greedy_worth_ratio["N"],
    df_greedy_worth_ratio["Average Time (s)"],
    label="Greatest Worth Ratio",
    marker="o",
    color="violet",
)

plt.title("Comparison of Running times of DP vs. Greedy Algorithms")
plt.xlabel("Input Size (n)")
plt.ylabel("Average Time (seconds)")
plt.grid(True)
plt.legend()
plt.tight_layout()
plt.show()


plt.figure(figsize=(10, 6))

# Plot for DP Bottom Up (Tabulation)
plt.plot(
    df_dp_bottom_up["N"],
    df_dp_bottom_up["Average Value"],
    label="DP Bottom Up (Tabulation)",
    marker="o",
    color="blue",
)

# Plot for DP Top Down (Memoization)
plt.plot(
    df_dp_top_down["N"],
    df_dp_top_down["Average Value"],
    label="DP Top Down (Memoization)",
    marker="o",
    color="red",
)

# Plot for Greedy Algorithms
plt.plot(
    df_greedy_largest_value["N"],
    df_greedy_largest_value["Average Value"],
    label="Largest Value First",
    marker="o",
    color="green",
)
plt.plot(
    df_greedy_smallest_weight["N"],
    df_greedy_smallest_weight["Average Value"],
    label="Smallest Weight First",
    marker="o",
    color="yellow",
)
plt.plot(
    df_greedy_worth_ratio["N"],
    df_greedy_worth_ratio["Average Value"],
    label="Greatest Worth Ratio",
    marker="o",
    color="violet",
)

plt.title("Comparison of Average Value of DP vs. Greedy Algorithms")
plt.xlabel("Input Size (n)")
plt.ylabel("Average Value")
plt.grid(True)
plt.legend()
plt.tight_layout()
plt.show()
