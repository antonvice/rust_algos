# Game of Life
This is a simple implementation of Conway's Game of Life in Rust. The game is played on a grid of cells, where each cell can be either alive or dead. The game evolves over time according to a set of rules based on the number of live neighbors each cell has.

## Dependencies
The following libraries are used in this implementation:
```
std::io
std::io::Write
std::thread::sleep
std::time::Duration
crossterm
std::sync::mpsc
std::error::Error
rand::Rng
```
## Code Structure
The code is organized into a main function and a GameOfLife struct with its associated methods.

### Main Function
The main function initializes the terminal, creates the game state, and enters the main game loop. In the loop, it handles input events, updates the game state, and sleeps for a short duration.
```rust
fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the terminal
    ...
    // Create the game state
    let mut game = GameOfLife::new();
    // Main game loop
    ...
    // Cleanup
    ...
    Ok(())
}
```

### GameOfLife Struct
The GameOfLife struct contains the necessary fields for the game state. It has the following methods:
```rust
new(): Initializes the struct with a random start state.
handle_key(key: KeyCode): Handles key events and GUI events, such as button clicks.
update(): Updates the game state and the GUI.
struct GameOfLife {
    // Add necessary fields here
}

impl GameOfLife {
    fn new() -> Self {
        ...
    }

    fn handle_key(&mut self, key: KeyCode) {
        ...
    }

    fn update(&mut self) {
        ...
    }
}
```

## Usage
To run the game, simply execute the main function. The game will start with a random initial state and evolve according to the rules of Conway's Game of Life. You can interact with the game using keyboard inputs. Press 'r' to reset the game state to a new random start state.

## License
This project is open-source and available under the MIT License.
