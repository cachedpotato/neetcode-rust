// Custon BTree Lib
//Playing with pointers YET AGAIN Hope I learned something from LinkedList

use std::ptr::NonNull;

pub struct BinaryTree<T> {
    head: Link<T>,
    depth: usize,
    size: usize,
}

type Link<T> = Option<NonNull<Node<T>>>;

struct Node<T> {
    elem: T,
    left: Link<T>,
    right: Link<T>,
}

impl<T: Ord + Eq> BinaryTree<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            depth: 0,
            size: 0,
        }
    }

    pub fn add(&mut self, elem: T) {
        unsafe {
            let new = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                elem,
                left: None,
                right: None,
            })));

            let mut curr = self.head;
            if let Some(node) = curr {
                let mut depth = 1;
                while (*node.as_ptr()).left.is_some() || (*node.as_ptr()).right.is_some() {
                    if (*node.as_ptr()).elem < (*new.as_ptr()).elem {
                        curr = (*node.as_ptr()).left;
                    } else {
                        curr = (*node.as_ptr()).right;
                    }
                    depth += 1;
                }
                if (*node.as_ptr()).elem < (*new.as_ptr()).elem {
                    (*curr.unwrap().as_ptr()).left = Some(new);
                } else {
                    (*curr.unwrap().as_ptr()).right = Some(new);
                }

                self.size += 1;
                self.depth = if self.depth > depth {self.depth} else {depth};

            } else {
                self.head = Some(new);
                self.size += 1;
                self.depth += 1;
            }

        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn depth(&self) -> usize {
        self.depth
    }
}

#[cfg(test)]
mod test {
    use super::BinaryTree;

    #[test]
    fn basics() {
        let mut tree = BinaryTree::<i32>::new();
        assert!(tree.is_empty());
        tree.add(10);
        tree.add(5);
        tree.add(11);
        tree.add(7);
        assert!(!tree.is_empty());
        assert_eq!(tree.depth(), 3);
        assert_eq!(tree.size(), 4);
    }
}