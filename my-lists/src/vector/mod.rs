use std::fmt::{Debug, Display, Formatter, Result};

mod iterator;
pub use iterator::VectorIter;

pub struct Vector<T> {
    data: Box<[Option<T>]>,
    length: usize,
    capacity: usize,
}

const DEFAULT_CAPACITY: usize = 4;

impl<T> Vector<T> {
    pub fn new() -> Self {
        Self {
            data: Self::allocate_buffer(DEFAULT_CAPACITY),
            length: 0,
            capacity: DEFAULT_CAPACITY,
        }
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn push(&mut self, value: T) {
        self.resize();
        self.data[self.length] = Some(value);
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.length == 0 {
            return None;
        }
        self.length -= 1;
        let value = self.data[self.length].take();
        self.resize();
        value
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.length {
            return None;
        }
        self.data[index].as_ref()
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.length {
            return None;
        }
        self.data[index].as_mut()
    }

    pub fn set(&mut self, index: usize, value: T) -> Option<T> {
        if index >= self.length {
            return None;
        }
        self.data[index].replace(value)
    }

    pub fn insert(&mut self, index: usize, value: T) {
        if index > self.length {
            panic!("Index out of bounds");
        }
        self.resize();
        for i in (index..self.length).rev() {
            self.data[i + 1] = self.data[i].take();
        }
        self.data[index] = Some(value);
        self.length += 1;
    }

    fn resize(&mut self) {
        if self.length == self.capacity {
            // resize up
            self.capacity *= 2;
        } else if self.length == self.capacity / 4 && self.capacity > DEFAULT_CAPACITY {
            // resize down
            self.capacity /= 2;
        }
        self.reallocate()
    }

    fn reallocate(&mut self) {
        let mut new_data = Self::allocate_buffer(self.capacity);
        for i in 0..self.length {
            new_data[i] = self.data[i].take();
        }
        self.data = new_data;
    }

    fn allocate_buffer(capacity: usize) -> Box<[Option<T>]> {
        std::iter::repeat_with(|| None)
            .take(capacity)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }
}


impl<T> Debug for Vector<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "[")?;

        for i in 0..self.length {
            write!(f, "{}", self.data[i].as_ref().unwrap())?;

            if i < self.length - 1 {
                write!(f, ", ")?;
            }
        }

        write!(f, "]")
    }
}
