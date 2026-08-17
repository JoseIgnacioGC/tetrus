# Release Guide (Maintainer Only)

Instructions for creating releases, uploading binaries, and publishing packages for **Tetrus**.

---

## 1. Create a New GitHub Release

1. Update the `version` field in `Cargo.toml` (e.g., `version = "0.1.0"`).
2. Commit the version bump:
   ```bash
   git commit -am "chore: bump version to 0.1.0"
   git push origin main
   ```
3. Create and push a matching Git tag:
   ```bash
   git tag v0.1.0
   git push origin v0.1.0
   ```
4. GitHub Actions will automatically build, package, and attach the release binaries (`.zip` for Windows, `.tar.gz` for Linux) to the GitHub release.

---

## 2. Publish to Crates.io (for `cargo install` & `cargo binstall`)

Publish the package to [crates.io](https://crates.io):

```bash
cargo publish
```

---

## 3. Publish / Update on Windows Package Manager (Winget)

Use Microsoft's official `wingetcreate` CLI tool:

### Initial Package Submission
```powershell
winget install wingetcreate
wingetcreate new https://github.com/JoseIgnacioGC/tetrus/releases/download/v0.1.0/tetrus-v0.1.0-x86_64-pc-windows-msvc.zip
```

### Subsequent Version Updates
```powershell
wingetcreate update JoseIgnacioGC.tetrus -u https://github.com/JoseIgnacioGC/tetrus/releases/download/v0.2.0/tetrus-v0.2.0-x86_64-pc-windows-msvc.zip -v 0.2.0
```
This automatically calculates SHA256 hashes, generates the manifests, and opens the PR against `microsoft/winget-pkgs`.
