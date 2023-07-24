# Keyboard heatmap and statistics

import matplotlib.pyplot as plt
import numpy as np
import matplotlib.cm as cm
import argparse

# Step 1: Save the provided data into a text file named "key_events.txt".

# Step 2: Read data from the file and process it.
parser = argparse.ArgumentParser()
parser.add_argument("--file", help="Specify the file path")
args = parser.parse_args()
if args.file:
    file_name = args.file
    # Now you can use the 'file_name' variable in your code.
    # Dictionary to store the count of each sent key
    sent_keys_count = {}

    with open(file_name, "r") as file:
        lines = file.readlines()
        for line in lines:
            try:
                sp = line.split("|")
                actual_key = sp[0]
                layer = sp[1]
                sent_key = sp[2].strip()

                print(sent_key)
                sent_key = sent_key.replace("KEY_", "")  # Remove the "KEY_" prefix
                sent_keys_count[sent_key] = sent_keys_count.get(sent_key, 0) + 1
            except:
                print("Error while processing: " + line)

    # Step 3: Generate the heatmap with color-coded bars.
    keys = list(sent_keys_count.keys())
    counts = list(sent_keys_count.values())

    # Get the color map and normalize the counts for color coding
    cmap = cm.get_cmap('plasma')
    normalize = plt.Normalize(vmin=min(counts), vmax=max(counts))
    colors = [cmap(normalize(value)) for value in counts]

    fig, ax = plt.subplots(figsize=(12, 6))  # Adjust the figure size
    heatmap = ax.bar(keys, counts, color=colors)  # Use colors for bars
    ax.set_xlabel("Sent Keys")
    ax.set_ylabel("Count")
    ax.set_title("Sent Keys Heatmap (Color-coded)")

    # Rotate the x-axis labels for better readability
    plt.xticks(rotation=45, ha='right')

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
else:
    print("No file provided\nPlease specify --file")
