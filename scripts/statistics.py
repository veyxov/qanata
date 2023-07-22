# Keyboard heatmap and statistics
#
# NOTE: please forgive me, I'm not a python dev 😁
# oooooh, I'm a python dev now 😎

import re
import pandas as pd

# Function to extract key code and event type from the log data
def parse_log(log_data):
    keys = []
    events = []
    pattern = r"code:\s(KEY_\w+),\svalue:\s(\w+)"
    matches = re.findall(pattern, log_data)
    for match in matches:
        keys.append(match[0])
        events.append(match[1])
    return keys, events

# Step 1: Read the log data from the file
file_path = "/home/iz/.local/bin/kanata/log"
with open(file_path, 'r') as file:
    log_data = file.read()

# Step 2: Parse the log data into DataFrame
keys, events = parse_log(log_data)
df = pd.DataFrame({"Key": keys, "Event": events})

import matplotlib.pyplot as plt
import seaborn as sns

# Create a pivot table to count key press frequency
heatmap_data = df.pivot_table(index='Event', columns='Key', aggfunc='size', fill_value=0)

# Sort the columns by frequency to arrange the heatmap
heatmap_data = heatmap_data[heatmap_data.sum().sort_values(ascending=False).index]

# Create the heatmap
plt.figure(figsize=(12, 8))
sns.heatmap(heatmap_data, cmap='YlGnBu', annot=True, fmt='d', cbar=False)
plt.title('Keyboard Heatmap - Key Press Frequency')
plt.xlabel('Key')
plt.ylabel('Event')
plt.show()
