use crate::vector2::Vector2;

pub struct Cursor{
    position: Vector2,
}

impl Cursor {
    pub fn new() -> Cursor {
        Cursor {
            position: Vector2{x: 0, y: 0},
        }
    }
    
    pub fn get_position(&self) -> Vector2 {
        self.position
    }


    pub fn reset(&mut self){
        self.position = Vector2{x: 0, y: 0}
    }

    pub fn up(&mut self){
        if self.position.y > 0 {
            self.position.y -= 1;
        }
    }

    pub fn down(&mut self){
        if self.position.y < 9 {
            self.position.y += 1;
        }
    }

    pub fn right(&mut self){
        if self.position.x < 9 {
            self.position.x += 1;
        }
    }

    pub fn left(&mut self){
        if self.position.x > 0{
            self.position.x -= 1
        }
    }
}