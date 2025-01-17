# 💣 Rusty Minesweeper

![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange.svg)
![GTK4](https://img.shields.io/badge/GTK-4.0-blue.svg)

A reimplementation of the classic Windows XP Minesweeper, built with Rust and GTK4.

![game-beginner2](https://github.com/user-attachments/assets/580412c6-2528-467b-af50-454fac145307)


## ✨ Features

- 🎮 Classic Minesweeper gameplay
- 🎯 Three difficulty levels: Beginner, Intermediate, and Expert
- ⏱️ Real-time game timer
- 🚩 Mine flagging system
- 😎 Classic XP-style emojis for game states
- 🎨 GTK4 interface

## 🚀 Technical Details

- Built using Rust 🦀
- Leverages `relm4` for reactive GUI programming with GTK4
- Minimal dependencies

## 🛠️ Building from Source

### Prerequisites

- Rust 1.75 or higher
- GTK4 development libraries

```bash
# Clone the repository
git clone https://github.com/not4rt/rusty-minesweeper
cd rusty-minesweeper

# Build and run
cargo run --release
```

## 🎮 How to Play

1. Left-click to reveal cells
2. Right-click to flag potential mines
3. Clear all non-mine cells to win!
4. Choose your difficulty level from the Game menu

## Notes

This project showcases some Rust concepts and patterns:

- Custom component architecture using `relm4`
- Board generation algorithm
- Type-safe game state management
- Reactive UI updates using GTK4's features
