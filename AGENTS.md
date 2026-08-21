# AGENTS.md - Context & Architectural Guidelines for AI Coding Assistants

This document provides essential architectural context, domain patterns, and development guidelines for AI coding assistants contributing to **Tetrus**.

---

## 1. Project Overview & Architecture

Tetrus is a cross-platform, terminal-based block puzzle game written in Rust using [Ratatui](https://github.com/ratatui/ratatui) and [Crossterm](https://github.com/crossterm-rs/crossterm).

### Core Components Structure:

- `src/main.rs`: Pure binary entrypoint declaring internal modules and initializing the game loop.
- `src/blocks.rs`: Block representations (`Block` enum, rotation states, bounding dimensions, theme colors) and SRS (Super Rotation System) coordinate transformations.
- `src/blocks_manager.rs`: `BlocksManager` abstraction implementing the 7-Bag randomizer to generate piece sequences, peek upcoming previews, and support deterministic RNG seeding (strictly used for testing and VHS demo recording).
- `src/board.rs`: Encapsulated game board state and business logic ($10 \times 22$ matrix, active falling piece, hold mechanics, lock delay, line clears, combo tracking, and scoring).
- `src/tui/`: Ratatui UI layer and rendering widgets:
  - `board_widget.rs`: Main game canvas widget handling 60 FPS tick cycle, inputs, board rendering, and particle effects.
  - `movement_widget.rs`: Stacked notification displays for line clear actions, T-Spins, B2B multipliers, and combos.
  - `metrics_widget.rs`: Displays score, level, line count, and active game timer.
  - `held_block_widget.rs`: Renders the hold piece preview box.
  - `next_blocks_widget.rs`: Renders the upcoming piece queue preview.
  - `debug_widget.rs`: Optional debug telemetry overlay.
- `src/constants.rs`: Centralized game balance timings (DAS, ARR, lock delay, notification durations, etc).
- `src/colors.rs`: Theme color palette and styling constants.
- `src/scores.rs`: High scores persistence, qualification logic.
- `src/utils/`: Common helpers and abstractions (such as integer formatting).

---

## 2. Critical Domain Rules & Patterns

1. **Ratatui Macros Convention**:
   - Prefer Ratatui's declarative layout and text macros (`vertical!`, `horizontal!`, `constraint!`, `line!`, `span!`, `text!`) over verbose builder patterns for UI layouts and text styling.
2. **No Direct Config File Edits for Dependencies**:
   - Follow standard Rust practices; always use (`cargo add`) rather than ad-hoc configuration mutations unless modifying release metadata.

---

## 3. Releases & Distribution Guidelines

- **Binary Distribution**: Multi-platform releases (Windows `.zip`, Linux `.tar.gz`) are built and published automatically by `.github/workflows/release.yml` on git tag push (`v*`).
- **Cargo Binstall Support**: Configured via `[package.metadata.binstall]` in `Cargo.toml`.
- **Winget Automation**: Managed via `winget-releaser` GitHub Action in CI.
- **Maintainer Release Workflow**: See `RELEASING.md` (`cargo release patch --execute`).
