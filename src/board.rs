use crate::consts;

const ROWS: usize = consts::ROWS as usize;
const COLUMNS: usize = consts::COLUMNS as usize;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Outcome {
    None,
    Win(f32, f32, f32, f32),
    Draw,
}

impl Outcome {
    pub fn is_none(self) -> bool {
        self == Outcome::None
    }
}

#[derive(Debug)]
pub struct FullColumn;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Slot {
    Empty,
    Red,
    Yellow,
}

impl Slot {
    pub fn is_empty(&self) -> bool {
        *self == Slot::Empty
    }

    pub fn next_player(self) -> Slot {
        match self {
            Slot::Yellow => Slot::Red,
            Slot::Red => Slot::Yellow,
            _ => panic!(),
        }
    }
}

pub type Board = [[Slot; COLUMNS]; ROWS];

pub trait BoardTrait {
    fn new() -> Self;
    fn insert(&mut self, column: usize, player: Slot) -> Result<(), FullColumn>;
    fn get_outcome(&self) -> Outcome;
    fn clear(&mut self);
}

impl BoardTrait for Board {
    fn new() -> Self {
        [[Slot::Empty; COLUMNS]; ROWS]
    }

    fn clear(&mut self) {
        *self = Self::new();
    }

    fn insert(&mut self, column: usize, player: Slot) -> Result<(), FullColumn> {
        for i in (0..ROWS).rev() {
            if self[i][column].is_empty() {
                self[i][column] = player;
                return Ok(());
            }
        }
        Err(FullColumn)
    }

    fn get_outcome(&self) -> Outcome {
        for i in 0..ROWS {
            for j in 0..COLUMNS {
                let slotij = self[i][j];
                if slotij.is_empty() {
                    continue;
                }
                if i < ROWS - 3 {
                    if j < COLUMNS - 3 {
                        // Check left-up to right-down diagonal
                        if slotij == self[i + 1][j + 1]
                            && slotij == self[i + 2][j + 2]
                            && slotij == self[i + 3][j + 3]
                        {
                            return Outcome::Win(
                                j as f32,
                                i as f32,
                                j as f32 + 3.0,
                                i as f32 + 3.0,
                            );
                        }
                    }
                    if j >= 3 {
                        // Check right-up to left-down diagonal
                        if slotij == self[i + 1][j - 1]
                            && slotij == self[i + 2][j - 2]
                            && slotij == self[i + 3][j - 3]
                        {
                            return Outcome::Win(
                                j as f32,
                                i as f32,
                                j as f32 - 3.0,
                                i as f32 + 3.0,
                            );
                        }
                    }
                    // Check Vertical
                    if slotij == self[i + 1][j]
                        && slotij == self[i + 2][j]
                        && slotij == self[i + 3][j]
                    {
                        return Outcome::Win(j as f32, i as f32, j as f32, i as f32 + 3.0);
                    }
                }
                if j < COLUMNS - 3 {
                    // Check Horizontal
                    if slotij == self[i][j + 1]
                        && slotij == self[i][j + 2]
                        && slotij == self[i][j + 3]
                    {
                        return Outcome::Win(j as f32, i as f32, j as f32 + 3.0, i as f32);
                    }
                }
            }
        }

        // Check if there's an empty slot on the top row
        for j in 0..COLUMNS {
            if self[0][j].is_empty() {
                return Outcome::None;
            }
        }

        Outcome::Draw
    }
}
