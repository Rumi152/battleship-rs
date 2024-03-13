mod board;
mod cursor;
mod direction;
mod ship;
mod vector2;
use crossterm::{
    event::{read, Event, KeyCode, KeyEventKind},
    QueueableCommand,
};
use ship::ShipSegment;
use vector2::Vector2;

fn main() {
    let mut manager: GameManager = GameManager::new();

    loop {
        if let Ok(event_read) = read() {
            match event_read {
                Event::Key(event) => {
                    if event.kind == KeyEventKind::Press {
                        manager.on_click(event.code);
                    }
                }
                _ => {}
            }
        }

        manager.frame();
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
            setup_phase: false,
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
        GameManager::clear_screen();
        let mut stdout = std::io::stdout();

        if self
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
            stdout
                .queue(crossterm::cursor::MoveTo(0, 0))
                .expect("Moving the cursor should work")
                .queue(crossterm::style::Print("Gracz 2 Wygrał"))
                .expect("Printing should work");

            std::io::Write::flush(&mut stdout).expect("Should be able to flush");
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
            stdout
                .queue(crossterm::cursor::MoveTo(0, 0))
                .expect("Moving the cursor should work")
                .queue(crossterm::style::Print("Gracz 1 Wygrał"))
                .expect("Printing should work");

            std::io::Write::flush(&mut stdout).expect("Should be able to flush");
        } else if self.curtain {
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
        } else {
            self.get_current_board().render_your_pov(
                Vector2 { x: 0, y: 0 },
                if self.setup_phase {
                    Some(&self.cursor)
                } else {
                    None
                },
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

            return;
        }

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
