# Encephalon Destroyer
Encephalon Destroyer is a BrainFuck VM written in just 4 days.

## Features:
- [x] Customizable array size
- [x] Cell wrapping
- [x] Error reporting (for loops at least, does not support line numbers)
- [x] Nice error and info messages

## Building

To build Encephalon Destroyer, clone this repo and then build it with `cargo build --release`:

```bash
git clone https://www.github.com/JhonnyRice/encephalon-destroyer
cd encephalon-destroyer
cargo build --release
```
## Installing
To install Encephalon Destroyer on Linux or MacOS use the provided Makefile:
```bash
sudo make install
```
To install Encephalon Destroyer on Windows, move manually the `ed` binary located in `target/release/ed` into one of the directories listed in the `%PATH%` variable.

Yes i know, i could've used `cargo install` but i dont know how to do cross-compatibility, and yes i know that is not the way to install windows binaries but i dont know how windows works, so...

## Usage
```
encephalon-destroyer [PATH_TO_FILE]
encephalon-destroyer -h | --help
encephalon-destroyer -u | --usage
encephalon-destroyer -v | --version
encephalon-destroyer -i | --info
encephalon-destroyer -A | --array [NUMBER] [PATH_TO_FILE]
encephalon-destroyer [PATH_TO_FILE] -A | --array [NUMBER]
```
## Inspiration
My main inspiration was [BF by Alexander Overvoorde](https://github.com/Overv/bf). The code looks identical, but the only thing that i really copied were the Loop implementation and the `run()` function args. Other than that is all made by me.

## Have Fun!
Thanks for checking my little project! If you have any idea or if you want to implement a new feature, just make a PR!