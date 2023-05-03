use macroquad::prelude::*;

const TILE_SIZE: f32 = 32.0;
const SPEED: f32 = 2.0;

struct Pacman {
    position: Vec2,
    velocity: Vec2,
}

struct Ghost {
    position: Vec2,
    velocity: Vec2,
}

struct Game {
    pacman: Pacman,
    ghosts: Vec<Ghost>,
    map: Vec<Vec<char>>,
    score: u32,
    lives: u32,
    game_over: bool,
}

impl Game {
    fn new() -> Game {
        let pacman = Pacman {
            position: vec2(1.0 * TILE_SIZE, 1.0 * TILE_SIZE),
            velocity: vec2(0.0, 0.0),
        };

        let ghost1 = Ghost {
            position: vec2(4.0 * TILE_SIZE, 1.0 * TILE_SIZE),
            velocity: vec2(0.0, 0.0),
        };

        let map = vec![
            vec!['#', '#', '#', '#', '#', '#', '#', '#', '#', '#'],
            vec!['#', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
            vec!['#', '.', '#', '#', '.', '#', '#', '.', '.', '#'],
            vec!['#', '.', '#', '.', '.', '.', '#', '.', '.', '#'],
            vec!['#', '.', '#', '.', '#', '.', '#', '.', '.', '#'],
            vec!['#', '.', '.', '.', '#', '.', '.', '.', '.', '#'],
            vec!['#', '#', '#', '#', '#', '#', '#', '#', '#', '#'],
        ];

        Game {
            pacman,
            ghosts: vec![ghost1],
            map,
            score: 0,
            lives: 3,
            game_over: false,
        }
    }

    fn draw(&self) {
        clear_background(WHITE);

        for (y, row) in self.map.iter().enumerate() {
            for (x, &ch) in row.iter().enumerate() {
                let pos = vec2(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE);

                match ch {
                    '#' => draw_rectangle(pos.x, pos.y, TILE_SIZE, TILE_SIZE, BLUE),
                    '.' => draw_circle(
                        pos.x + TILE_SIZE / 2.0,
                        pos.y + TILE_SIZE / 2.0,
                        TILE_SIZE / 4.0,
                        YELLOW,
                    ),
                    _ => (),
                }
            }
        }

        draw_circle(
            self.pacman.position.x + TILE_SIZE / 2.0,
            self.pacman.position.y + TILE_SIZE / 2.0,
            TILE_SIZE / 2.0,
            YELLOW,
        );

        for ghost in &self.ghosts {
            draw_circle(
                ghost.position.x + TILE_SIZE / 2.0,
                ghost.position.y + TILE_SIZE / 2.0,
                TILE_SIZE / 2.0,
                RED,
            );
        }
    }

    fn is_valid_move(&self, position: Vec2) -> bool {
        let x = (position.x / TILE_SIZE) as usize;
        let y = (position.y / TILE_SIZE) as usize;
        self.map[y][x] != '#'
    }

    fn update_pacman(&mut self) {
        let mut new_velocity = self.pacman.velocity;

        if is_key_down(KeyCode::Up) {
            new_velocity = vec2(0.0, -SPEED);
        }

        if is_key_down(KeyCode::Down) {
            new_velocity = vec2(0.0, SPEED);
        }

        if is_key_down(KeyCode::Left) {
            new_velocity = vec2(-SPEED, 0.0);
        }
        if is_key_down(KeyCode::Right) {
            new_velocity = vec2(SPEED, 0.0);
        }

        let new_position = self.pacman.position + new_velocity;
        if self.is_valid_move(new_position) {
            self.pacman.velocity = new_velocity;
            self.pacman.position = new_position;
        }
    }

    fn update_ghosts(&mut self) {
        let mut new_positions = Vec::new();

        for ghost in &self.ghosts {
            let direction = rand::gen_range(1, 4);

            let new_velocity = match direction {
                0 => vec2(0.0, -SPEED),
                1 => vec2(0.0, SPEED),
                2 => vec2(-SPEED, 0.0),
                3 => vec2(SPEED, 0.0),
                _ => vec2(0.0, 0.0),
            };

            let new_position = ghost.position + new_velocity;
            if self.is_valid_move_for_ghost(new_position) {
                new_positions.push((ghost.position, new_velocity));
            } else {
                new_positions.push((ghost.position, vec2(0.0, 0.0)));
            }
        }

        for (i, (position, velocity)) in new_positions.into_iter().enumerate() {
            self.ghosts[i].position = position;
            self.ghosts[i].velocity = velocity;
        }
    }

    fn is_valid_move_for_ghost(&self, position: Vec2) -> bool {
        let x = (position.x / TILE_SIZE) as usize;
        let y = (position.y / TILE_SIZE) as usize;
        self.map[y][x] != '#'
    }

    fn eat_pellet(&mut self) {
        let x = (self.pacman.position.x / TILE_SIZE) as usize;
        let y = (self.pacman.position.y / TILE_SIZE) as usize;

        if self.map[y][x] == '.' {
            self.map[y][x] = ' ';
            self.score += 10;
        }
    }

    fn check_collisions(&self) -> bool {
        for ghost in &self.ghosts {
            let dist = self.pacman.position.distance(ghost.position);
            if dist < TILE_SIZE {
                return true;
            }
        }
        false
    }

    fn update(&mut self) {
        if !self.game_over {
            self.update_pacman();
            self.eat_pellet();
            self.update_ghosts();
            let pacman_caught = self.check_collisions();

            if pacman_caught {
                self.lives -= 1;
                if self.lives == 0 {
                    self.game_over = true;
                } else {
                    self.reset_positions();
                }
            }
        }
    }

    fn reset_positions(&mut self) {
        self.pacman.position = vec2(1.0 * TILE_SIZE, 1.0 * TILE_SIZE);
        self.pacman.velocity = vec2(0.0, 0.0);

        self.ghosts[0].position = vec2(4.0 * TILE_SIZE, 1.0 * TILE_SIZE);
        self.ghosts[0].velocity = vec2(0.0, 0.0);
    }
}

#[macroquad::main("Pac-Man")]
async fn main() {
    let mut game = Game::new();

    loop {
        game.update();
        game.draw();

        if game.game_over {
            println!("Game Over! Final Score: {}", game.score);
            break;
        }

        next_frame().await;
    }
}
