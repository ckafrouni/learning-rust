use std::fmt::Debug;

mod iterator;

pub use iterator::LinkedListIter;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let list: LinkedList<i32> = LinkedList::new();
        assert_eq!(list.length(), 0);
    }

    #[test]
    fn test_push() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.length(), 3);
    }

    #[test]
    fn test_pop() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
        assert_eq!(list.length(), 0);
    }

    #[test]
    fn test_get() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.get(0), Some(&1));
        assert_eq!(list.get(1), Some(&2));
        assert_eq!(list.get(2), Some(&3));
        assert_eq!(list.get(3), None);
    }

    #[test]
    fn test_get_mut() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.get_mut(0), Some(&mut 1));
        assert_eq!(list.get_mut(1), Some(&mut 2));
        assert_eq!(list.get_mut(2), Some(&mut 3));
        assert_eq!(list.get_mut(3), None);
    }

    #[test]
    fn test_set() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.set(0, 4), Some(1));
        assert_eq!(list.set(1, 5), Some(2));
        assert_eq!(list.set(2, 6), Some(3));
        assert_eq!(list.set(3, 7), None);
        assert_eq!(list.get(0), Some(&4));
        assert_eq!(list.get(1), Some(&5));
        assert_eq!(list.get(2), Some(&6));
    }
}