mod board;
mod cursor;
mod direction;
mod ship;
mod vector2;
use crossterm::{
    event::{read, Event, KeyCode, KeyEventKind, KeyModifiers},
    terminal::enable_raw_mode,
    QueueableCommand,
};
use vector2::Vector2;

fn main() {
    let mut manager: GameManager = GameManager::new();

    enable_raw_mode().expect("Should be able to enable raw terminal mode");
    GameManager::clear_screen();

    loop {
        manager.frame();

        if let Ok(event_read) = read() {
            match event_read {
                Event::Key(event) if event.kind == KeyEventKind::Press => {
                    if (event.code == KeyCode::Char('c') || event.code == KeyCode::Char('C'))
                        && event.modifiers == KeyModifiers::CONTROL
                    {
                        return;
                    }

                    manager.on_click(event.code);
                }
                _ => {}
            }
        }
    }
}

struct GameManager {
    shot_this_turn: bool,
    curtain: bool,
    p2_turn: bool,
    p1_board: board::Board,
    p2_board: board::Board,
    cursor: cursor::Cursor,
    setup_phase: bool,
    held_ship_index: Option<usize>,
}

impl GameManager {
    fn new() -> GameManager {
        GameManager {
            shot_this_turn: false,
            curtain: false,
            p2_turn: false,
            p1_board: board::Board::new(),
            p2_board: board::Board::new(),
            cursor: cursor::Cursor::new(),
            setup_phase: true,
            held_ship_index: None,
        }
    }

    fn clear_screen() {
        let mut stdout = std::io::stdout();
        crossterm::QueueableCommand::queue(
            &mut stdout,
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
        )
        .unwrap()
        .queue(crossterm::cursor::MoveTo(0, 0))
        .unwrap()
        .queue(crossterm::cursor::Hide)
        .unwrap();
        std::io::Write::flush(&mut stdout).unwrap();
    }

    fn get_current_board(&self) -> &board::Board {
        if self.p2_turn {
            &self.p2_board
        } else {
            &self.p1_board
        }
    }

    fn get_enemy_board(&self) -> &board::Board {
        if self.p2_turn {
            &self.p1_board
        } else {
            &self.p2_board
        }
    }

    fn get_current_board_mut(&mut self) -> &mut board::Board {
        if self.p2_turn {
            &mut self.p2_board
        } else {
            &mut self.p1_board
        }
    }

    fn get_enemy_board_mut(&mut self) -> &mut board::Board {
        if self.p2_turn {
            &mut self.p1_board
        } else {
            &mut self.p2_board
        }
    }

    fn frame(&mut self) {
        let mut stdout = std::io::stdout();

        if self.curtain {
            GameManager::clear_screen();

            stdout
                .queue(crossterm::cursor::MoveTo(0, 0))
                .expect("Moving the cursor should work")
                .queue(crossterm::style::Print(format!(
                    "Tura gracza {}",
                    if self.p2_turn { 2 } else { 1 }
                )))
                .expect("Printing should work")
                .queue(crossterm::cursor::MoveTo(0, 1))
                .expect("Moving the cursor should work")
                .queue(crossterm::style::Print("[Kliknij dowolny przycisk]"))
                .expect("Printing should work");

            std::io::Write::flush(&mut stdout).expect("Should be able to flush");
        } else if self
            .p1_board
            .get_ships()
            .iter()
            .flat_map(|x| x.get_segments())
            .all(|seg| {
                self.p1_board
                    .get_shot_positions()
                    .contains(&seg.get_position())
            })
        {
            GameManager::clear_screen();
            stdout
                .queue(crossterm::cursor::MoveTo(0, 0))
                .expect("Moving the cursor should work")
                .queue(crossterm::style::Print("Gracz 2 Wygrał"))
                .expect("Printing should work")
                .queue(crossterm::cursor::MoveTo(0, 1))
                .expect("Moving the cursor should work")
                .queue(crossterm::style::Print("Gracz 1:"))
                .expect("Printing should work")
                .queue(crossterm::cursor::MoveTo(30, 1))
                .expect("Moving the cursor should work")
                .queue(crossterm::style::Print("Gracz 2:"))
                .expect("Printing should work");

            self.p1_board.shot(Vector2 { x: 69, y: 69 });
            self.p2_board.shot(Vector2 { x: 69, y: 69 });

            self.p1_board
                .render_your_pov(Vector2 { x: 0, y: 2 }, None, None);
            self.p2_board
                .render_your_pov(Vector2 { x: 15, y: 2 }, None, None);

            std::io::Write::flush(&mut stdout).expect("Should be able to flush");

            std::process::exit(0);
        } else if self
            .p2_board
            .get_ships()
            .iter()
            .flat_map(|x| x.get_segments())
            .all(|seg| {
                self.p2_board
                    .get_shot_positions()
                    .contains(&seg.get_position())
            })
        {
            GameManager::clear_screen();

            stdout
                .queue(crossterm::cursor::MoveTo(0, 0))
                .expect("Moving the cursor should work")
                .queue(crossterm::style::Print("Gracz 1 Wygrał"))
                .expect("Printing should work")
                .queue(crossterm::cursor::MoveTo(0, 1))
                .expect("Moving the cursor should work")
                .queue(crossterm::style::Print("Gracz 1:"))
                .expect("Printing should work")
                .queue(crossterm::cursor::MoveTo(30, 1))
                .expect("Moving the cursor should work")
                .queue(crossterm::style::Print("Gracz 2:"))
                .expect("Printing should work");

            self.p1_board.shot(Vector2 { x: 69, y: 69 });
            self.p2_board.shot(Vector2 { x: 69, y: 69 });

            self.p1_board
                .render_your_pov(Vector2 { x: 0, y: 2 }, None, None);
            self.p2_board
                .render_your_pov(Vector2 { x: 15, y: 2 }, None, None);

            std::io::Write::flush(&mut stdout).expect("Should be able to flush");

            std::process::exit(0);
        } else {
            if self.setup_phase {
                stdout
                    .queue(crossterm::cursor::MoveTo(0, 13))
                    .expect("Moving the cursor should work")
                    .queue(crossterm::style::Print(
                        "Uzyj strzalek aby poruszac kursorem\r\n\
                        Uzyj ENTER aby chwycic statek i ustawic go na miejsce\r\n\
                        Uzyj r/R aby obrocic chwycony statek\r\n\
                        Uzyj CTRL-C aby zakonczyc dzialanie programu\r\n\
                        Uzyj ESCAPE aby zakonczyc ustawianie swoich statkow i oddac ture dla przeciwnika",
                    ))
                    .expect("Printing should work");
            } else {
                stdout
                    .queue(crossterm::cursor::MoveTo(0, 13))
                    .expect("Moving the cursor should work")
                    .queue(crossterm::style::Print(
                        "Uzyj strzalek aby poruszac kursorem\r\n\
                        Uzyj ENTER aby strzlic\r\n\
                        Uzyj CTRL-C aby zakonczyc dzialanie programu\r\n\
                        \r\n\
                        Bialy celownik oznacza pudlo\r\n\
                        Czerwony przekreslony kwadrat oznacza trafienie\r\n\
                        Czerwony zkrzyżowany kwadrat oznacza zatopienie"
                    ))
                    .expect("Printing should work");
            }

            self.get_current_board().render_your_pov(
                Vector2 { x: 0, y: 0 },
                if self.setup_phase {
                    Some(&self.cursor)
                } else {
                    None
                },
                self.held_ship_index,
            );
            self.get_enemy_board().render_enemy_pov(
                Vector2 { x: 15, y: 0 },
                if !self.setup_phase && !self.shot_this_turn {
                    Some(&self.cursor)
                } else {
                    None
                },
            );
        }
    }

    fn on_click(&mut self, key: KeyCode) {
        if self.curtain {
            self.curtain = false;
            GameManager::clear_screen();
        } else if self.setup_phase {
            match key {
                KeyCode::Up => {
                    self.cursor.up();
                    self.move_held_ship();
                }
                KeyCode::Down => {
                    self.cursor.down();
                    self.move_held_ship();
                }
                KeyCode::Left => {
                    self.cursor.left();
                    self.move_held_ship();
                }
                KeyCode::Right => {
                    self.cursor.right();
                    self.move_held_ship();
                }
                KeyCode::Enter => {
                    if self.held_ship_index.is_some() {
                        let mut invalid_segments: Vec<Vector2> = Vec::new();

                        //out of bounds
                        invalid_segments.append(
                            &mut self
                                .get_current_board()
                                .get_ships()
                                .iter()
                                .flat_map(|ship| ship.get_segments())
                                .map(|seg| seg.get_position())
                                .filter(|pos| pos.x < 0 || pos.x > 9 || pos.y < 0 || pos.y > 9)
                                .collect(),
                        );

                        // duplicates
                        invalid_segments.append(
                            &mut self
                                .get_current_board()
                                .get_ships()
                                .iter()
                                .flat_map(|ship| ship.get_segments())
                                .enumerate()
                                .map(|(index, seg)| (index, seg.get_position()))
                                .filter(|(outer_index, outer_seg)| {
                                    self.get_current_board()
                                        .get_ships()
                                        .iter()
                                        .flat_map(|ship| ship.get_segments())
                                        .enumerate()
                                        .map(|(index, seg)| (index, seg.get_position()))
                                        .filter(|(index, _)| outer_index != index)
                                        .filter(|(_, seg)| seg == outer_seg)
                                        .count()
                                        > 0
                                })
                                .map(|(_, seg)| seg)
                                .collect(),
                        );

                        //too close
                        let mut comb_ships: Vec<(Vec<Vector2>, Vec<Vector2>)> = Vec::new();
                        for k in 0..self.get_current_board().get_ships().len() {
                            let mut to_cmp: Vec<Vector2> = Vec::new();
                            for l in 0..self.get_current_board().get_ships().len() {
                                if k == l {
                                    continue;
                                }

                                to_cmp.append(
                                    &mut self.get_current_board().get_ships()[l]
                                        .get_segments()
                                        .iter()
                                        .map(|seg| seg.get_position())
                                        .collect(),
                                );
                            }
                            comb_ships.push((
                                self.get_current_board().get_ships()[k]
                                    .get_segments()
                                    .iter()
                                    .map(|seg| seg.get_position())
                                    .collect(),
                                to_cmp,
                            ))
                        }

                        for (left, right) in comb_ships {
                            'pos_comparisons: for left_position in left {
                                for x in (left_position.x - 1)..(left_position.x + 2) {
                                    for y in (left_position.y - 1)..(left_position.y + 2) {
                                        if x < 0 || x > 9 || y < 0 || y > 9 {
                                            continue;
                                        }

                                        if right.contains(&Vector2 { x, y }) {
                                            invalid_segments.push(left_position);
                                            continue 'pos_comparisons;
                                        }
                                    }
                                }
                            }
                        }

                        if !self.get_current_board().get_ships()[self.held_ship_index.unwrap()]
                            .get_segments()
                            .iter()
                            .any(|seg| invalid_segments.contains(&seg.get_position()))
                        {
                            self.held_ship_index = None;
                        }
                    } else {
                        let ship_on_cursor = self
                            .get_current_board()
                            .get_ships()
                            .iter()
                            .enumerate()
                            .find(|&(_, ship)| {
                                ship.get_segments()
                                    .iter()
                                    .any(|seg| seg.get_position() == self.cursor.get_position())
                            });

                        if let Some((index, ship)) = ship_on_cursor {
                            self.cursor
                                .set_position(ship.get_segments()[0].get_position());
                            self.held_ship_index = Some(index);
                            self.move_held_ship();
                        }
                    }
                }
                KeyCode::Char('r' | 'R') => {
                    let ship_on_cursor = self
                        .get_current_board()
                        .get_ships()
                        .iter()
                        .enumerate()
                        .find(|&(_, ship)| {
                            ship.get_segments()
                                .iter()
                                .any(|seg| seg.get_position() == self.cursor.get_position())
                        });

                    if ship_on_cursor.is_some() {
                        if let Some(held_ship_index) = self.held_ship_index {
                            self.get_current_board_mut()
                                .get_ships_mut()
                                .get_mut(held_ship_index)
                                .expect(
                                    "Ships are created on program start and index should be valid",
                                )
                                .rotate();
                        }
                    }
                }
                KeyCode::Esc => {
                    self.curtain = true;
                    if self.p2_turn {
                        self.setup_phase = false;
                    }
                    self.p2_turn ^= true;

                    self.cursor.reset();
                }
                _ => (),
            }
        } else if self.shot_this_turn {
            self.shot_this_turn = false;

            if !self
                .get_enemy_board()
                .get_ships()
                .iter()
                .flat_map(|x| x.get_segments())
                .any(|seg| {
                    self.get_enemy_board()
                        .get_shot_positions()
                        .last()
                        .is_some_and(|x| x == &seg.get_position())
                })
            {
                self.curtain = true;
                self.cursor.reset();
                self.p2_turn ^= true;
            }
        } else {
            match key {
                KeyCode::Up => self.cursor.up(),
                KeyCode::Down => self.cursor.down(),
                KeyCode::Left => self.cursor.left(),
                KeyCode::Right => self.cursor.right(),
                KeyCode::Enter => {
                    let position = self.cursor.get_position();

                    if !(*self.get_enemy_board())
                        .get_shot_positions()
                        .iter()
                        .any(|&x| x == position)
                    {
                        self.get_enemy_board_mut().shot(position);
                        self.shot_this_turn = true;
                        // self.p2_turn ^= true;
                        // self.cursor.reset();
                    }
                }
                _ => {}
            }
        }
    }

    fn move_held_ship(&mut self) {
        let pos = self.cursor.get_position();

        if let Some(held_ship_index) = self.held_ship_index {
            self.get_current_board_mut()
                .get_ships_mut()
                .get_mut(held_ship_index)
                .expect("Ships are created on program start and index should be valid")
                .move_to(pos);
        }
    }
}
