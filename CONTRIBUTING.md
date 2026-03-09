# Contributing

## Local workflow

1. Install Node.js, `pnpm`, and Rust.
2. Run `corepack pnpm install`.
3. Start the desktop app with `corepack pnpm tauri:dev`.
4. Run `corepack pnpm build` before opening a pull request.

## Pull requests

- Keep changes scoped to one feature or fix.
- Add tests for preset logic or command construction when behavior changes.
- Update `README.md` if supported formats or setup steps change.
- Do not commit third-party binaries into git.

## Releases

- Tag releases using `v*.*.*`.
- GitHub Actions will build the Windows installer and attach artifacts to the release.