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
