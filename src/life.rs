use std::fmt;

#[derive(Clone, Copy)]
pub enum Cell {
    Alive,
    Dead,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let token = match self {
            Cell::Alive => "🟪",
            Cell::Dead => "⬛",
        };
        write!(f, "{}", token)
    }
}

pub struct Board {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Board {
        let cells = vec![Cell::Dead; width * height];
        Board {
            width,
            height,
            cells,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Cell {
        let index = y * self.width + x;
        self.cells[index]
    }

    pub fn set(&mut self, x: usize, y: usize, cell: Cell) {
        let index = y * self.width + x;
        self.cells[index] = cell;
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self.get(x, y))?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
