# Contributing to Takumi

Thank you for your interest in contributing to Takumi! This document explains how to get started and what we expect from contributions.

## Code of Conduct

This project follows the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## Getting Started

1. Fork the repository on GitHub.
2. Clone your fork locally:

   ```bash
   git clone https://github.com/<your-username>/Takumi.git
   cd Takumi
   ```

3. Create a branch for your changes:

   ```bash
   git checkout -b feature/my-change
   ```

## Development Setup

Takumi is a Rust workspace. Make sure you have a recent stable Rust toolchain installed via [rustup](https://rustup.rs/).

```bash
# Verify the project builds
cargo build

# Run the proxy locally
cargo run
```

## Making Changes

### Scope

- Keep pull requests focused on a single concern.
- Match the existing code style and conventions in the file you are editing.
- Avoid unrelated refactors or drive-by changes.

### Code Quality

Before submitting a pull request, run:

```bash
# Format code
cargo fmt --all

# Run the linter
cargo clippy --all-targets --all-features -- -D warnings

# Run tests
cargo test --all
```

Fix any warnings or test failures before opening your pull request.

### Commit Messages

Write clear, concise commit messages that explain **why** the change was made:

```
fix: handle disconnect during login handshake

Prevent the connection from hanging when the client closes
the socket before authentication completes.
```

Use imperative mood ("add feature" not "added feature") and keep the subject line under 72 characters when possible.

## Submitting a Pull Request

1. Push your branch to your fork.
2. Open a pull request against the `master` branch of [AxenoDev/Takumi](https://github.com/AxenoDev/Takumi).
3. Fill in the pull request description:
   - What problem does this solve?
   - How was it tested?
   - Are there any breaking changes?
4. Link any related issues (e.g. `Fixes #42`).

Maintainers will review your pull request and may request changes. Once approved, your contribution will be merged.

## Reporting Bugs

Open a [GitHub issue](https://github.com/AxenoDev/Takumi/issues/new) and include:

- A clear description of the problem
- Steps to reproduce
- Expected vs. actual behavior
- Your Rust toolchain version (`rustc --version`)
- Relevant logs or error messages

## Suggesting Features

Feature requests are welcome! Open an issue describing the use case and why it would benefit Takumi users. Discussing the idea before implementing helps avoid wasted effort.

## Questions

If you have questions, open a GitHub issue with the `question` label or reach out through the issue tracker.

Thank you for helping make Takumi better!
