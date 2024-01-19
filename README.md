# About

My first attempt at making a game. It'll be some sort of top down rpg dungeon crawler. It's written in Rust and uses SDL for rendering.

# Installation

Make sure that SDL2 is installed in your system. If you're on macos it's just

```bash
brew install sdl2
```

and also make sure that your library path in your shell config contains the SDL library.

```bash
export LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix)
```

### Running

```bash
cargo run
```

### Controls

Arrow keys to move, space to use equipped item. But there is currently no gameplay other than moving the character.

# Build

```bash
cargo build --release
```
