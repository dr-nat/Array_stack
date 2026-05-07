struct ArrayList {
   elements: Vec<i32>,
}

impl ArrayList {
    fn get(self, i: usize) {
        self.elements[i];
    }

    fn add(mut self, i: usize, x: i32) {
        if self.elements.capacity() == self.elements.len() {
            self.element.reserve(self.element.len())
        }
    }

    fn remove(self) {}

    fn grow(self) {}

    fn shrink(self) {}
}
