mod iterator;

pub struct BinarySearchTree<T> {
    root: Option<Box<Node<T>>>,
}

#[derive(Debug)]
pub struct Node<T> {
    data: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> BinarySearchTree<T>
where
    T: PartialOrd,
{
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            left: None,
            right: None,
        });

        let mut node = &mut self.root;

        while let Some(next) = node {
            if new_node.data < next.data {
                node = &mut next.left;
            } else {
                node = &mut next.right;
            }
        }

        *node = Some(new_node);
    }

    pub fn contains(&self, data: T) -> bool {
        let mut node = &self.root;

        while let Some(next) = node {
            if data == next.data {
                return true;
            } else if data < next.data {
                node = &next.left;
            } else {
                node = &next.right;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert() {
        let mut bst = BinarySearchTree::new();

        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);
        bst.insert(6);
        bst.insert(8);

        assert_eq!(bst.contains(5), true);
        assert_eq!(bst.contains(3), true);
        assert_eq!(bst.contains(7), true);
        assert_eq!(bst.contains(2), true);
        assert_eq!(bst.contains(4), true);
        assert_eq!(bst.contains(6), true);
        assert_eq!(bst.contains(8), true);
        assert_eq!(bst.contains(9), false);
    }

    #[test]
    fn iter() {
        let mut bst = BinarySearchTree::new();

        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);
        bst.insert(6);
        bst.insert(8);

        let mut iter = bst.iter();

        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), Some(&6));
        assert_eq!(iter.next(), Some(&7));
        assert_eq!(iter.next(), Some(&8));
        assert_eq!(iter.next(), None);
    }
}