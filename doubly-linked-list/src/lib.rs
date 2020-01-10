mod pre_implemented;

use std::ptr::NonNull;

type NodePtr<T> = NonNull<Node<T>>;
type OptNodePtr<T> = Option<NodePtr<T>>;

struct Node<T> {
    prev: OptNodePtr<T>,
    next: OptNodePtr<T>,
    element: T,
}

impl<T> Node<T> {
    unsafe fn new(element: T) -> NodePtr<T> {
        NonNull::new_unchecked(Box::into_raw(Box::new(Self {
            element,
            next: None,
            prev: None,
        })))
    }

    unsafe fn link(mut left: NodePtr<T>, mut right: NodePtr<T>) {
        left.as_mut().next = Some(right);
        right.as_mut().prev = Some(left);
    }
}

#[derive(Default)]
pub struct LinkedList<T> {
    head: OptNodePtr<T>,
    tail: OptNodePtr<T>,
    len: usize,
}

// for advanced tests
unsafe impl<T: Send> Send for LinkedList<T> {}
unsafe impl<T: Sync> Sync for LinkedList<T> {}

pub struct Cursor<'a, T> {
    node: OptNodePtr<T>,
    list: &'a mut LinkedList<T>,
}

pub struct Iter<'a, T> {
    node: Option<NonNull<Node<T>>>,
    _list: std::marker::PhantomData<&'a LinkedList<T>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn cursor_front(&mut self) -> Cursor<'_, T> {
        Cursor {
            node: self.head,
            list: self,
        }
    }

    pub fn cursor_back(&mut self) -> Cursor<'_, T> {
        Cursor {
            node: self.tail,
            list: self,
        }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            node: self.head,
            _list: std::marker::PhantomData,
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut cursor = self.cursor_front();
        while cursor.take().is_some() {}
    }
}

impl<T> Cursor<'_, T> {
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        unsafe { self.node.map(|node| &mut (*node.as_ptr()).element) }
    }

    pub fn next(&mut self) -> Option<&mut T> {
        unsafe { self._step(|node| node.next) }
    }

    pub fn prev(&mut self) -> Option<&mut T> {
        unsafe { self._step(|node| node.prev) }
    }

    unsafe fn _step(&mut self, move_node: impl Fn(&Node<T>) -> OptNodePtr<T>) -> Option<&mut T> {
        self.node = move_node(self.node?.as_ref());
        self.node.map(|n| &mut (*n.as_ptr()).element)
    }

    pub fn take(&mut self) -> Option<T> {
        unsafe {
            let mut node = self.node?;
            let &mut Node { prev, next, .. } = node.as_mut();

            self.node = next.or(prev);
            match next {
                Some(mut next) => next.as_mut().prev = prev,
                None => self.list.tail = prev,
            };
            match prev {
                Some(mut prev) => prev.as_mut().next = next,
                None => self.list.head = next,
            }

            self.list.len -= 1;
            Some(Box::from_raw(node.as_ptr()).element)
        }
    }

    pub fn insert_after(&mut self, element: T) {
        unsafe {
            self._insert(
                element,
                |list| &mut list.tail,
                |mut cursor_node, new_node| {
                    if let Some(next) = cursor_node.as_mut().next {
                        Node::link(new_node, next);
                    }
                    Node::link(cursor_node, new_node);
                },
            );
        }
    }

    pub fn insert_before(&mut self, element: T) {
        unsafe {
            self._insert(
                element,
                |list| &mut list.head,
                |mut cursor_node, new_node| {
                    if let Some(prev) = cursor_node.as_mut().prev {
                        Node::link(prev, new_node);
                    }
                    Node::link(new_node, cursor_node);
                },
            );
        }
    }

    unsafe fn _insert(
        &mut self,
        element: T,
        end_node: impl Fn(&mut LinkedList<T>) -> &mut OptNodePtr<T>,
        link_new_node: impl Fn(NodePtr<T>, NodePtr<T>),
    ) {
        let new_node = Node::new(element);
        if self.node.is_none() {
            self.node = Some(new_node);
            self.list.tail = self.node;
            self.list.head = self.node;
            self.list.len = 1;
        } else {
            let cursor_node = self.node.unwrap();
            link_new_node(cursor_node, new_node);
            let end = end_node(&mut self.list);
            if *end == Some(cursor_node) {
                *end = Some(new_node);
            }
            self.list.len += 1;
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        self.node.map(|node| unsafe {
            self.node = node.as_ref().next;
            &(*node.as_ptr()).element
        })
    }
}
