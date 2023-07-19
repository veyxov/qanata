#!/bin/bash

swaymsg --raw -t get_tree | jq '.nodes[].nodes[].nodes[] | {name: .name, is_focused: .focused}'
