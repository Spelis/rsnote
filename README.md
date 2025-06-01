# RSNote - Simple Note taking app in Rust

Very simple Command Line Interface to create, list, edit, and search through notes.

## Building:

`cargo build` to build with debug (add `--release` for a release build)

then run with `./target/release or debug/rsnote`

or:

`cargo run -- <args>` to run a debug build quickly

or:

`make all` uses the Makefile to cross compile the app across different platforms

# 🔧 Requirements Per Target (Linux)
|Platform|Tool Required        |Install Command (example)|
|Linux   |Native Rust toolchain|Already installed if Rust is set up|
|macOS	 |`osxcross` w/ macOS SDK|`osxcross` + SDK|
|Windows |`mingw-w64`            |`sudo pacman -S mingw-w64-gcc` or `apt` equiv|
