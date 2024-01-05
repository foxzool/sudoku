pub struct Cell {
    index: usize,
}

impl Cell {
    pub fn new(index: usize) -> Self {
        Self { index }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn row(&self) -> usize {
        self.index / 9
    }

    pub fn col(&self) -> usize {
        self.index % 9
    }

    pub fn block(&self) -> usize {
        (self.row() / 3) * 3 + (self.col() / 3)
    }
}
