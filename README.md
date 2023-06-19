# Rust Boomerang Game

This project is a simple terminal-based game written in Rust. In the game, you control a player who can throw boomerangs at incoming enemies. The game uses the `crossterm` crate for terminal I/O.

## Installation

1. Make sure you have Rust installed. If not, download it from the official website and install it.
2. Clone this repository to your local machine.
3. Go to the cloned directory and run `cargo build --release`.

## How to play

After you've successfully compiled the game, run it using the following command:

```
cargo run
```

You will be prompted to enter the height and width of the playing field. After that, the game starts.

You can control the player using the following keys:

- `w`: Move up
- `s`: Move down
- `a`: Move left
- `d`: Move right
- ` `: Throw boomerang

You can exit the game at any time by pressing `q`.

## Game Entities

The game has several different entities, such as the player, the boomerangs, and the enemies.

- **Player**: The player is controlled by you. The player can move around and throw boomerangs.

- **Boomerang**: The player can throw a boomerang in the direction they are facing. When a boomerang hits an enemy, it destroys the enemy and becomes safe to pick up again. The player can throw the boomerang again once they pick it up.

- **Enemy**: Enemies move towards the player from the right side of the screen. If an enemy touches the player, the game is over.

## Contributing

This project is open-source, and contributions are welcome. If you want to contribute, please make sure to follow the Rust coding standards.

Please note that this project is released with a Contributor Code of Conduct. By participating in this project, you agree to abide by its terms.

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for more details.