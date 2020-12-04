# Advent of Code 2020

The solutions are all separate binaries this time around, written in Rust. Assuming Rust is installed, you can compile and run a given day like so:

```
cargo run --bin day03 -- src/bin/day03_input.txt
```

My input lives alongside the binary source, and typically the binaries just need one arg - path to the input. Where that isn't true, they will tell you with help messages when you try running them!

To build all of the binaries (use `--bin dayXX` to just build one day) and put them in the local `./target/release` folder:

```
cargo build --release
```

Release mode (achieved with `--release`) can be significantly faster than the default debug mode, and may be required for some of the days to run in a reasonable time.

To install all binaries so that they are on your `$PATH` (and overwrite existing ones with `--force`):

```
cargo install --force --path .
```