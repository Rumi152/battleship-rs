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
        Ship::new_with_direction(ship_size, position, Directions::Up)
    }

    pub fn new_with_direction(ship_size: u8, position: Vector2, direction: Directions) -> Ship {
        let dir_offset = direction.as_vector2() * -1;

        match ship_size {
            1 => Ship {
                segments: vec![ShipSegment {
                    position: Vector2 {
                        x: position.x,
                        y: position.y,
                    },
                }],
                rotation: direction,
            },
            2 => Ship {
                segments: vec![
                    ShipSegment {
                        position: Vector2 {
                            x: position.x + dir_offset.x,
                            y: position.y + dir_offset.y,
                        },
                    },
                    ShipSegment {
                        position: Vector2 {
                            x: position.x,
                            y: position.y,
                        },
                    },
                ],
                rotation: direction,
            },
            3 => Ship {
                segments: vec![
                    ShipSegment {
                        position: Vector2 {
                            x: position.x + 2 * dir_offset.x,
                            y: position.y + 2 * dir_offset.y,
                        },
                    },
                    ShipSegment {
                        position: Vector2 {
                            x: position.x + 1 * dir_offset.x,
                            y: position.y + 1 * dir_offset.y,
                        },
                    },
                    ShipSegment {
                        position: Vector2 {
                            x: position.x,
                            y: position.y,
                        },
                    },
                ],
                rotation: direction,
            },
            4 => Ship {
                segments: vec![
                    ShipSegment {
                        position: Vector2 {
                            x: position.x + 3 * dir_offset.x,
                            y: position.y + 3 * dir_offset.y,
                        },
                    },
                    ShipSegment {
                        position: Vector2 {
                            x: position.x + 2 * dir_offset.x,
                            y: position.y + 2 * dir_offset.y,
                        },
                    },
                    ShipSegment {
                        position: Vector2 {
                            x: position.x + 1 * dir_offset.x,
                            y: position.y + 1 * dir_offset.y,
                        },
                    },
                    ShipSegment {
                        position: Vector2 {
                            x: position.x,
                            y: position.y,
                        },
                    },
                ],
                rotation: direction,
            },
            5 => Ship {
                segments: vec![
                    ShipSegment {
                        position: Vector2 {
                            x: position.x + 4 * dir_offset.x,
                            y: position.y + 4 * dir_offset.y,
                        },
                    },
                    ShipSegment {
                        position: Vector2 {
                            x: position.x + 3 * dir_offset.x,
                            y: position.y + 3 * dir_offset.y,
                        },
                    },
                    ShipSegment {
                        position: Vector2 {
                            x: position.x + 2 * dir_offset.x,
                            y: position.y + 2 * dir_offset.y,
                        },
                    },
                    ShipSegment {
                        position: Vector2 {
                            x: position.x + 1 * dir_offset.x,
                            y: position.y + 1 * dir_offset.y,
                        },
                    },
                    ShipSegment {
                        position: Vector2 {
                            x: position.x,
                            y: position.y,
                        },
                    },
                ],
                rotation: direction,
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
