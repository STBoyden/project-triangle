# Triangular Tribulations

![Project Triangle (debug builds)](https://github.com/STBoyden/project-triangle/workflows/Project%20Triangle%20(debug%20builds)/badge.svg)
![Project Triangle (release builds)](https://github.com/STBoyden/project-triangle/workflows/Project%20Triangle%20(release%20builds)/badge.svg)

Developed in Rust, this prototype game/game engine was made for me to learn Rust using the game C library, 
Raylib (main: https://github.com/raysan5/raylib, Rust bindings: https://github.com/deltaphc/raylib-rs). 

The current debug and release builds can be found going [here](https://github.com/STBoyden/project-triangle/actions) and 
clicking on the most recent commit build. You can find the most recent debug builds for both Windows and Linux.

Please excuse any "bad Rust" as I am new to the language, but I am improving by working on this project, no matter how 
painful and annoying it can be working with Rust.


## Building and Running
If you haven't got Rust, install Rust [here](https://rustup.rs).

Once Rust is installed or if you already have it installed, clone this repository using `git` or download the zip file and simple run `./make.sh --release` to build the the release build. The binary for your OS should then be in `target/release/`.
