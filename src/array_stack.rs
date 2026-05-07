struct ArrayList {
   elements: Vec<i32>,
}

impl ArrayList {
    fn get(self, i: usize) {
        self.elements[i];
    }

    fn add(mut self, i: usize, x: i32) {
        if self.elements.capacity() == self.elements.len() {
            self.elements.reserve(self.elements.len());

            let top = self.elements.len();
            for index in (i+1..top).rev() {
                self.elements[index + 1] = self.elements[index];
            }
            self.elements[i] = x
        }

    }

    fn remove(self) {}

    fn grow(self) {}

    fn shrink(self) {}
}
