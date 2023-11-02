use super::{LinkedList, Node};


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