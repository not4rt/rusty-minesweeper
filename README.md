# 💣 Rusty Minesweeper

![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange.svg)
![GTK4](https://img.shields.io/badge/GTK-4.0-blue.svg)

A reimplementation of the classic Windows Minesweeper, built with Rust and Relm4/GTK4.

![game-beginner3](https://github.com/user-attachments/assets/34a434c2-81aa-4938-89a2-c2efdff9e308)

## Features

- 🎮 Classic Minesweeper gameplay
- 😎 Emojis for game states
- 🎯 Difficulty levels: Beginner, Intermediate, Expert and Custom
- ⏱️ Real-time game timer
- 🚩 [Chording](https://minesweeper.fandom.com/wiki/Chording) and flagging system

## Technical Details

- Built using Rust 🦀
- Leverages `relm4` for simplier GUI programming with GTK4
- Minimal dependencies

## Building from Source

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

## How to Play

1. Left-click to reveal cells
2. Right-click to flag potential mines
3. Middle-click to [chord](https://minesweeper.fandom.com/wiki/Chording)
4. Clear all non-mine cells to win!
5. Choose your difficulty level from the Game menu