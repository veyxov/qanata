# qanata

Application-aware layer switching for [kanata](https://github.com/jtroo/kanata) on [sway](https://github.com/swaywm/sway).

qanata watches the focused sway window and tells kanata which layer to use.

## How it works

1. `main` is the default layer.
2. If the focused application matches a file in `~/.config/keyboard/apps/`, qanata switches to that layer.
3. If no matching file exists, qanata falls back to `main`.

Example:

```text
~/.config/keyboard/apps/org.telegram.desktop
```

If Telegram is focused, qanata sends `org.telegram.desktop` to kanata and kanata switches to that layer.

## Run

```sh
cargo run -- --port 7070
```

Useful flags:

- `--debug` for debug logs
- `--trace` for trace logs
- `--white-list-file <path>` to ignore apps that should not trigger layer changes

## Overlay

The overlay window is optional and controlled by the `overlay` feature:

```sh
cargo run --features overlay -- --port 7070
```

## Nix development shell

This repository includes a [`flake.nix`](./flake.nix) for development dependencies.

1. Enable [Flakes](https://wiki.nixos.org/wiki/Flakes) in your Nix configuration.
2. Install [direnv](https://direnv.net/), for example with `nix profile install nixpkgs#direnv`.
3. Clone the repository and enter it.
4. Allow the environment with `direnv allow`.

After that, the shell provides the build and runtime dependencies used by the project.

## Statistics

The repository also includes a small script for visualizing kanata logs:

```sh
./run_stats.sh --file path/to/kanata/log
```

Log format:

```text
actual_key_presses|layer|resulting_key_press
```

## Reference

For a real configuration example, see my [kanata config](https://github.com/veyxov/dots/tree/main/keyboard/.config/keyboard).

## Status

This project is still experimental. It works, but it is not polished yet.
