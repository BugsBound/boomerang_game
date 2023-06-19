use crossterm::{
    event::{poll, read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::error::Error;
use std::process::Command;
use std::time::Duration;

pub enum Proportions {
    Height(u8),
    Width(u8),
}

pub struct Field {
    height: u8,
    width: u8,
}

impl Field {
    pub fn new(x: Proportions, y: Proportions) -> Field {
        Field {
            height: Self::check_range(x),
            width: Self::check_range(y),
        }
    }

    fn check_range(proportions: Proportions) -> u8 {
        match proportions {
            Proportions::Height(x) => {
                if !(1..=20).contains(&x) {
                    println!("SetDefaul for height - 10");
                    10
                } else {
                    x
                }
            }
            Proportions::Width(x) => {
                if !(50..=80).contains(&x) {
                    println!("SetDefault for width - 60");
                    60
                } else {
                    x
                }
            }
        }
    }
}

struct Entity {
    position_raw: u8,
    position_col: u8,
    skin: &'static str,
    max_raw: u8,
    max_col: u8,
}

struct Boomerang {
    is_hide: bool,
    is_danger: bool,
    is_fly: bool,
    entity: Entity,
    max_range: u8,
    iterators: u8,
}

impl Boomerang {
    fn new(field: &Field) -> Boomerang {
        let entity = Entity {
            position_raw: 0,
            position_col: 0,
            skin: "🌀",
            max_raw: field.height,
            max_col: field.width,
        };

        Boomerang {
            is_hide: true,
            is_danger: false,
            is_fly: false,
            max_range: 20,
            iterators: 0,
            entity,
        }
    }

    fn fly(&mut self) {
        if self.iterators == 0 {
            self.is_fly = false;
            return;
        }

        if self.is_danger {
            if self.entity.position_col != self.entity.max_col - 1 {
                self.entity.position_col += 1;
            }
            self.iterators -= 1;
        } else {
            self.entity.position_col -= 1;
            self.iterators -= 1;
        }

        if (self.iterators as i32 - self.max_range as i32) < -2 {
            self.is_danger = false;
        }
    }
}

struct Player {
    is_alive: bool,
    has_boomerang: bool,
    entity: Entity,
}

impl Player {
    fn new(field: &Field) -> Player {
        let entity = Entity {
            position_raw: 0,
            position_col: 0,
            skin: "😎",
            max_raw: field.height,
            max_col: field.width,
        };

        Player {
            is_alive: true,
            has_boomerang: true,
            entity,
        }
    }

    fn move_down(&mut self) {
        if self.entity.max_raw - 1 != self.entity.position_raw {
            self.entity.position_raw += 1;
        }
    }

    fn move_up(&mut self) {
        if self.entity.position_raw > 0 {
            self.entity.position_raw -= 1;
        }
    }

    fn move_left(&mut self) {
        if self.entity.position_col > 0 {
            self.entity.position_col -= 1;
        }
    }

    fn move_right(&mut self) {
        if self.entity.max_col - 1 != self.entity.position_col {
            self.entity.position_col += 1;
        }
    }
}

struct Enemy {
    id: u8,
    is_alive: bool,
    entity: Entity,
}

impl Enemy {
    fn new(field: &Field, id: u8) -> Enemy {
        let entity = Entity {
            position_raw: id,
            position_col: field.width - 1,
            skin: "🗿",
            max_raw: field.height,
            max_col: field.width,
        };

        Enemy {
            id,
            is_alive: true,
            entity,
        }
    }

    fn enemy_move(&mut self) {
        if self.is_alive {
            if self.entity.position_col == 0 {
                self.entity.position_col = self.entity.max_col - 1;
            } else {
                self.entity.position_col -= 1;
            }
        } else {
            self.is_alive = true;
            self.entity.position_raw = self.id;
            self.entity.position_col = self.entity.max_col - 1;
        }
    }
}

struct Game {
    is_end: bool,
    player: Player,
    field: Field,
    boomerang: Boomerang,
    enemies: Vec<Enemy>,
}

impl Game {
    fn new() -> Game {
        let field = Self::start_game().unwrap_or_else(|err| {
            println!("{err}");
            std::process::exit(1)
        });

        let player = Player::new(&field);
        let boomerang = Boomerang::new(&field);
        let mut enemies = Vec::new();

        for i in 0..field.height {
            enemies.push(Enemy::new(&field, i))
        }

        Game {
            is_end: false,
            field,
            player,
            boomerang,
            enemies,
        }
    }

    fn set_data(x: &str) -> Result<u8, &'static str> {
        println!("Enter {}", x);
        let mut num = String::new();

        if std::io::stdin().read_line(&mut num).is_err() {
            return Err("Can't read")
        }

        let num: u8 = match num.trim().parse() {
            Ok(n) => n,
            Err(_) => return Err("Need num < 255"),
        };

        Ok(num)
    }

    fn start_game() -> Result<Field, &'static str> {
        println!("BUMERANG!");

        Ok(Field::new(
            Proportions::Height(Self::set_data("height")?),
            Proportions::Width(Self::set_data("width")?),
        ))
    }

    fn draw(&self) {
        clear_console(true);
        let mut field = vec![vec![" "; self.field.width.into()]; self.field.height.into()];
        let height = field.len();
        let width = field[0].len();

        if !self.boomerang.is_hide {
            field[self.boomerang.entity.position_raw as usize]
                [self.boomerang.entity.position_col as usize] = self.boomerang.entity.skin;
        }

        field[self.player.entity.position_raw as usize][self.player.entity.position_col as usize] =
            self.player.entity.skin;

        let top = "=".repeat(width + 2);

        for enemy in &self.enemies {
            field[enemy.entity.position_raw as usize][enemy.entity.position_col as usize] =
                enemy.entity.skin;
        }

        println!("{top}");

        for item in field.iter().take(height) {
            println!("|{}|", item.join(""));
        }

        println!("{top}");
    }

    fn play(&mut self) -> Result<(), Box<dyn Error>> {
        clear_console(true);
        self.draw();

        let mut count = 0.0;
        loop {
            if self.is_end || !self.player.is_alive {
                println!("GAME OVER!");
                disable_raw_mode()?;
                break;
            }

            clear_console(true);

            if self.boomerang.is_fly {
                self.boomerang.fly();
            }

            for enemy in &mut self.enemies {
                if self.boomerang.is_danger
                    && enemy.entity.position_raw == self.boomerang.entity.position_raw
                    && (self.boomerang.entity.position_col == enemy.entity.position_col
                        || self.boomerang.entity.position_col + 1 == enemy.entity.position_col
                        || self.boomerang.entity.position_col + 2 == enemy.entity.position_col
                        || self.boomerang.entity.position_col + 3 == enemy.entity.position_col
                        || self.boomerang.entity.position_col + 4 == enemy.entity.position_col)
                {
                    enemy.is_alive = false;
                } else {
                    enemy.enemy_move();
                }

                if self.player.entity.position_raw == enemy.entity.position_raw
                    && self.player.entity.position_col == enemy.entity.position_col
                {
                    self.player.is_alive = false;
                }
            }

            self.draw();
            count += 0.1;
            let formatted = format!("{:.1}", count);
            println!("Time: {}", formatted);

            enable_raw_mode()?;
            if poll(Duration::from_millis(100))? {
                let ev = read()?;

                if let Event::Key(event) = ev { match event.code {
                        KeyCode::Char('q') => {
                            self.is_end = true;
                        }
                        KeyCode::Char('w') => {
                            self.player.move_up();
                        }
                        KeyCode::Char('s') => {
                            self.player.move_down();
                        }
                        KeyCode::Char('a') => {
                            self.player.move_left();
                        }
                        KeyCode::Char('d') => {
                            self.player.move_right();
                        }
                        KeyCode::Char(' ') => {
                            if self.player.has_boomerang {
                                self.player.has_boomerang = false;

                                self.boomerang.is_danger = true;
                                self.boomerang.is_fly = true;
                                self.boomerang.is_hide = false;

                                if self.player.entity.position_col + 2 < self.player.entity.max_col
                                {
                                    self.boomerang.entity.position_col =
                                        self.player.entity.position_col + 2;
                                } else {
                                    self.boomerang.entity.position_col =
                                        self.player.entity.position_col;
                                }

                                self.boomerang.entity.position_raw =
                                    self.player.entity.position_raw;
                                self.boomerang.iterators = 38;
                            }
                        }
                    _ => {}
                } }

                if !self.boomerang.is_hide
                    && !self.player.has_boomerang
                    && !self.boomerang.is_fly
                    && self.boomerang.entity.position_col == self.player.entity.position_col
                    && self.boomerang.entity.position_raw == self.player.entity.position_raw
                {
                    self.boomerang.is_hide = true;
                    self.player.has_boomerang = true;
                    self.boomerang.entity.position_col = 0;
                    self.boomerang.entity.position_raw = 0;
                }
            }
            disable_raw_mode()?;
        }
        Ok(())
    }
}

fn clear_console(is_unix: bool) {
    if is_unix {
        // Для Unix-подобных систем (Linux, macOS)
        let _ = Command::new("clear").status();
    } else {
        // Для Windows
        let _ = Command::new("cls").status();
    }
}

fn main() {
    clear_console(true);

    let mut game = Game::new();
    game.play().expect("SOMETHING WRONG!");
}
