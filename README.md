# Tetrus

A cross-platform terminal based Tetris game (inspired by [tetr.io](https://tetr.io/)) built in Rust using Ratatui.

## Demo

![Demo Video](./assets/demo.gif)

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [VHS](https://github.com/charmbracelet/vhs) (optional, for recording demo GIFs)

## How to Run

```bash
git clone https://github.com/JoseIgnacioGC/tetrus.git
cd tetrus
cargo run --release
```

## Record Demo (VHS)

To render the deterministic demo GIF:

```bash
cargo build --release --features vhs
vhs demo.tape
```
