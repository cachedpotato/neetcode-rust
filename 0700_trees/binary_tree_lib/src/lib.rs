use std::{collections::VecDeque, ptr::NonNull};

pub struct BinaryTree<T> {
    root: Link<T>,
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
            root: None,
            size: 0,
            depth: 0,
        }
    }

    // "flatten" the binary tree into a vec
    // in BFS order
    pub fn flatten(self) -> Vec<T> {
        let mut out = Vec::<T>::new();
        if self.is_empty() {
            return out;
        }

        unsafe {
            let mut cur_level = VecDeque::new();
            cur_level.push_back(self.root);
            //BFS
            while !cur_level.is_empty() {
                let l = cur_level.len();
                for _ in 0..l {
                    let cur = Box::from_raw(cur_level.pop_front().unwrap().unwrap().as_ptr());
                    if cur.left.is_some() {
                        cur_level.push_back(cur.left);
                    }
                    if cur.right.is_some() {
                        cur_level.push_back(cur.right);
                    }
                    out.push(cur.elem);
                }
            }
        }
        out
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
        self.root.map(|node| unsafe { &(*node.as_ptr()).elem })
    }
}

impl<T: PartialOrd> BinaryTree<T> {
    pub fn is_binary_search_tree(&self) -> bool {
        //edge case: empty tree
        if self.is_empty() {
            return true;
        }

        unsafe { Node::helper(&(*self.root.as_ref().unwrap().as_ptr())) }
    }
}

impl<T: PartialOrd> Node<T> {
    fn helper(node: &Node<T>) -> bool {
        //helper function for is_binary_tree
        //edge case 1: no siblings
        if node.right.is_none() && node.left.is_none() {
            return true;
        }
        unsafe {
            if node.right.is_some() && node.left.is_some() {
                let r = node.right.map(|rnode| &(*rnode.as_ptr()).elem).unwrap();
                let c = &node.elem;
                let l = node.left.map(|rnode| &(*rnode.as_ptr()).elem).unwrap();
                if l < c && c < r {
                    Node::helper(&(*node.left.as_ref().unwrap().as_ptr()))
                        && Node::helper(&(*node.right.as_ref().unwrap().as_ptr()))
                } else {
                    false
                }
            } else {
                false
            }
        }
    }
}
d
impl<T> Default for BinaryTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

// Not all BT is BST
// Make BST as a trait of BT
pub trait BinarySearchTree<T> {
    fn binary_search(&self, elem: T) -> bool;
    fn bst_add(&mut self, elem: T);
}

impl<T: PartialEq + PartialOrd + std::fmt::Debug> BinarySearchTree<T> for BinaryTree<T> {
    fn binary_search(&self, target: T) -> bool {
        let mut cur = self.root.as_ref().unwrap();
        unsafe {
            // Binary search algorithm
            // At current: compare target with self.elem
            // if match: return true
            // if target smaller: move left
            // if target bigger: move right
            // return false if respective sides are empty
            // loop ends if there if no siblings left to compare
            while (*cur.as_ptr()).left.is_some() || (*cur.as_ptr()).right.is_some() {
                let curr_elem = &(*cur.as_ptr()).elem;
                if curr_elem == &target {
                    return true;
                } else if curr_elem > &target {
                    //move to left
                    if (*cur.as_ptr()).left.is_some() {
                        cur = (*cur.as_ptr()).left.as_ref().unwrap();
                    } else {
                        return false;
                    }
                } else {
                    //move to right
                    if (*cur.as_ptr()).right.is_some() {
                        cur = (*cur.as_ptr()).right.as_ref().unwrap();
                    } else {
                        return false;
                    }
                }
            }
            //final node check
            (*cur.as_ptr()).elem == target
        }
    }

    fn bst_add(&mut self, target: T) {
        //pretty much the same as binary_search with one additional step
        //if there's a hit, do nothing
        //if there isn't add to tree
        unsafe {
            let mut new = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                elem: target,
                left: None,
                right: None,
            })));

            if self.is_empty() {
                self.root = Some(new);
                self.size += 1;
                self.depth += 1;
                return;
            }

            let mut cur = self.root.as_ref().unwrap();
            let mut curr_elem = &(*cur.as_ptr()).elem;
            let comp = &(*new.as_ptr()).elem;
            let mut depth = 1;
            // Binary search algorithm
            // At current: compare target with self.elem
            // if match: return true
            // if target smaller: move left
            // if target bigger: move right
            // return false if respective sides are empty
            // loop ends if there if no siblings left to compare
            todo!()
        }
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
                    acc.root = Some(new);
                    acc.depth += 1;
                } else {
                    let mut nodes = VecDeque::new();
                    nodes.push_back(acc.root.as_mut());
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
    use crate::BinarySearchTree;

    use super::BinaryTree;

    #[test]
    fn creation_from_iterable() {
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
        assert_eq!(bt.flatten(), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn bst() {
        let bt = BinaryTree::from_iter([1, 2, 3]);
        assert!(!bt.is_binary_search_tree());
        let bst = BinaryTree::from_iter([2, 1, 3]);
        assert!(bst.is_binary_search_tree());
        let mut bst = BinaryTree::from_iter([10, 5, 11, 3, 6]);
        assert!(bst.is_binary_search_tree());
        assert!(bst.binary_search(10));
        assert!(bst.binary_search(11));
        assert!(bst.binary_search(6));
        assert!(!bst.binary_search(7));

        //adding to bst
        bst.bst_add(1);
        bst.bst_add(13);
        assert_eq!(bst.flatten(), vec![10, 5, 11, 3, 6, 13, 1]);
        assert_eq!(bst.depth(), 4);
        assert_eq!(bst.size(), 7);
    }
}
