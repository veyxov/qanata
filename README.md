# Sent current window to kanata
## Usefull for application specific layers

## TODO
- [ ] Configuration
    - [ ] List of registered apps that have specific layers
    - [ ] Aliases and patterns for application names

- [ ] Refactor the code
- [ ] Don't rely on swaymsg, make universal
    - [x] Now using swayipc

## How it works
- Create a file and include it in your kanata configuration
    - NOTE: The name of the file should match what sway returns

## Bugs
- [ ] Panics if there are no windows in current workspace (when the wallpaper is visible)
