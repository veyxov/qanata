# Application aware layer switching 🔁 with [kanata](https://github.com/jtroo/kanata/) ⌨️ and [sway](https://github.com/swaywm/sway) 💨

## Additional features:
### Current layer text overlay
- Using sdl2 for rendering just text, is overkill
- [ ] Find a more lightweight alternative
- Is there any better way of showing current-layer information? 
- [ ] Maybe some sway prompt using ipc?

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
- [ ] Make the overlay a compile time feature
- [ ] Fork kanata and apply the logging patches

## How it works ⚙️
1. You have a `main` layer, this will be used by default.
2. You have a `~/.config/keyboard/apps/org.telegram.desktop` layer.
    When `telegram` is focused it sends the window name (`org.telegram.desktop` in this case) to kanata.
    Kanata switches to this layer.
NOTE: If it can't find the corresponding file in `/apps` it fallbacks to the default layer (`main`)

- Create a file and include it in your kanata configuration
    - NOTE: The name of the file should match what sway returns

## Check out my [kanata config](https://github.com/veyxov/dots/tree/main/keyboard/.config/keyboard) for reference 💡

# Development
## Using Cargo
Dependencies:
- SDL2 for the overlay window, optional (todo)

```sh
git clone https://github.com/veyxov/qanata
cd qanata/
cargo run -- --port 7070
```

## Using Nix
You can use the provided [`flake.nix`](./flake.nix) to setup all build- and run-time dependencies:

1. Enable [Flakes](https://wiki.nixos.org/wiki/Flakes) in your Nix configuration.
1. Add [`direnv`](https://direnv.net/) to your profile:
   `nix profile install nixpkgs#direnv`
1. `cd` into the directory where you have cloned this directory.
1. Allow use of [`.envrc`](./.envrc): `direnv allow`
1. After the dependencies have been installed, your shell will now have all of the necessary development dependencies.

# Visualize statistics generated from kanata
#### python script for generating heatmap for key presses
### How to run:
```sh
./run_stats.sh --file path/to/kanata/log

# Current format: actual_key_presses|layer|resulting_key_press
```
---

## Bugs 🐞
- [x] ~~Panics if there are no windows in current workspace (when the wallpaper is visible)~~
- [x] Overheats the CPU when sway is locked; Fixed in [commit](https://github.com/veyxov/qanata/commit/e8ae9d1e51606bab5a3d8a57bb97eab2cb01de1b)

# Caution: This is very experimental and raw. Needs a lot of work to make usable
