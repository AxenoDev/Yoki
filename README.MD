<div align="center">

# Takumi

**A high-performance Minecraft proxy server written in Rust**

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-2024%20edition-orange.svg)](https://www.rust-lang.org)
[![GitHub stars](https://img.shields.io/github/stars/AxenoDev/Takumi?style=social)](https://github.com/AxenoDev/Takumi/stargazers)
[![GitHub issues](https://img.shields.io/github/issues/AxenoDev/Takumi)](https://github.com/AxenoDev/Takumi/issues)
[![GitHub pull requests](https://img.shields.io/github/issues-pr/AxenoDev/Takumi)](https://github.com/AxenoDev/Takumi/pulls)

</div>

Takumi sits between players and your Minecraft backend servers. Clients connect to the proxy, which handles the Minecraft protocol and routes traffic to the appropriate server.

## Features

- Async I/O powered by [Tokio](https://tokio.rs/)
- Minecraft protocol handling (handshaking, status, login)
- Modular workspace with reusable crates:
  - `minecraft_packet` — packet parsing and connection management
  - `takumi_macros` — procedural macros for packet definitions

## Requirements

- [Rust](https://rustup.rs/) (2024 edition)

## Getting Started

Clone the repository and run the proxy:

```bash
git clone https://github.com/AxenoDev/Takumi.git
cd Takumi
cargo run
```

The proxy listens on **`0.0.0.0:25565`** by default.

## Building

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

The release binary is available at `target/release/takumi`.

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) before opening a pull request.

By participating in this project, you agree to abide by our [Code of Conduct](CODE_OF_CONDUCT.md).

## Security

If you discover a security vulnerability, please report it responsibly. See [SECURITY.md](SECURITY.md) for details.

## License

Takumi is licensed under the [GNU General Public License v3.0](LICENSE).

<p align="center">
  <sub>Some parts of the code are inspired by <a href="https://github.com/Quozul/PicoLimbo">PicoLimbo</a>.</sub>
</p>