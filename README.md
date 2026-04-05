# Minesweeper CLI 💣

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

A fast, lightweight, and minimalist terminal-based Minesweeper game written entirely in [Rust](https://www.rust-lang.org/). Play the classic logic puzzle game directly from your command line without any graphical distractions.

## 🌟 Features

* **Terminal Native:** Runs entirely in your command line/terminal emulator.
* **Blazing Fast & Safe:** Built with Rust, ensuring snappy performance, memory safety, and minimal resource footprint.
* **Customizable Grids:** Play on standard difficulties (Beginner, Intermediate, Expert) or specify custom grid sizes and mine counts.
* **Keyboard Navigation:** Navigate and interact with the grid using intuitive keyboard controls.
* **Cross-Platform:** Works seamlessly on Linux, macOS, and Windows.

## 🛠️ Prerequisites

To build and run this project from source, you will need to have Rust and Cargo installed on your system. 

If you don't have Rust installed, you can easily get it via [rustup](https://rustup.rs/):
```bash
curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh
````

## 🚀 Installation

Clone the repository and compile the project using Cargo:

```bash
git clone [https://github.com/ibrohim-b/minesweeper-cli.git](https://github.com/ibrohim-b/minesweeper-cli.git)
cd minesweeper-cli
cargo build --release
```

To install the executable globally so you can run it from anywhere, use:

```bash
cargo install --path .
```

## ⌨️ Command Installation

After running `cargo install --path .`, the `minesweeper` command is placed in `~/.cargo/bin/`. Make sure that directory is on your `PATH`:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

Add that line to your shell config (`~/.bashrc`, `~/.zshrc`, etc.) to make it permanent. You can verify the installation with:

```bash
which minesweeper
minesweeper --version
```

## 🎮 Usage and Controls

Start the game by simply running the executable:

```bash
minesweeper
```

*(If you haven't installed it globally, you can just use `cargo run --release` from the project directory).*

### In-Game Controls

  * **Navigation:** Use `Arrow Keys` (or `w/a/s/d` / `h/j/k/l`) to move your cursor across the grid.
  * **Reveal Cell:** Press `Space` or `Enter` to dig/reveal a cell.
  * **Flag Mine:** Press `F` to plant or remove a flag (🚩) on suspected mines.
  * **Quit:** Press `Q` or `Ctrl+C` to exit the game.

## 🤝 Contributing

Contributions, issues, and feature requests are always welcome\! Feel free to check the [issues page](https://www.google.com/search?q=https://github.com/ibrohim-b/minesweeper-cli/issues) if you want to contribute.

1.  Fork the project.
2.  Create your feature branch (`git checkout -b feature/AmazingFeature`).
3.  Commit your changes (`git commit -m 'Add some AmazingFeature'`).
4.  Push to the branch (`git push origin feature/AmazingFeature`).
5.  Open a Pull Request.

## 📝 License

This project is licensed under the MIT License - see the `LICENSE` file for details.

-----

*Developed with ❤️ by [ibrohim-b](https://www.google.com/search?q=https://github.com/ibrohim-b)*
