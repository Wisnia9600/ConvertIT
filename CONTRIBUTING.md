# Contributing

## Local workflow

1. Install Rust and Visual Studio Build Tools with the C++ workload.
2. Stage vendor tools with `./scripts/fetch-tools.ps1 -Destination "./vendor/bin"`.
3. Run `cargo test --manifest-path src-tauri/Cargo.toml`.
4. Run `cargo build --release --manifest-path src-tauri/Cargo.toml` before opening a pull request.

## Pull requests

- Keep changes scoped to one feature or fix.
- Add tests for preset logic or command construction when behavior changes.
- Update `README.md` if supported formats or setup steps change.
- Do not commit third-party binaries into git.

## Releases

- Tag releases using `v*.*.*`.
- GitHub Actions will build the Windows zip package and attach artifacts to the release.
