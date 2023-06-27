use raylib::prelude::*;
mod snake;
use rand::Rng;
use snake::{Cube, Snake};
use std::{fs, io::Write};
const BG_COLOUR: Color = Color::BLACK;
const FG_COLOUR: Color = Color::WHITE;
const PLAYER_COLOUR: Color = Color::RED;
const FOOD_COLOUR: Color = Color::GREEN;
const TILE_SIZE: i32 = 24;
const MOVE_SPEED: f32 = 3f32;
const W: i32 = TILE_SIZE * 22;
const H: i32 = TILE_SIZE * 20;
const FPS: u32 = 45u32;
const TITLE_FONT_SIZE: i32 = 24;

enum GameState {
    Starting,
    Playing,
    GameOver,
    Paused,
}

fn main() {
    let (mut rl, thread) = raylib::init().size(W, H).title("Better Snake Rust").build();
    rl.set_target_fps(FPS);

    // Game Variables
    let mut current_main_text = "Press Space to Start";
    let mut subtitle_text = "";
    let mut game_state = GameState::Starting;
    let mut screen_shake = 0f32;
    let mut score = 0;
    let mut highscore: i32 = fs::read_to_string("./data/highscore.txt").expect("Couldn't Open File").split("\n").last().expect("Weird Data in File").parse().expect("NaN in File");


    let mut space_pressed: bool;
    let mut down_pressed: bool;
    let mut up_pressed: bool;
    let mut left_pressed: bool;
    let mut right_pressed: bool;

    let mut snake = Snake::new();
    let mut food = Cube::new(Vector2::new(0f32, 0f32), FOOD_COLOUR);

    while !rl.window_should_close() {
        // Get Input Info
        space_pressed = rl.is_key_pressed(KeyboardKey::KEY_SPACE);
        down_pressed = rl.is_key_down(KeyboardKey::KEY_DOWN);
        up_pressed = rl.is_key_down(KeyboardKey::KEY_UP);
        left_pressed = rl.is_key_down(KeyboardKey::KEY_LEFT);
        right_pressed = rl.is_key_down(KeyboardKey::KEY_RIGHT);

        // Updating
        match game_state {
            GameState::Starting | GameState::GameOver => {
                if space_pressed {
                    score = 0;
                    game_state = GameState::Playing;
                    snake = Snake::new();
                    food.go_to_random_pos();
                    while snake.hit_food(&food) {
                        food.go_to_random_pos();
                    }
                }
            }
            GameState::Playing => {
                // Updating the Snake
                snake.update(left_pressed, right_pressed, up_pressed, down_pressed);
                // Food Collision Check
                if snake.hit_food(&food) {
                    snake.grow();
                    score += 1;
                    food.go_to_random_pos();
                    if score < W / TILE_SIZE * H / TILE_SIZE {
                        while snake.hit_food(&food) {
                            food.go_to_random_pos();
                        }
                    } else {
                        game_state = GameState::GameOver;
                        subtitle_text = "You Won!";
                        current_main_text = "Press Space to Restart.";
                        //  New High Score
                        highscore = score;

                        // Opening High Score File
                        let mut data_file = fs::OpenOptions::new()
                            .append(true)
                            .open("./data/highscore.txt")
                            .expect("cannot open file"
                        );

                        // Saving to High Score File
                        data_file
                            .write(format!("\n{}", highscore).as_bytes())
                            .expect("Couldn't Save High Score");
                    }
                }

                // Pause Checks
                if space_pressed {
                    game_state = GameState::Paused;
                    subtitle_text = "Game Paused";
                    current_main_text = "Press Space to Resume";
                    if score > highscore {
                        // New High Score
                        highscore = score;

                        // Opening High Score File
                        let mut data_file = fs::OpenOptions::new()
                            .append(true)
                            .open("./data/highscore.txt")
                            .expect("cannot open file"
                        );

                        // Saving to High Score File
                        data_file
                            .write(format!("\n{}", highscore).as_bytes())
                            .expect("Couldn't Save High Score");
                    }
                }

                // Game Over Checks
                if snake.is_dead() {
                    game_state = GameState::GameOver;
                    screen_shake = (FPS / 6) as f32;
                    subtitle_text = "Game Over.";
                    current_main_text = "Press Space to Start";
                    if score > highscore {
                        // New High Score
                        highscore = score;

                        // Opening High Score File
                        let mut data_file = fs::OpenOptions::new()
                            .append(true)
                            .open("./data/highscore.txt")
                            .expect("cannot open file"
                        );

                        // Saving to High Score File
                        data_file
                            .write(format!("\n{}", highscore).as_bytes())
                            .expect("Couldn't Save High Score");
                    }
                }
            }
            GameState::Paused => {
                if space_pressed {
                    game_state = GameState::Playing;
                }
            }
        }

        // Drawing
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(BG_COLOUR);

        match game_state {
            GameState::Starting => {
                d.draw_text(
                    current_main_text,
                    W / 2 - measure_text(current_main_text, TITLE_FONT_SIZE) / 2,
                    H / 2 - TITLE_FONT_SIZE / 2,
                    TITLE_FONT_SIZE,
                    FG_COLOUR,
                );
            }
            GameState::GameOver | GameState::Paused => {
                let mut offs = (0, 0);
                if screen_shake > 0f32 {
                    offs.0 = rand::thread_rng().gen_range(-26..26) / 10;
                    offs.1 = rand::thread_rng().gen_range(-26..26) / 10;
                    screen_shake -= 1f32;
                }
                food.draw(&mut d);
                snake.draw(&mut d, offs);
                d.draw_text(
                    current_main_text,
                    W / 2 - measure_text(current_main_text, TITLE_FONT_SIZE) / 2 + offs.0,
                    H / 2 - TITLE_FONT_SIZE / 2 + offs.1,
                    TITLE_FONT_SIZE,
                    FG_COLOUR,
                );
                d.draw_text(
                    subtitle_text,
                    W / 2 - measure_text(subtitle_text, TITLE_FONT_SIZE - 1) / 2 + offs.0,
                    H / 2 - TITLE_FONT_SIZE - 5 - (TITLE_FONT_SIZE - 1) / 2 + offs.1,
                    TITLE_FONT_SIZE - 1,
                    FG_COLOUR,
                );
                d.draw_text(
                    format!("High Score: {}", highscore).as_str(),
                    8 + offs.0,
                    8 + offs.1,
                    TITLE_FONT_SIZE - 3,
                    FG_COLOUR,
                );
                d.draw_text(
                    format!("Score: {}", score).as_str(),
                    W - measure_text(format!("Score: {}", score).as_str(), TITLE_FONT_SIZE - 3) - 8 + offs.0,
                    8 + offs.1,
                    TITLE_FONT_SIZE - 3,
                    FG_COLOUR,
                );
            }
            GameState::Playing => {
                food.draw(&mut d);
                snake.draw(&mut d, (0, 0));
                d.draw_text(
                    format!("Score: {}", score).as_str(),
                    W - measure_text(format!("Score: {}", score).as_str(), TITLE_FONT_SIZE - 3) - 8,
                    8,
                    TITLE_FONT_SIZE - 3,
                    FG_COLOUR,
                );
            }
        }
    }
}
