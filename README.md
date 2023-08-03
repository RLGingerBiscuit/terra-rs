# terra-rs

Just a Terraria character editor written in Rust to learn Rust.

## Usage

Download the latest build from [Github Actions](https://github.com/RLGingerBiscuit/terra-rs/actions) and extract it using your preferred tool (I use [7-zip](https://www.7-zip.org/)).

Run `terra-rs.exe` (or `terra-rs` on Linux).

## Building from source

- Install Cargo via <https://rustup.rs> (or from your preferred package manager).
- Clone the repo, and use `git submodule update --init --recursive` to clone the required submodules.
- Extract Terraria's assets (I use [TConvert](https://github.com/trigger-segfault/TConvert), which is Windows-only).
- Place all images starting with `Item` in to `terra-res/resources/items`.
- Place all images starting with `Buff` in to `terra-res/resources/buffs`.
- Run `cargo run -p terra-res` to generate the required resources.
- Finally, run `cargo run` to run the final application.
