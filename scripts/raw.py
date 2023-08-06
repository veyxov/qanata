import shutil

def parse_data(line):
    parts = line.strip().split("|")
    if len(parts) == 3:

        layer = parts[1]
        # adaptive layers should be merged with main
        if layer.startswith("adaptive"):
            layer = "main"

        return {
            "actual_key": parts[0],
            "layer": layer,
            "resulting_key": parts[2],
        }
    else:
        return None

def calculate_statistics(data):
    # Count the occurrences of each actual key
    key_count = {}
    for item in data:
        actual_key = item["actual_key"]
        key_count[actual_key] = key_count.get(actual_key, 0) + 1

    # Count the occurrences of each resulting key
    resulting_key_count = {}
    for item in data:
        resulting_key = item["resulting_key"]
        resulting_key_count[resulting_key] = resulting_key_count.get(resulting_key, 0) + 1

    # Count the occurrences of each layer
    layer_count = {}
    for item in data:
        layer = item["layer"]
        layer_count[layer] = layer_count.get(layer, 0) + 1

    return key_count, resulting_key_count, layer_count

def main():
    file_path = "/home/iz/kanata.log"  # Replace with the path to your file
    backup_file_path = "/home/iz/kanata.log.backup"

    # Make a backup of the original file
    shutil.copyfile(file_path, backup_file_path)

    # Read the data from the file and parse it
    with open(file_path, "r") as file:
        lines = file.readlines()

    data = [parse_data(line) for line in lines]
    data = [item for item in data if item is not None]

    key_count, resulting_key_count, layer_count = calculate_statistics(data)

    # Overwrite the original file with the cleaned data
    with open(file_path, "w") as file:
        for item in data:
            file.write(f"{item['actual_key']}|{item['layer']}|{item['resulting_key']}\n")

    print("Key counts:")
    for key, count in key_count.items():
        print(f"{key}: {count}")

    print("\nResulting key counts:")
    for key, count in resulting_key_count.items():
        print(f"{key}: {count}")

    print("\nLayer counts:")
    for layer, count in layer_count.items():
        print(f"{layer}: {count}")

if __name__ == "__main__":
    main()
