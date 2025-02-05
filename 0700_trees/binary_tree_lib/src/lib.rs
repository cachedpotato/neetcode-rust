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

pub struct CursorMut<'a, T> {
    cur: Link<T>,
    tree: &'a mut BinaryTree<T>,
}

impl<'a, T> CursorMut<'a, T> {
    pub fn has_siblings(&mut self) -> bool {
        unsafe {
            if self.cur.is_none() {
                return false;
            }
            (*self.cur.unwrap().as_ptr()).right.is_some()
                || (*self.cur.unwrap().as_ptr()).left.is_some()
        }
    }

    pub fn move_left(&mut self) {
        unsafe {
            if let Some(node) = self.cur {
                self.cur = (*node.as_ptr()).left;
            } else {
                //fallback: return to head
                self.cur = self.tree.head;
            }
        }
    }

    pub fn move_right(&mut self) {
        unsafe {
            if let Some(node) = self.cur {
                self.cur = (*node.as_ptr()).right;
            } else {
                //fallback: return to head
                self.cur = self.tree.head;
            }
        }
    }
}

impl<T: Ord + Eq> BinaryTree<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            depth: 0,
            size: 0,
        }
    }

    pub fn cursor_mut(&mut self) -> CursorMut<'_, T> {
        CursorMut {
            cur: None,
            tree: self,
        }
    }

    pub fn add(&mut self, elem: T) {
        unsafe {
            let new = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                elem,
                left: None,
                right: None,
            })));
            if self.head.is_none() {
                self.head = Some(new);
                self.size += 1;
                self.depth += 1;
                return;
            }
            let mut m = self.cursor_mut();
            while let Some(node) = m.cur {
                if !m.has_siblings() {
                    let val = &(*node.as_ptr()).elem;
                    if &(*new.as_ptr()).elem < val {
                        (*node.as_ptr()).left = Some(new);
                    } else {
                        (*node.as_ptr()).right = Some(new);
                    }
                    self.size += 1;
                    self.depth += 1;
                    return;
                }
            }
            unreachable!();
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
