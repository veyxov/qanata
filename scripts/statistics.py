# Keyboard heatmap and statistics
import matplotlib.pyplot as plt
import numpy as np
import matplotlib.cm as cm
import argparse
from matplotlib.widgets import Button

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

    # Step 3: Generate the heatmap with color-coded bars and a button to toggle layers.
    layers = list(sent_keys_by_layer.keys())
    current_layer_index = [0]  # Use a list to hold the current layer index

    def update_heatmap(event):
        current_layer_index[0] = (current_layer_index[0] + 1) % len(layers)  # Toggle between layers
        ax.clear()

        layer = layers[current_layer_index[0]]
        keys = list(sent_keys_by_layer[layer].keys())
        counts = list(sent_keys_by_layer[layer].values())

        # Normalize the counts for color coding
        cmap = cm.get_cmap('plasma')
        normalize = plt.Normalize(vmin=min(counts), vmax=max(counts))
        colors = [cmap(normalize(value)) for value in counts]

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

        # Rotate the x-axis labels for better readability
        ax.set_xticklabels(ax.get_xticklabels(), rotation=45, ha='right')
        plt.draw()

    fig, ax = plt.subplots(figsize=(12, 6))
    toggle_button_ax = plt.axes([0.8, 0.01, 0.1, 0.05])
    toggle_button = Button(toggle_button_ax, 'Toggle Layer')
    toggle_button.on_clicked(update_heatmap)
    plt.subplots_adjust(bottom=0.1)

    update_heatmap(None)  # Show the initial heatmap

    plt.show()

else:
    print("No file provided\nPlease specify --file")
