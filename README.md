# Pac-Mac Rust Game generated with GPT-4

![Pac-Mac Rust Game generated with GPT-4!](/img/game.png "Pac-Mac")


## An explanation for each line of code in the program


1. `use macroquad::prelude::*;`: Imports all necessary items from the `macroquad` crate, which is a game framework.

2-4. `const TILE_SIZE: f32 = 32.0;`, `const SPEED: f32 = 2.0;`: Define constants for the tile size and speed of the game characters.

5-9. Define a `Pacman` struct with `position` and `velocity` fields as 2D vectors.

10-14. Define a `Ghost` struct with the same fields as `Pacman`.

15-22. Define a `Game` struct with fields for the Pac-Man character, a vector of ghosts, the game map, score, lives, and game over status.

24-42. Implement a `new` function for the `Game` struct. This function initializes a new game state with a Pac-Man character, one ghost, a map, and the initial score, lives, and game over status.

44-70. Implement a `draw` function for the `Game` struct. This function draws the game map, Pac-Man, and ghosts on the screen using the `macroquad` crate's drawing functions.

72-77. Implement an `is_valid_move` function. This function checks if a move is valid by verifying that the new position isn't a wall.

79-93. Implement an `update_pacman` function. This function updates the position and velocity of Pac-Man based on user input and only moves if the new position is valid.

95-116. Implement an `update_ghosts` function. This function updates the position and velocity of each ghost randomly while ensuring the new position is valid.

118-123. Implement an `is_valid_move_for_ghost` function, which checks if the new position is valid for a ghost to move to.

125-133. Implement an `eat_pellet` function. This function checks if Pac-Man is on a pellet and updates the map and score if a pellet is eaten.

135-143. Implement a `check_collisions` function. This function checks if Pac-Man is too close to any of the ghosts, indicating a collision.

145-162. Implement an `update` function. This function updates the game state by calling the update, eat_pellet, and check_collision functions, and updates the game status based on the results.

164-175. Implement a `reset_positions` function. This function resets the position and velocity of Pac-Man and the ghosts after a collision.

177-194. Define the `main` function, which initializes a new game and enters the game loop. In the loop, the game state is updated and drawn. If the game is over, it prints the final score and breaks the loop.

This program is a simple implementation of a Pac-Man-like game using the `macroquad` crate. It features a Pac-Man character that can move around and eat pellets, one ghost that moves randomly, and a basic map with walls.
