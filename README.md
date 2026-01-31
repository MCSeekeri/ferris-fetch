# ferris-fetch 🦀

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)
[![GitHub stars](https://img.shields.io/github/stars/NORMAL-EX/ferris-fetch?style=for-the-badge)](https://github.com/NORMAL-EX/ferris-fetch/stargazers)

> A fast and cute system information tool written in Rust, featuring Ferris the crab! 🦀

<p align="center">
  <img src="https://i.imgur.com/3CjKlQa.png" alt="ferris-fetch demo" width="600">
</p>

## ✨ Features

- 🚀 **Blazingly Fast** - Pure Rust implementation with minimal dependencies
- 🌈 **Beautiful Themes** - Multiple color themes (rust, ocean, forest, sunset, mono)
- 🦀 **Cute Ferris** - ASCII art featuring the beloved Rust mascot
- 💻 **Cross-Platform** - Works on Windows, Linux, and macOS
- 📊 **Rich Information** - OS, Kernel, Uptime, Shell, CPU, Memory, and more
- 🎨 **Customizable** - Minimal mode, disable colors, hide art

## 📦 Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/NORMAL-EX/ferris-fetch.git
cd ferris-fetch

# Build and install
cargo install --path .
```

### Using Cargo

```bash
cargo install ferris-fetch
```

### Pre-built Binaries

Download the latest release from the [Releases](https://github.com/NORMAL-EX/ferris-fetch/releases) page.

## 🚀 Usage

```bash
# Default display with Ferris art
ferris-fetch

# Use a different color theme
ferris-fetch --theme ocean
ferris-fetch --theme forest
ferris-fetch --theme sunset
ferris-fetch --theme mono

# Minimal mode (smaller Ferris, less info)
ferris-fetch --minimal

# Disable colors (for piping or non-color terminals)
ferris-fetch --no-color

# Hide ASCII art
ferris-fetch --no-art

# Combine options
ferris-fetch --theme ocean --minimal
```

## 🎨 Themes

| Theme   | Description                          |
|---------|--------------------------------------|
| `rust`  | 🟠 Default orange theme (Rust colors) |
| `ocean` | 🔵 Blue and cyan theme               |
| `forest`| 🟢 Green and yellow theme            |
| `sunset`| 🔴 Red, yellow and magenta theme     |
| `mono`  | ⚪ Monochrome white theme            |

## 📋 Example Output

```
        _~^~^~_          cja@NORMAL-PC
    \) /  o o  \ (/      ─────────────────
      '_   -   _'        OS: Windows 11 22H2
      / '-----' \        Kernel: 22631
     /           \       Uptime: 5h 32m
    /  /       \  \      Shell: pwsh.exe
   (  |         |  )     CPU: AMD Ryzen 7 5800X (16 cores)
    \_|         |_/      Memory: 12.3 GB / 32.0 GB [████████░░] 38%

                         ███████████████████████████
                         ███████████████████████████
```

## 🔧 Building

### Requirements

- Rust 1.70+ (2021 edition)
- Cargo

### Build Commands

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run directly
cargo run -- --theme ocean
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Inspired by [neofetch](https://github.com/dylanaraps/neofetch), [fastfetch](https://github.com/fastfetch-cli/fastfetch)
- Ferris the crab 🦀 - the unofficial Rust mascot
- Built with ❤️ using Rust

## 🤝 Contributing

Contributions are welcome! Feel free to:

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📊 Star History

[![Star History Chart](https://api.star-history.com/svg?repos=NORMAL-EX/ferris-fetch&type=Date)](https://star-history.com/#NORMAL-EX/ferris-fetch&Date)

---

<p align="center">
  Made with 🦀 by <a href="https://github.com/NORMAL-EX">NORMAL-EX</a>
</p>
