use std::ops;

#[derive(Copy, Clone)]
pub struct Vector2 {
    pub x: i8,
    pub y: i8,
}

impl ops::Add for Vector2 {
    type Output = Vector2;
    fn add(self, other: Vector2) -> Vector2 {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::Sub for Vector2 {
    type Output = Vector2;
    fn sub(self, other: Vector2) -> Vector2 {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl ops::Mul<i8> for Vector2 {
    type Output = Vector2;
    fn mul(self, other: i8) -> Vector2 {
        Vector2 {
            x: self.x * other,
            y: self.y * other,
        }
    }
    
}

impl PartialEq for Vector2 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for Vector2 {}
