use std::mem;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Node<T> {
    pub data: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T, next: Option<Box<Node<T>>>) -> Node<T> {
        Node { data, next }
    }
}

#[derive(Default)]
pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        fn count<T>(node: &Option<Box<Node<T>>>) -> Option<usize> {
            match node {
                None => None,
                Some(node) if !&node.next.is_some() => Some(1),
                Some(node) => count(&node.next).map(|r| r + 1),
            }
        }

        match count(&self.head) {
            None => 0,
            Some(n) => n,
        }
    }

    pub fn push(&mut self, element: T) {
        self.head = Some(Box::new(Node::new(
            element,
            mem::replace(&mut self.head, None),
        )));
    }

    pub fn pop(&mut self) -> Option<T> {
        match mem::replace(&mut self.head, None) {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.data)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match self.head {
            None => None,
            Some(ref node) => Some(&node.data),
        }
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        let mut result: SimpleLinkedList<T> = SimpleLinkedList::new();
        let mut node_opt = self.head.clone();
        while let Some(node) = node_opt {
            result.push(node.data);
            node_opt = node.next
        }
        result
    }
}

impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(item: &[T]) -> Self {
        let mut result: SimpleLinkedList<T> = SimpleLinkedList::new();
        for i in item.iter() {
            result.push(i.clone());
        }
        result
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut result: Vec<T> = vec![];
        let mut node_opt = self.head;
        while let Some(node) = node_opt {
            result.push(node.data);
            node_opt = node.next
        }
        result.reverse();
        result
    }
}
