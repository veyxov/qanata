# Keyboard heatmap and statistics
#
# NOTE: please forgive me, I'm not a python dev 😁
# oooooh, I'm a python dev now 😎

import matplotlib.pyplot as plt
import numpy as np

# Step 1: Save the provided data into a text file named "key_events.txt".

# Step 2: Read data from the file and process it.
filename = "/home/iz/.local/bin/kanata/log"

# Dictionary to store the count of each sent key
sent_keys_count = {}

with open(filename, "r") as file:
    lines = file.readlines()
    for line in lines:
        if "SENT=" in line:
            sent_key = line.split("SENT=")[1].strip()
            sent_key = sent_key.replace("KEY_", "")  # Remove the "KEY_" prefix
            sent_keys_count[sent_key] = sent_keys_count.get(sent_key, 0) + 1

# Step 3: Generate the heatmap.
keys = list(sent_keys_count.keys())
counts = list(sent_keys_count.values())

fig, ax = plt.subplots(figsize=(10, 6))
heatmap = ax.bar(keys, counts, color='b')
ax.set_xlabel("Sent Keys")
ax.set_ylabel("Count")
ax.set_title("Sent Keys Heatmap")

# Annotate each bar with its count value.
for bar in heatmap:
    height = bar.get_height()
    ax.annotate('{}'.format(height),
                xy=(bar.get_x() + bar.get_width() / 2, height),
                xytext=(0, 3),
                textcoords="offset points",
                ha='center', va='bottom')

plt.tight_layout()
plt.show()
