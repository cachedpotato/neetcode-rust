use std::{collections::VecDeque, ptr::NonNull};

pub struct BinaryTree<T> {
    head: Link<T>,
    size: usize,
    depth: usize,
}

type Link<T> = Option<NonNull<Node<T>>>; //fuck it we ball

struct Node<T> {
    elem: T,
    left: Link<T>,
    right: Link<T>,
}

// NOT a binary SEARCH tree
impl<T> BinaryTree<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            size: 0,
            depth: 0,
        }
    }

    pub fn add(&mut self, elem: T) {
        unsafe {
            let new = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                elem,
                left: None,
                right: None,
            })));

            if let Some(node) = self.head {
                let mut nodes = VecDeque::new();
                nodes.push_back(Some(node));
                loop {
                    let l = nodes.len();
                    let mut new_depth = VecDeque::new();

                    for _ in 0..l {
                        let cur = nodes.pop_front().unwrap().unwrap();
                        if (*cur.as_ptr()).left.is_some() {
                            new_depth.push_back((*cur.as_ptr()).left);
                        }
                        if (*cur.as_ptr()).right.is_some() {
                            new_depth.push_back((*cur.as_ptr()).right);
                        }
                    }

                    if new_depth.is_empty() {
                        break;
                    }
                }
            } else {
                self.head = Some(new);
                self.size += 1;
                self.depth += 1;
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn depth(&self) -> usize {
        self.depth
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

impl<T> Default for BinaryTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    //use super::BinaryTree;

    #[test]
    fn basics() {
        todo!();
    }
}
