use crate::{H, MOVE_SPEED, PLAYER_COLOUR, TILE_SIZE, W};
use raylib::prelude::*;
extern crate rand;
use rand::Rng;

pub struct Cube {
    colour: Color,
    rect: Rectangle,
}

impl Cube {
    pub fn new(place: Vector2, colour: Color) -> Cube {
        Cube {
            colour: colour,
            rect: rrect(place.x, place.y, TILE_SIZE, TILE_SIZE),
        }
    }

    pub fn go_to_random_pos(&mut self) {
        self.rect.x = rand::thread_rng().gen_range(0..W - TILE_SIZE + 1) as f32;
        self.rect.y = rand::thread_rng().gen_range(TILE_SIZE..H - TILE_SIZE + 1) as f32;
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle_rec(self.rect, self.colour);
    }
}

pub struct Snake {
    body: Vec<Cube>,
    desired_dir: (f32, f32),
    direction: (f32, f32),
    should_change: bool,
}

impl Snake {
    pub fn new() -> Snake {
        Snake {
            body: vec![Cube::new(
                Vector2::new((W / 2) as f32, (H / 2) as f32),
                PLAYER_COLOUR,
            )],
            desired_dir: (1f32, 0f32),
            direction: (1f32, 0f32),
            should_change: false,
        }
    }

    pub fn update(&mut self, left: bool, right: bool, up: bool, down: bool) {
        // Horizontal Movement Checks
        if left && self.direction.0 == 0f32 {
            self.desired_dir = (-1f32, 0f32);
            self.should_change = true;
        } else if right && self.direction.0 == 0f32 {
            self.desired_dir = (1f32, 0f32);
            self.should_change = true;
        }

        // Vertical Movement Checks
        if up && self.direction.1 == 0f32 {
            self.desired_dir = (0f32, -1f32);
            self.should_change = true;
        } else if down && self.direction.1 == 0f32 {
            self.desired_dir = (0f32, 1f32);
            self.should_change = true;
        }

        if self.should_change && self.on_tile() {
            self.direction = self.desired_dir;
            self.should_change = false;
        }

        if self.body.len() > 1 {
            let mut cur_i: usize;
            // Move Body Starting from End (ignoring head)
            for i in 1..self.body.len() {
                // Current Section
                cur_i = self.body.len() - i;
                self.body[cur_i].rect.x = self.body[cur_i - 1].rect.x;
                self.body[cur_i].rect.y = self.body[cur_i - 1].rect.y;
            }
        }

        // Move Head
        self.body[0].rect.x += self.direction.0 * MOVE_SPEED;
        self.body[0].rect.y += self.direction.1 * MOVE_SPEED;
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, offset: (i32, i32)) {
        for item in &self.body {
            d.draw_rectangle(
                item.rect.x as i32 + offset.0,
                item.rect.y as i32 + offset.1,
                item.rect.width as i32,
                item.rect.height as i32,
                item.colour,
            );
        }
    }

    fn on_tile(&self) -> bool {
        if self.body[0].rect.x % TILE_SIZE as f32 == 0f32
            && self.body[0].rect.y % TILE_SIZE as f32 == 0f32
        {
            return true;
        }
        false
    }

    pub fn is_dead(&self) -> bool {
        // Out of Bounds Horizontally
        if self.body[0].rect.x < 0f32 || self.body[0].rect.x > (W - TILE_SIZE) as f32 {
            return true;
        }
        // Out of Bounds Vertically
        else if self.body[0].rect.y < 0f32 || self.body[0].rect.y > (H - TILE_SIZE) as f32 {
            return true;
        }

        // Crashed Into Body (not first few sections that will always hit)
        if self.body.len() >= (TILE_SIZE / MOVE_SPEED as i32 * 3) as usize
            && self.body[(TILE_SIZE / MOVE_SPEED as i32 * 3) as usize..self.body.len()]
                .iter()
                .any(|section| section.rect.check_collision_recs(&self.body[0].rect))
        {
            return true;
        }

        false
    }

    pub fn grow(&mut self) {
        let mut count = 0;
        while count < TILE_SIZE / MOVE_SPEED as i32 {
            self.body.push(Cube::new(
                Vector2::new(
                    self.body[self.body.len() - 1].rect.x,
                    self.body[self.body.len() - 1].rect.y,
                ),
                PLAYER_COLOUR,
            ));
            count += 1;
        }
    }

    pub fn hit_food(&self, food: &Cube) -> bool {
        if self
            .body
            .iter()
            .any(|section| section.rect.check_collision_recs(&food.rect))
        {
            return true;
        }
        false
    }
}
