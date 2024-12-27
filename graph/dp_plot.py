import matplotlib.pyplot as plt
import pandas as pd

# Data for DP Algorithms
bottom_up = pd.read_csv("results/bottom_up.csv")
top_down = pd.read_csv("results/top_down.csv")

bottom_up_n = bottom_up["N"].values
bottom_up_avg_val = bottom_up["Average"].values
bottom_up_avg_time = bottom_up["Average Time"].values

top_down_n = top_down["N"].values
top_down_avg_val = top_down["Average"].values
top_down_avg_time = top_down["Average Time"].values

# Create DataFrames
df_bottom_up_n = pd.DataFrame(bottom_up_n)
df_bottom_up_avg_val = pd.DataFrame(bottom_up_avg_val)
df_bottom_up_avg_time = pd.DataFrame(bottom_up_avg_time)

df_top_down_n = pd.DataFrame(top_down_n)
df_top_down_avg_val = pd.DataFrame(top_down_avg_val)
df_top_down_avg_time = pd.DataFrame(top_down_avg_time)


# Create figure and axis objects
plt.figure(figsize=(10, 6))

# Plot for Average Time Comparison
plt.plot(
    df_bottom_up_n,
    df_bottom_up_avg_time,
    label="DP Bottom Up (Tabulation)",
    linewidth=1,
    marker="o",
    markersize=2.5,
    color="red",
)

plt.plot(
    df_top_down_n,
    df_top_down_avg_time,
    label="DP Top Down (Memoization)",
    linewidth=1,
    marker="o",
    markersize=2.5,
    color="blue",
)

plt.title("Comparison of Dynamic Programming Bottom Up vs Top Down")
plt.xlabel("N")
plt.ylabel("Average Time (s)")
plt.grid(True)
plt.legend()
plt.tight_layout()
plt.show()
