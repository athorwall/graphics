
pub struct Frame<T> {
    cells: Vec<Vec<T>>,
}

impl <T: Copy> Frame<T> {
    pub fn new(width: usize, height: usize, value: T) -> Frame<T> {
        let row = vec![value; width];
        return Frame{cells: vec![row; height]};
    }

    pub fn width(&self) -> usize {
        return self.cells.get(0).map(|row| row.len()).unwrap_or(0);
    }

    pub fn height(&self) -> usize {
        return self.cells.len();
    }

    pub fn at(&self, x: usize, y: usize) -> Option<T> {
        return self.cells.get(y).and_then(|v| v.get(x)).map(|v| *v);
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        match self.cells.get_mut(y) {
            Some(row) => row[x] = value,
            None => {},
        }
    }
}