# terra-rs

Just a Terraria character editor written in Rust to learn Rust.

## Usage

Download the [latest release](https://github.com/RLGingerBiscuit/terra-rs/releases/latest) and extract it using your preferred tool (I use [7-zip](https://www.7-zip.org/)).

Run `terra-rs.exe` (or `terra-rs` on Linux).

## Building from source

- Install Cargo via <https://rustup.rs> (or from your preferred package manager).
- Clone the repo and run `cargo build` to build a debug executable.
- Copy `data/resources` into `target/debug`.
  - Alternatively, extract Terraria's assets (I use [TConvert](https://github.com/trigger-segfault/TConvert), which is Windows-only, but may work with Wine).
  - Place all images starting with `Item` in to `terra-res/resources/items`.
  - Place all images starting with `Buff` in to `terra-res/resources/buffs`.
  - Place all images starting with `Extra` in to `terra-res/resources/other` (technically you only need `Extra_54.png` at this time).
  - Run `cargo run -p terra-res` to generate the required resources.
- Finally, run `cargo run` to run the final application.
