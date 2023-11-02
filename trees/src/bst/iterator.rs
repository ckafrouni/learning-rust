use std::collections::VecDeque;

use super::{BinarySearchTree, Node};

pub struct BSTIterator<'a, T> {
    stack: VecDeque<&'a Box<Node<T>>>,
}

impl<T> BinarySearchTree<T>
where
    T: PartialOrd,
{
    pub fn iter(&self) -> BSTIterator<T> {
        let mut stack = VecDeque::new();
        let mut current = &self.root;
        while let Some(node) = current {
            stack.push_front(node);
            current = &node.left;
        }
        BSTIterator { stack }
    }
}

impl<'a, T> Iterator for BSTIterator<'a, T>
where
    T: PartialOrd,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.stack.pop_front() {
            let mut current = &node.right;
            while let Some(next) = current {
                self.stack.push_front(next);
                current = &next.left;
            }
            Some(&node.data)
        } else {
            None
        }
    }
}
