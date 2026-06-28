<div align="center">

# Yoki

**A high-performance Minecraft server written from scratch in Rust**

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-2024%20edition-orange.svg)](https://www.rust-lang.org)
[![GitHub stars](https://img.shields.io/github/stars/AxenoDev/Yoki?style=social)](https://github.com/AxenoDev/Yoki/stargazers)
[![GitHub issues](https://img.shields.io/github/issues/AxenoDev/Yoki)](https://github.com/AxenoDev/Yoki/issues)
[![GitHub pull requests](https://img.shields.io/github/issues-pr/AxenoDev/Yoki)](https://github.com/AxenoDev/Yoki/pulls)

</div>

Yoki is a lightweight, blazing-fast Minecraft server implementation built from the ground up in Rust. Designed as a modern, memory-efficient alternative to traditional Java servers like Paper, it handles game logic, worlds, and player connections natively with optimal performance.

## Getting Started

Clone the repository and run the proxy:

```bash
git clone https://github.com/AxenoDev/Yoki.git
cd Yoki
cargo run -- -c target/config/
```

The proxy listens on **`0.0.0.0:25565`** by default.

## Building

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

The release binary is available at `target/release/yoki`.

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) before opening a pull request.

By participating in this project, you agree to abide by our [Code of Conduct](CODE_OF_CONDUCT.md).

## Security

If you discover a security vulnerability, please report it responsibly. See [SECURITY.md](SECURITY.md) for details.

## License

Yoki is licensed under the [GNU General Public License v3.0](LICENSE).

<p align="center">
  <sub>Some parts of the code are inspired by <a href="https://github.com/Quozul/PicoLimbo">PicoLimbo</a>.</sub>
</p>