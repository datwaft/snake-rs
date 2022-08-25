use crate::snake::{Direction, Snake};
use rand::seq::SliceRandom;

pub struct Game {
    size: (u16, u16),
    snake: Snake,
    food: (u16, u16),
}

impl Game {
    pub fn new(size: (u16, u16)) -> Game {
        let (width, height) = size;
        let snake = Snake::new((width / 2, height / 2));
        let food = Game::food_new_coords(&snake, size);
        Game { size, snake, food }
    }

    pub fn tick(&mut self) -> Result<(), &'static str> {
        if let Some(future_head) = self.snake.future_head(self.size) {
            if future_head == self.food {
                self.snake.digesting = true;
                self.food = Game::food_new_coords(&self.snake, self.size);
            }
            if self.snake.body.contains(&future_head) {
                return Err("snake ate itself");
            }
        };
        self.snake.tick(self.size)?;
        Ok(())
    }

    fn food_new_coords(snake: &Snake, size: (u16, u16)) -> (u16, u16) {
        let (width, height) = size;

        let mut possible_points = Vec::new();
        for row in 0..height {
            for col in 0..width {
                if !snake.body.contains(&(col, row)) {
                    possible_points.push((col, row));
                }
            }
        }

        *possible_points.choose(&mut rand::thread_rng()).unwrap()
    }

    pub fn render(&self) -> String {
        let (width, height) = self.size;

        let mut acc = String::with_capacity(usize::from(width * height));

        for _ in 0..width + 2 {
            acc += "*";
        }
        acc += "\r\n";

        for row in 0..height {
            acc += "*";
            for col in 0..width {
                if self.snake.body.contains(&(col, row)) {
                    acc += "█";
                } else if self.food == (col, row) {
                    acc += "*";
                } else {
                    acc += " ";
                }
            }
            acc += "*\r\n";
        }

        for _ in 0..width + 2 {
            acc += "*";
        }
        acc += "\r\n";

        acc
    }

    pub fn handle_up_event(&mut self) {
        if self.snake.direction != Direction::Down {
            self.snake.direction = Direction::Up;
        }
    }

    pub fn handle_down_event(&mut self) {
        if self.snake.direction != Direction::Up {
            self.snake.direction = Direction::Down;
        }
    }

    pub fn handle_left_event(&mut self) {
        if self.snake.direction != Direction::Right {
            self.snake.direction = Direction::Left;
        }
    }

    pub fn handle_right_event(&mut self) {
        if self.snake.direction != Direction::Left {
            self.snake.direction = Direction::Right;
        }
    }
}
