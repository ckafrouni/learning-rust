use std::fmt::Debug;

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    length: usize,
}

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T>
where
    T: Default,
{
    pub fn new() -> Self {
        Self {
            head: None,
            length: 0,
        }
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn push(&mut self, value: T) {
        let new_node = Box::new(Node { value, next: None });
        let mut node = &mut self.head;

        while let Some(next) = node {
            node = &mut next.next;
        }

        *node = Some(new_node);
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        // If the list is empty
        if self.head.is_none() {
            return None;
        }

        // If there's only one node in the list
        if self.head.as_ref().unwrap().next.is_none() {
            let node = self.head.take();
            self.length -= 1;
            return node.map(|node| node.value);
        }

        // If there are two or more nodes in the list
        let mut node = &mut self.head;
        while node.as_ref().unwrap().next.as_ref().unwrap().next.is_some() {
            node = &mut node.as_mut().unwrap().next;
        }

        let last_node = node.as_mut().unwrap().next.take();
        self.length -= 1;
        last_node.map(|node| node.value)
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.length {
            return None;
        }

        let mut node = &self.head;
        for _ in 0..index {
            node = &node.as_ref().unwrap().next;
        }

        node.as_ref().map(|node| &node.value)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.length {
            return None;
        }

        let mut node = &mut self.head;
        for _ in 0..index {
            node = &mut node.as_mut().unwrap().next;
        }

        node.as_mut().map(|node| &mut node.value)
    }

    pub fn set(&mut self, index: usize, value: T) -> Option<T> {
        if index >= self.length {
            return None;
        }

        let mut node = &mut self.head;
        for _ in 0..index {
            node = &mut node.as_mut().unwrap().next;
        }

        node.as_mut().map(|node| std::mem::replace(&mut node.value, value))
    }
}

// The iterator type that will hold a reference to the current node
pub struct LinkedListIter<'a, T> {
    current: Option<&'a Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    // Method to create an iterator for the LinkedList
    pub fn iter(&self) -> LinkedListIter<T> {
        LinkedListIter {
            current: self.head.as_ref(),
        }
    }
}

impl<'a, T> Iterator for LinkedListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|node| {
            self.current = node.next.as_ref();
            &node.value
        })
    }
}

impl<T> Debug for LinkedList<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut node = &self.head;
        write!(f, "[")?;
        while let Some(current) = node {
            if let Some(_) = &current.next {
                write!(f, "{:?}, ", current.value)?;
            } else {
                write!(f, "{:?}", current.value)?;
            }
            node = &current.next;
        }
        write!(f, "]")
    }
}
