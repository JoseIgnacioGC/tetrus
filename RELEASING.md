# Release Guide (Maintainer Only)

Instructions for creating releases, uploading binaries, and publishing packages for **Tetrus**.

## The 1-Command Automated Release (Recommended)

Install `cargo-release` (one-time setup):

```bash
cargo install cargo-release
```

### Release a New Version:

```bash
# For bug fixes:
cargo release patch --execute

# For new features:
cargo release minor --execute
```
