# Bugsweeper

[![Itch.io](https://img.shields.io/badge/Itch-%23FF0B34.svg?style=for-the-badge&logo=Itch.io&logoColor=white)](https://sanket143.itch.io/bugsweeper)
[![](https://img.shields.io/badge/Twitch-9146FF?style=for-the-badge&logo=twitch&logoColor=white)](https://www.twitch.tv/sanket143__)

Bugsweeper is like Minesweeper but along with mines, it also includes bugs.
The difference between bugs and mines is a bug can move around the grid, in close tiles,
while Mines cannot. The goal is to kill all the mines, uncover all the safe tiles, and avoid mines.
Opening a mine will end the game.

# Compiling
It requires cargo (Rust's package manager)
```
$ cargo run # should just run the game
$ cargo build --release # to create a release binary in ./target/release
```
