use std::{collections::VecDeque, ptr::NonNull, usize};

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

    // "flatten" the binary tree into a vec
    // in BFS order
    pub fn flatten(self) -> Vec<T> {
        todo!()
    }

    pub fn add(&mut self, elem: T) {
        todo!()
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

    pub fn root_node(&self) -> Option<&T> {
        self.head.map(|node| unsafe { &(*node.as_ptr()).elem })
    }
}

impl<T> Default for BinaryTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> FromIterator<T> for BinaryTree<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        iter.into_iter().fold(BinaryTree::new(), |mut acc, x| {
            unsafe {
                //BFS add
                let new = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                    elem: x,
                    left: None,
                    right: None,
                })));

                if acc.is_empty() {
                    acc.head = Some(new);
                    acc.depth += 1;
                } else {
                    let mut nodes = VecDeque::new();
                    nodes.push_back(acc.head.as_mut());
                    let mut cur_depth = 0;

                    //BFS with deque
                    'outer: loop {
                        let l = nodes.len();
                        for _ in 0..l {
                            cur_depth += 1;
                            // check if there's sibling nodes
                            // if vacant, fill in the void with the new node and return
                            // if full, add to next depth nodes
                            // prioritze left over right as per BT
                            let cur = nodes.pop_front().unwrap().unwrap();
                            if (*cur.as_ptr()).left.is_none() {
                                (*cur.as_ptr()).left = Some(new);
                                cur_depth += 1;
                                break 'outer;
                            } else if (*cur.as_ptr()).right.is_none() {
                                cur_depth += 1;
                                (*cur.as_ptr()).right = Some(new);
                                break 'outer;
                            }
                            nodes.push_back((*cur.as_ptr()).left.as_mut());
                            nodes.push_back((*cur.as_ptr()).right.as_mut());
                        }
                    }
                    acc.depth = cur_depth;
                }
                acc.size += 1;
                acc
            }
        })
    }
}

#[cfg(test)]
mod test {
    //use super::BinaryTree;

    use crate::BinaryTree;

    #[test]
    fn basics() {
        todo!();
    }

    #[test]
    fn from_iter() {
        let bt: BinaryTree<i32> = BinaryTree::new();
        assert!(bt.is_empty());
        assert!(bt.size() == 0);
        assert!(bt.depth() == 0);

        let bt = BinaryTree::from_iter([1]);
        assert!(!bt.is_empty());
        assert!(bt.size() == 1);
        assert!(bt.depth() == 1);

        let bt = BinaryTree::from_iter([1, 2, 3]);
        assert_eq!(bt.size(), 3);
        assert_eq!(bt.root_node(), Some(&1));
        assert_eq!(bt.depth(), 2);

        let bt = BinaryTree::from_iter([1, 2, 3, 4, 5]);
        assert_eq!(bt.size(), 5);
        assert_eq!(bt.root_node(), Some(&1));
        assert_eq!(bt.depth(), 3);
    }
}
