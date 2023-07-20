# Sent current window to kanata
## Usefull for application specific layers

## TODO
- [ ] Configuration
    - [ ] List of registered apps that have specific layers
        - [x] Currently getting from file system (`~/.config/keyboard/apps/*`)
        - [ ] Get from config file?
    - [ ] Aliases and patterns for application names

- [ ] Refactor the code
- [ ] Don't rely on swaymsg, make universal
    - [x] Now using swayipc
    - [ ] Make it work on other than sway?

## How it works
- Create a file and include it in your kanata configuration
    - NOTE: The name of the file should match what sway returns

## How to run
```sh
git clone https://github.com/veyxov/qanata
cd qanata/
cargo run -- --port 7070
```

## Bugs
- [ ] Panics if there are no windows in current workspace (when the wallpaper is visible)

