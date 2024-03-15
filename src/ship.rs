use crate::{direction::Directions, vector2::Vector2};

pub struct ShipSegment {
    position: Vector2,
}

impl ShipSegment {
    pub fn get_position(&self) -> Vector2 {
        self.position
    }
}

pub struct Ship {
    segments: Vec<ShipSegment>,
    rotation: Directions,
}

impl Ship {
    pub fn new(ship_size: u8, position: Vector2) -> Ship {
        match ship_size {
            1 => Ship {
                segments: vec![ShipSegment {
                    position: Vector2 { x: position.x, y: position.y },
                }],
                rotation: Directions::Up,
            },
            2 => Ship {
                segments: vec![
                    ShipSegment {
                    position: Vector2 { x: position.x, y: position.y + 1 },
                    },
                    ShipSegment {
                        position: Vector2 { x: position.x, y: position.y },
                    },
                ],
                rotation: Directions::Up,
            },
            3 => Ship {
                segments: vec![
                    ShipSegment {
                        position: Vector2 { x: position.x, y: position.y + 2 },
                    },
                    ShipSegment {
                    position: Vector2 { x: position.x, y: position.y + 1 },
                    },
                    ShipSegment {
                        position: Vector2 { x: position.x, y: position.y },
                    },
                ],
                rotation: Directions::Up,
            },
            4 => Ship {
                segments: vec![
                    ShipSegment {
                        position: Vector2 { x: position.x, y: position.y + 3 },
                    },
                    ShipSegment {
                        position: Vector2 { x: position.x, y: position.y + 2 },
                    },
                    ShipSegment {
                    position: Vector2 { x: position.x, y: position.y + 1 },
                    },
                    ShipSegment {
                        position: Vector2 { x: position.x, y: position.y },
                    },
                ],
                rotation: Directions::Up,
            },
            5 => Ship {
                segments: vec![
                    ShipSegment {
                        position: Vector2 { x: position.x, y: position.y + 4 },
                    },
                    ShipSegment {
                        position: Vector2 { x: position.x, y: position.y + 3 },
                    },
                    ShipSegment {
                        position: Vector2 { x: position.x, y: position.y + 2 },
                    },
                    ShipSegment {
                    position: Vector2 { x: position.x, y: position.y + 1},
                    },
                    ShipSegment {
                        position: Vector2 { x: position.x, y: position.y },
                    },
                ],
                rotation: Directions::Up,
            },
            _ => panic!("Invalid ship size"),
        }
    }

    pub fn move_to(&mut self, position: Vector2) {
        for (i, x) in self.segments.iter_mut().enumerate() {
            x.position = position + self.rotation.as_vector2() * (i as i8);
        }
    }

    pub fn rotate(&mut self) {
        self.rotation.rotate();
        self.move_to(self.segments[0].position);
    }

    pub fn get_segments(&self) -> &[ShipSegment] {
        &self.segments
    }
}
