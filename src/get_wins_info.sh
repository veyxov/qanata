#!/bin/bash

# Requires: swaymsg, jq
# Note: now using swayipc so this is not needed; usefull for debugging purposes
swaymsg --raw -t get_tree | jq '.nodes[].nodes[].nodes[] | {name: .name, is_focused: .focused}'
