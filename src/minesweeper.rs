use crate::random::random_range;
use std::{
    collections::HashSet,
    fmt::{Display, Write},
};

pub type Position = (usize, usize);

pub enum OpenResult {
    Mine,
    NoMine(u8),
}

pub struct Minesweeper {
    width: usize,
    height: usize,
    open_fields: HashSet<Position>,
    mines: HashSet<Position>,
    flagged_fields: HashSet<Position>,
    lost: bool,
}

impl Display for Minesweeper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let position = (x, y);

                if !self.open_fields.contains(&position) {
                    if self.lost && self.mines.contains(&position) {
                        f.write_str("💣 ")?;
                    } else if self.flagged_fields.contains(&position) {
                        f.write_str("⛳️ ")?;
                    } else {
                        f.write_str("🟪 ")?;
                    }
                } else if self.mines.contains(&position) {
                    f.write_str("💣 ")?;
                } else {
                    let mine_count = self.neighboring_mines(position);

                    if mine_count > 0 {
                        write!(f, " {} ", mine_count)?;
                    } else {
                        f.write_str("◻️ ")?;
                    }
                }
            }

            f.write_char('\n')?;
        }

        Ok(())
    }
}

impl Minesweeper {
    pub fn new(width: usize, height: usize, mine_count: usize) -> Minesweeper {
        Minesweeper {
            width,
            height,
            open_fields: HashSet::new(),
            mines: {
                let mut mines = HashSet::new();

                while mines.len() < mine_count {
                    mines.insert((random_range(0, width), random_range(0, height)));
                }

                mines
            },
            flagged_fields: HashSet::new(),
            lost: false,
        }
    }

    pub fn iter_neighbors(&self, (x, y): Position) -> impl Iterator<Item = Position> {
        let width = self.width;
        let height = self.height;

        (x.max(1) - 1..=(x + 1).min(width - 1))
            .flat_map(move |i| (y.max(1) - 1..=(y + 1).min(height - 1)).map(move |j| (i, j)))
            .filter(move |&pos| pos != (x, y))
    }

    pub fn neighboring_mines(&self, position: Position) -> u8 {
        self.iter_neighbors(position)
            .filter(|position| self.mines.contains(position))
            .count() as u8
    }

    pub fn open(&mut self, position: Position) -> Option<OpenResult> {
        if self.lost
            || self.open_fields.contains(&position)
            || self.flagged_fields.contains(&position)
        {
            return None;
        }

        self.open_fields.insert(position);

        let is_mine = self.mines.contains(&position);

        if is_mine {
            self.lost = true;
            Some(OpenResult::Mine)
        } else {
            let mine_count = self.neighboring_mines(position);

            if mine_count == 0 {
                for neighbor in self.iter_neighbors(position) {
                    self.open(neighbor);
                }
            }

            Some(OpenResult::NoMine(mine_count))
        }
    }

    pub fn toggle_flag(&mut self, position: Position) {
        if self.lost || self.open_fields.contains(&position) {
            return;
        }

        if self.flagged_fields.contains(&position) {
            self.flagged_fields.remove(&position);
        } else {
            self.flagged_fields.insert(position);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Minesweeper;

    #[test]
    fn test() {
        let mut ms = Minesweeper::new(10, 10, 5);
        ms.open((5, 5));
        ms.toggle_flag((6, 6));
        ms.open((6, 6));

        println!("{}", ms);
    }
}
