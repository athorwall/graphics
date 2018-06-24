pub struct Frame<T> {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<T>,
}

impl <T: Copy> Frame<T> {
    pub fn new(width: usize, height: usize, value: T) -> Frame<T> {
        let cells = vec![value; width * height];
        return Frame{cells, width, height};
    }

    pub fn width(&self) -> usize {
        return self.width;
    }

    pub fn height(&self) -> usize {
        return self.height;
    }

    pub fn at(&self, x: usize, y: usize) -> Option<T> {
        return self.cells.get(self.width * y + x).map(|a| *a);
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        self.cells[self.width * y + x] = value;
    }

    pub fn set_all(&mut self, value: T) {
        self.cells.iter_mut().for_each(|x| *x = value);
    }

    pub fn cells(&self) -> &Vec<T> {
        return &self.cells;
    }
}