use super::Vector;

pub struct VectorIter<'a, T> {
    vector: &'a Vector<T>,
    index: usize,
}

impl<T> Vector<T> {
    pub fn iter(&self) -> VectorIter<'_, T> {
        println!("Hello, world!");
        VectorIter {
            vector: self,
            index: 0,
        }

    }
}

impl<'a, T> Iterator for VectorIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.vector.length {
            return None;
        }
        let value = self.vector.get(self.index);
        self.index += 1;
        value
    }
}