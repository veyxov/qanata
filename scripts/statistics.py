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
    # Dictionary to store the count of each sent key and its layer
    sent_keys_by_layer = {}

    with open(file_name, "r") as file:
        lines = file.readlines()
        for line in lines:
            try:
                sp = line.split("|")
                actual_key = sp[0]
                layer = sp[1]
                sent_key = sp[2].strip()

                sent_key = sent_key.replace("KEY_", "")  # Remove the "KEY_" prefix
                combined_key = f"{sent_key} ({layer})"  # Include layer in the key label

                if layer not in sent_keys_by_layer:
                    sent_keys_by_layer[layer] = {}

                sent_keys_by_layer[layer][combined_key] = sent_keys_by_layer[layer].get(combined_key, 0) + 1
            except:
                print("Error while processing: " + line)

    # Step 3: Generate separate heatmaps with color-coded bars for each layer.
    cmap = cm.get_cmap('plasma')
    fig, axs = plt.subplots(len(sent_keys_by_layer), 1, figsize=(12, 6 * len(sent_keys_by_layer)), sharex=True)
    fig.subplots_adjust(hspace=0.5)

    for idx, layer in enumerate(sent_keys_by_layer):
        keys = list(sent_keys_by_layer[layer].keys())
        counts = list(sent_keys_by_layer[layer].values())

        # Normalize the counts for color coding
        normalize = plt.Normalize(vmin=min(counts), vmax=max(counts))
        colors = [cmap(normalize(value)) for value in counts]

        ax = axs[idx]
        heatmap = ax.bar(keys, counts, color=colors)  # Use colors for bars
        ax.set_ylabel("Count")
        ax.set_title(f"Sent Keys Heatmap for Layer {layer} (Color-coded)")

        # Annotate each bar with its count value.
        for bar in heatmap:
            height = bar.get_height()
            ax.annotate('{}'.format(height),
                        xy=(bar.get_x() + bar.get_width() / 2, height),
                        xytext=(0, 3),
                        textcoords="offset points",
                        ha='center', va='bottom')

    # Rotate the x-axis labels for better readability in the last subplot
    axs[-1].set_xticklabels(axs[-1].get_xticklabels(), rotation=45, ha='right')

    plt.tight_layout()
    plt.show()
else:
    print("No file provided\nPlease specify --file")

