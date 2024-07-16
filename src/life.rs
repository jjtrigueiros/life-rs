use std::fmt;

#[derive(Clone, Copy, PartialEq)]
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
    pub fn new(width: usize, height: usize) -> Result<Self, &'static str> {
        let array_size = width.checked_mul(height).ok_or("Board size is too big (width * height should fit in usize)")?;
        Ok(
            Board {
                width,
                height,
                cells: vec![Cell::Dead; array_size],
        })
    }

    fn wrap_coordinate(&self, coord: isize, max: usize) -> usize {
        coord.rem_euclid(isize::try_from(max).unwrap()).try_into().unwrap()
    }

    fn wrap_x(&self, x: isize) -> usize { self.wrap_coordinate(x, self.width) }

    fn wrap_y(&self, y: isize) -> usize { self.wrap_coordinate(y, self.height) }

    pub fn index_from_signed(&self, x: isize, y: isize) -> usize {
        let bounded_x = self.wrap_x(x);
        let bounded_y = self.wrap_y(y);
        bounded_y.checked_mul(self.width).unwrap().checked_add(bounded_x).unwrap()
    }

    fn index(&self, x: usize, y: usize) -> usize {
        y.checked_mul(self.width).unwrap().checked_add(x).unwrap()
    }

    pub fn get_from_signed(&self, x: isize, y: isize) -> Result<Cell, &'static str> {
        let idx = self.index_from_signed(x, y);
        Ok(self.cells[idx])
    }

    pub fn get(&self, x: usize, y: usize) -> Cell {
        self.cells[self.index(x, y)]
    }

    pub fn set(&mut self, x: isize, y: isize, cell: Cell) {
        let index = self.index_from_signed(x, y);
        self.cells[index] = cell;
    }

    fn get_neighbors(&self, x: usize, y: usize) -> Vec<Cell> {
        let mut neighbors: Vec<Cell> = Vec::new();
        let neighboring_range: [isize; 3] = [-1, 0, 1];
        for dx in neighboring_range {
            for dy in neighboring_range {
                if dx == 0 && dy == 0 { continue; }
                let neighbor_x = isize::try_from(x).unwrap() + dx;
                let neighbor_y = isize::try_from(y).unwrap() + dy;
                neighbors.push(self.get_from_signed(neighbor_x, neighbor_y).unwrap())
            }
        }
        neighbors

    }

    pub fn count_living_neighbors(&self, x: usize, y: usize) -> u8 {
        let mut count: u8 = 0;
        for neighbor in self.get_neighbors(x, y) {
            if neighbor == Cell::Alive { count += 1 }
        }
        count
    }

    pub fn get_next_state(&mut self) {
        // "https://en.wikipedia.org/wiki/Conway's_Game_of_Life#Rules"
        let mut new_cells = self.cells.clone();
        for y in 0..self.height {
            for x in 0..self.height {
                let cell = self.get(x, y);
                let living_neighbors = self.count_living_neighbors(x, y);
                let new_cell = match (cell, living_neighbors) {
                    (Cell::Alive, 2) | (_, 3) => Cell::Alive,
                    _ => Cell::Dead,
                };
                new_cells[self.index(x, y)] = new_cell;
            }
        }
        self.cells = new_cells
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
