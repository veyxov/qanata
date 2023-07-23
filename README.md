# Application aware layer switching 🔁 with kanata ⌨️ and sway 💨

## TODO 📔
- [ ] Configuration
    - [ ] List of registered apps that have specific layers
        - [x] Currently getting from file system (`~/.config/keyboard/apps/*`)
        - [ ] Get from config file?
    - [ ] Aliases and patterns for application names

- [ ] Refactor the code
- [ ] Don't rely on swaymsg, make universal
    - [x] Now using swayipc
    - [ ] Make it work on other than sway?
- [x] Whitelist/blacklist

## How it works ⚙️
1. You have a `main` layer, this will be used by default.
2. You have a `~/.config/keyboard/apps/org.telegram.desktop` layer.
    When `telegram` is focused it sends the window name (`org.telegram.desktop` in this case) to kanata.
    Kanata switches to this layer.
NOTE: If it can't find the corresponding file in `/apps` it fallbacks to the default layer (`main`)

- Create a file and include it in your kanata configuration
    - NOTE: The name of the file should match what sway returns

## Check out my [kanata config](https://github.com/veyxov/dots/tree/main/.config/keyboard) for reference 💡

## How to run 🏃
```sh
git clone https://github.com/veyxov/qanata
cd qanata/
cargo run -- --port 7070
```

## Bugs 🐞
- [x] ~~Panics if there are no windows in current workspace (when the wallpaper is visible)~~

# Caution: This is very experimental and raw. Needs a lot of work to make usable
