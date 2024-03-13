// 0 1 2 3 4 5 6 7 8 9
//[][][][][][][][][][]0  [][][][] x1
//[][][][][][][][][][]1
//[][][][][][][][][][]2  [][][] x2
//[][][][][][][][][][]3
//[][][][][][][][][][]4  [][] x3
//[][][][][][][][][][]5
//[][][][][][][][][][]6  [] x4
//[][][][][][][][][][]7
//[][][][][][][][][][]8
//[][][][][][][][][][]9

use crate::{cursor::Cursor, ship::Ship, vector2::Vector2};

use crossterm::QueueableCommand;
use std::io::{stdout, Write};

pub struct Board {
    ships: Vec<Ship>,
    shot_positions: Vec<Vector2>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            ships: vec![
                Ship::new(4, Vector2 { x: 0, y: 0 }),
                Ship::new(3, Vector2 { x: 0, y: 5 }),
                Ship::new(3, Vector2 { x: 2, y: 0 }),
                Ship::new(2, Vector2 { x: 2, y: 5 }),
                Ship::new(2, Vector2 { x: 4, y: 0 }),
                Ship::new(2, Vector2 { x: 4, y: 5 }),
                Ship::new(1, Vector2 { x: 6, y: 0 }),
                Ship::new(1, Vector2 { x: 6, y: 5 }),
                Ship::new(1, Vector2 { x: 8, y: 0 }),
                Ship::new(1, Vector2 { x: 8, y: 5 }),
            ],
            shot_positions: Vec::new(),
            // shot_positions: vec![
            //     Vector2 { x: 0, y: 2 },
            //     Vector2 { x: 4, y: 1 },
            //     Vector2 { x: 0, y: 4 },
            //     Vector2 { x: 0, y: 3 },
            // ],
        }
    }

    pub fn render_your_pov(&self, starting_position: Vector2, cursor: Option<&Cursor>) {
        let mut stdout = stdout();

        for i in -1..11 {
            for j in -1..11 {
                let cell: String = if i == -1 && j == -1 {
                    "╔═".to_owned()
                } else if i == -1 && j == 10 {
                    "╚═".to_owned()
                } else if i == 10 && j == -1 {
                    "╗".to_owned()
                } else if i == 10 && j == 10 {
                    "╝".to_owned()
                } else if i == 10 {
                    if cursor.is_some_and(|cursor| cursor.get_position().y == j) {
                        format!("\x1b[0;33m{}\x1b[0m", j)
                    } else {
                        j.to_string()
                    }
                } else if j == -1 {
                    if cursor.is_some_and(|cursor| cursor.get_position().x == i) {
                        format!("\x1b[0;33m{}\x1b[0m", ('A' as u8 + i as u8) as char)
                    } else {
                        (('A' as u8 + i as u8) as char).to_string()
                    }
                } else if i == -1 {
                    "║".to_owned()
                } else if j == 10 {
                    "══".to_owned()
                } else if cursor.is_some()
                    && cursor.unwrap().get_position() == (Vector2 { x: i, y: j })
                {
                    "○".to_owned()
                } else {
                    let mut ships_flatmap = self.ships.iter().flat_map(|ship| ship.get_segments());
                    let current_iter_pos = Vector2 { x: i, y: j };

                    if let Some(&ref found_ship) =
                        ships_flatmap.find(|&segment| segment.get_position() == current_iter_pos)
                    {
                        if self.shot_positions.contains(&found_ship.get_position()) {
                            if &found_ship.get_position()
                                == self
                                    .shot_positions
                                    .last()
                                    .expect("Shot positions contain other element so its not empty")
                            {
                                "\x1b[0;31m⯐\x1b[m".to_owned()
                                // "⛝"
                            } else {
                                // "\x1b[0;31m⛶\x1b[m"
                                // "\x1b[0;31m⛝\x1b[m"
                                if self
                                    .ships
                                    .iter()
                                    .find(|&ship| {
                                        ship.get_segments().iter().any(|segment| {
                                            segment.get_position() == found_ship.get_position()
                                        })
                                    })
                                    .unwrap()
                                    .get_segments()
                                    .iter()
                                    .all(|segment| {
                                        self.shot_positions.contains(&segment.get_position())
                                    })
                                {
                                    "\x1b[0;31m⛝\x1b[m".to_owned()
                                } else {
                                    "\x1b[0;31m⛞\x1b[m".to_owned()
                                }
                            }
                        } else {
                            // "⛶ "
                            "[]".to_owned()
                        }
                    } else if self
                        .shot_positions
                        .last()
                        .is_some_and(|&x| x == current_iter_pos)
                    {
                        "⯐".to_owned()
                    } else {
                        "  ".to_owned()
                    }
                };

                stdout
                    .queue(crossterm::cursor::MoveTo(
                        2 * (i + 1 + starting_position.x) as u16,
                        (j + 1 + starting_position.y) as u16,
                    ))
                    .expect("Moving the cursor should work")
                    .queue(crossterm::style::Print(cell))
                    .expect("Printing should work");
            }
        }

        stdout.flush().expect("Should be able to flush");
    }

    pub fn render_enemy_pov(&self, starting_position: Vector2, cursor: Option<&Cursor>) {
        let mut stdout = stdout();

        for i in -1..11 {
            for j in -1..11 {
                let cell: String = if i == -1 && j == -1 {
                    "╔═".to_owned()
                } else if i == -1 && j == 10 {
                    "╚═".to_owned()
                } else if i == 10 && j == -1 {
                    "╗".to_owned()
                } else if i == 10 && j == 10 {
                    "╝".to_owned()
                } else if i == 10 {
                    if cursor.is_some_and(|cursor| cursor.get_position().y == j) {
                        format!("\x1b[0;33m{}\x1b[0m", j)
                    } else {
                        j.to_string()
                    }
                } else if j == -1 {
                    if cursor.is_some_and(|cursor| cursor.get_position().x == i) {
                        format!("\x1b[0;33m{}\x1b[0m", ('A' as u8 + i as u8) as char)
                    } else {
                        (('A' as u8 + i as u8) as char).to_string()
                    }
                } else if i == -1 {
                    "║".to_owned()
                } else if j == 10 {
                    "══".to_owned()
                } else if cursor.is_some()
                    && cursor.unwrap().get_position() == (Vector2 { x: i, y: j })
                {
                    "○".to_owned()
                } else {
                    let current_iter_pos = Vector2 { x: i, y: j };
                    if self.shot_positions.contains(&current_iter_pos) {
                        let mut ships_flatmap =
                            self.ships.iter().flat_map(|ship| ship.get_segments());
                        if ships_flatmap.any(|segment| segment.get_position() == current_iter_pos) {
                            if self
                                .ships
                                .iter()
                                .find(|&ship| {
                                    ship.get_segments().iter().any(|segment| {
                                        segment.get_position() == current_iter_pos
                                    })
                                })
                                .unwrap()
                                .get_segments()
                                .iter()
                                .all(|segment| {
                                    self.shot_positions.contains(&segment.get_position())
                                })
                            {
                                "\x1b[0;31m⛝\x1b[m".to_owned()
                            } else {
                                "\x1b[0;31m⛞\x1b[m".to_owned()
                            }
                        } else {
                            "⯐".to_owned()
                        }
                    } else {
                        "".to_owned()
                    }
                };

                stdout
                    .queue(crossterm::cursor::MoveTo(
                        2 * (i + 1 + starting_position.x) as u16,
                        (j + 1 + starting_position.y) as u16,
                    ))
                    .expect("Moving the cursor should work")
                    .queue(crossterm::style::Print(cell))
                    .expect("Printing should work");
            }
        }

        stdout.flush().expect("Should be able to flush");
    }

    pub fn get_shot_positions(&self) -> &[Vector2] {
        &self.shot_positions
    }

    pub fn get_ships(&self) -> &[Ship] {
        &self.ships
    }


    pub fn shot(&mut self, position: Vector2) {
        self.shot_positions.push(position);
    }
}
