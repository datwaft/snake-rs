#[derive(Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Point = (u16, u16);

#[derive(Debug)]
pub struct Snake {
    pub direction: Direction,
    pub body: Vec<Point>,
    pub digesting: bool,
}

impl Snake {
    pub fn new(position: Point) -> Snake {
        Snake {
            direction: Direction::Up,
            body: vec![position],
            digesting: false,
        }
    }

    pub fn future_head(&self, (width, height): (u16, u16)) -> Option<Point> {
        let (x, y) = self.body[0];
        match self.direction {
            Direction::Up => {
                if y == 0 {
                    None
                } else {
                    Some((x, y - 1))
                }
            }
            Direction::Down => {
                if y == height - 1 {
                    None
                } else {
                    Some((x, y + 1))
                }
            }
            Direction::Left => {
                if x == 0 {
                    None
                } else {
                    Some((x - 1, y))
                }
            }
            Direction::Right => {
                if x == width - 1 {
                    None
                } else {
                    Some((x + 1, y))
                }
            }
        }
    }

    pub fn tick(&mut self, game_size: (u16, u16)) -> Result<(), &'static str> {
        if let Some(future_head) = self.future_head(game_size) {
            self.body.insert(0, future_head);
        } else {
            return Err("invalid tick");
        };

        if self.digesting {
            self.digesting = false;
        } else {
            self.body.pop();
        }

        Ok(())
    }
}
