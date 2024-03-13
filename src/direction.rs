use crate::vector2::Vector2;

pub enum Directions {
    Up,
    Right,
    Down,
    Left,
}

impl Directions {
    pub fn as_vector2(&self) -> Vector2 {
        match self {
            Directions::Up => Vector2 { x: 0, y: -1 },
            Directions::Right => Vector2 { x: 1, y: 0 },
            Directions::Down => Vector2 { x: 0, y: 1 },
            Directions::Left => Vector2 { x: -1, y: 0 },
        }
    }
    
    pub fn rotate(&mut self) {
        *self = match self {
            Directions::Up => Directions::Right,
            Directions::Right => Directions::Down,
            Directions::Down => Directions::Left,
            Directions::Left => Directions::Up,
        };
    }
}
