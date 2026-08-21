# Contributing to Tetrus

## How to Run

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)

```bash
git clone https://github.com/JoseIgnacioGC/tetrus.git
cd tetrus
cargo run --release
```

## Record Demo (VHS)

### Prerequisites

- [VHS](https://github.com/charmbracelet/vhs)

```bash
vhs demo.tape
```

## How to Contribute

1. Fork the repository and create a new feature branch (`git checkout -b feature/my-feature`).
1. Make your changes and verify code compiles cleanly (`cargo check`).
1. Record a demo if your feature changes gameplay/UI (`vhs demo.tape` it only works on linux).
1. Commit your changes following conventional commit messages.
1. Push your branch and open a Pull Request.

## AI Coding Assistants

If you are using an AI coding assistant (such as GitHub Copilot, Cursor, Claude, Antigravity, etc.), please refer to [AGENTS.md](AGENTS.md) for project architecture context, domain rules, and development guidelines.

