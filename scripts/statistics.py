# Keyboard heatmap and statistics
import matplotlib.pyplot as plt
import numpy as np
import matplotlib.cm as cm
import argparse
from matplotlib.widgets import CheckButtons

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

    # Step 3: Generate the heatmap with color-coded bars and checkboxes for each layer.
        layers = list(sent_keys_by_layer.keys())

    fig, ax = plt.subplots(figsize=(12, 6))
    plt.subplots_adjust(bottom=0.3)

    # Create a dictionary to map the checkbox objects to their corresponding layer visibility
    checkbox_dict = {}
    visibility_dict = {}  # Dictionary to store the visibility state for each layer
    for idx, layer in enumerate(layers):
        visibility_dict[layer] = False  # Set all layers to be unchecked by default
        checkbox = CheckButtons(plt.axes([0.1 + (idx % 6) * 0.15, 0.9 - (idx // 6) * 0.1, 0.1, 0.05]), [layer], [False])
        checkbox.on_clicked(lambda event, layer=layer: update_visibility(event, layer))
        checkbox_dict[layer] = checkbox

    cmap = cm.get_cmap('plasma')
    bars_dict = {}  # Dictionary to store references to the bars (Rectangles) for each layer
    for layer, data in sent_keys_by_layer.items():
        keys = list(data.keys())
        counts = list(data.values())

        # Sort the keys and counts in descending order of counts
        sorted_indices = np.argsort(counts)[::-1]
        keys = np.array(keys)[sorted_indices]
        counts = np.array(counts)[sorted_indices]

        normalize = plt.Normalize(vmin=min(counts), vmax=max(counts))
        colors = [cmap(normalize(value)) for value in counts]

        bars = ax.bar(keys, counts, color=colors, alpha=0.5)  # Use colors for bars (with alpha for better visibility)
        bars_dict[layer] = bars  # Store the bars for this layer in the dictionary

        for bar in bars:
            height = bar.get_height()
            ax.annotate('{}'.format(height),
                        xy=(bar.get_x() + bar.get_width() / 2, height),
                        xytext=(0, 3),
                        textcoords="offset points",
                        ha='center', va='bottom')

    def update_visibility(event, layer):
        visibility_dict[layer] = not visibility_dict[layer]  # Toggle visibility state

        for bar in bars_dict[layer]:
            bar.set_visible(visibility_dict[layer])

        for annotation in ax.texts:
            if layer in annotation.get_text():
                annotation.set_visible(visibility_dict[layer])

        # Adjust x-axis limits to accommodate the visible bars
        visible_layers = [layer for layer, visible in visibility_dict.items() if visible]
        keys = [key for layer in visible_layers for key in sent_keys_by_layer[layer].keys()]
        ax.set_xlim(keys[0], keys[-1])

        plt.draw()

    ax.set_ylabel("Count")
    ax.set_title("Sent Keys Heatmap (Color-coded)")
    ax.set_xticklabels(ax.get_xticklabels(), rotation=45, ha='right')

    plt.show()

else:
    print("No file provided\nPlease specify --file")
