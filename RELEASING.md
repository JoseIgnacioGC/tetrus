# Release Guide (Maintainer Only)

Instructions for creating releases, building binary distributions, and publishing packages for **Tetrus**.

## Prerequisites

- [cargo-release](https://github.com/crate-ci/cargo-release)

## Release Process

### 1. Pre-Release Checks

1. Checks
   ```bash
   cargo check
   cargo check --release
   ```
2. Render new demo (only when UI/GamePlay changed):
   ```bash
   vhs demo.tape
   git add ...
   git commit -m "docs: update demo recording"
   ```

### 2. Version Bump & Tagging

Run `cargo release <flag> --execute`

This command automatically:

- Updates the version in `Cargo.toml` and `Cargo.lock`.
- Creates the release commit (`chore(release): bump version to <version>`).
- Creates the annotated git tag `v<version>`.
- Pushes commits and tags to GitHub (`git push origin main --tags`).

### 3. Automated CI / Distribution Pipeline

Once the `v*` tag is pushed, GitHub Actions ([release.yml](.github/workflows/release.yml)) automatically triggers:

1. **Multi-Platform Binary Builds**:
   - Linux GNU (`x86_64-unknown-linux-gnu` `.tar.gz`)
   - Linux Musl (`x86_64-unknown-linux-musl` `.tar.gz`)
   - Windows MSVC (`x86_64-pc-windows-msvc` `.zip`)
2. **GitHub Releases**:
   - Publishes the GitHub Release with attached binaries and SHA256 checksums.
   - Generates release notes automatically.
3. **Cargo Binstall**:
   - Immediately available to install via `cargo binstall tetrus`.
4. **Winget**:
   - Submits the new version manifest to the Windows Package Manager repository via `winget-releaser`.
