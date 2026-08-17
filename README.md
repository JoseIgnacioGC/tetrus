# Tetrus

A cross-platform terminal based Tetris game (inspired by [tetr.io](https://tetr.io/)) built in Rust using Ratatui.

## Demo

![Demo Video](./assets/demo.gif)

## Installation

### Windows (Winget)

```powershell
winget install JoseIgnacioGC.tetrus
```

### Linux & Cross-Platform (Cargo Binstall / Cargo)

Install the pre-compiled binary instantly with [cargo-binstall](https://github.com/cargo-bins/cargo-binstall):

```bash
cargo binstall tetrus
```

Or build and install from source:

```bash
cargo install tetrus
```

## How to Run

Once installed, simply type:

```bash
tetrus
```

## Updating

- **Windows (Winget)**:
  ```powershell
  winget upgrade JoseIgnacioGC.tetrus
  ```
- **Cargo Binstall / Cargo**:
  ```bash
  cargo binstall tetrus --force
  # or
  cargo install tetrus --force
  ```

## Contributing

Contributions are welcome! Please check out [CONTRIBUTING.md](CONTRIBUTING.md) for development setup, running tests, and guidelines on how to contribute.

## License

This project is licensed under the [MIT License](LICENSE).
