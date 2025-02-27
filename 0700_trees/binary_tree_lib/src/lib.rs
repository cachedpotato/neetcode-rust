use std::{collections::VecDeque, fmt::Debug, marker::PhantomData, ptr::NonNull};

pub struct BinaryTree<T> {
    root: Link<T>,
    size: usize,
    depth: usize,
}

type Link<T> = Option<NonNull<Node<T>>>; //fuck it we ball

#[derive(Debug, Clone)]
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

    pub fn custom_create(input: Vec<Option<T>>) -> Self {
        // EX: [Some(1), None, Some(2), Some(3), Some(4), Some(5)]
        //                          [Some(1)]
        //
        //           [None]                     [Some(2)]
        //
        //                              [Some(3)]      [Some(4)]
        //
        //                     [Some(5)]
        //

        unsafe {
            for elem in input {
                let mut new = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                    elem,
                    left: None,
                    right: None,
                })));

                // BFS With a twist
                // if we hit none, then we need to stop creating sibling nodes
            }
        }
        todo!()
    }

    // "flatten" the binary tree into a vec
    // in BFS order
    pub fn flatten(self) -> VecDeque<T> {
        let mut out = VecDeque::<T>::new();
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
                    out.push_back(cur.elem);
                }
            }
        }
        out
    }

    pub fn dfs(&self) -> Vec<&T> {
        // Non-recursive, stack dfs
        // In-Order
        let mut stack = Vec::new();
        unsafe {
            stack.push(self.root.as_ref().unwrap());
            loop {
                let curr = stack[0];
                if (*curr.as_ptr()).left.is_some() {
                    stack.push((*curr.as_ptr()).left.as_ref().unwrap());
                    continue;
                }
                break;
            }
        }
        todo!()
    }

    // getting referecnes of nth (in BFS order) node
    // zero indexing obviously
    // or is one indexing more intuitive here idk
    pub fn nth(&self, n: usize) -> Option<&T> {
        if self.size < n {
            return None;
        }
        let mut deque = VecDeque::new();
        deque.push_back(self.root.as_ref());
        let mut idx = 0;
        unsafe {
            loop {
                // BFS logic
                let curr = deque.pop_front().unwrap().unwrap();
                if n == idx {
                    return Some(&(*curr.as_ptr()).elem);
                }

                if (*curr.as_ptr()).left.is_some() {
                    deque.push_back((*curr.as_ptr()).left.as_ref());
                }
                if (*curr.as_ptr()).right.is_some() {
                    deque.push_back((*curr.as_ptr()).right.as_ref());
                }
                idx += 1;
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

    pub fn root_node(&self) -> Option<&T> {
        self.root.map(|node| unsafe { &(*node.as_ptr()).elem })
    }

    // iterators
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            root: self.root,
            size: self.size,
            idx: None,
            _phantom: PhantomData,
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            root: self.root,
            size: self.size,
            idx: None,
            _phantom: PhantomData,
        }
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

// ITERATORS
// Because tree in and of itself is a goddamn pain to iterate over
// First flatten it into a vec
// then iterate over that

// 1. Iter
pub struct Iter<'a, T> {
    root: Link<T>,
    size: usize,
    idx: Option<usize>,
    _phantom: PhantomData<&'a T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.root.is_none() {
                None
            } else if self.idx.is_none() {
                self.idx = Some(0);
                self.root.map(|node| &(*node.as_ptr()).elem)
            } else {
                let mut deque = VecDeque::new();
                deque.push_back(self.root.as_ref());
                let mut at = 0;
                let target = self.idx.unwrap() + 1;
                if target >= self.size {
                    return None;
                }
                loop {
                    // BFS logic
                    let curr = deque.pop_front().unwrap().unwrap();
                    if at == target {
                        self.idx = Some(target);
                        return Some(&(*curr.as_ptr()).elem);
                    }
                    if (*curr.as_ptr()).left.is_some() {
                        deque.push_back((*curr.as_ptr()).left.as_ref());
                    }
                    if (*curr.as_ptr()).right.is_some() {
                        deque.push_back((*curr.as_ptr()).right.as_ref());
                    }
                    at += 1;
                }
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.size, Some(self.size))
    }
}

impl<T> ExactSizeIterator for Iter<'_, T> {
    fn len(&self) -> usize {
        self.size
    }
}

impl<'a, T> IntoIterator for &'a BinaryTree<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

// 2. IterMut
// LESSON LEARNED: DO NOT CREATE ITERATORS THAT BORROW ITEMS FROM ITSELF
// IT WILL CAUSE ALL SORTS OF JANK
pub struct IterMut<'a, T> {
    root: Link<T>,
    size: usize,
    idx: Option<usize>,
    _phantom: PhantomData<&'a mut T>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.root.is_none() {
                None
            } else if self.idx.is_none() {
                self.idx = Some(0);
                self.root.map(|node| &mut (*node.as_ptr()).elem)
            } else {
                let mut deque = VecDeque::new();
                deque.push_back(self.root.as_ref());
                let mut at = 0;
                let target = self.idx.unwrap() + 1;
                if target >= self.size {
                    return None;
                }
                loop {
                    // BFS logic
                    let curr = deque.pop_front().unwrap().unwrap();
                    if at == target {
                        self.idx = self.idx.map(|n| n + 1);
                        return Some(&mut (*curr.as_ptr()).elem);
                    }
                    if (*curr.as_ptr()).left.is_some() {
                        deque.push_back((*curr.as_ptr()).left.as_ref());
                    }
                    if (*curr.as_ptr()).right.is_some() {
                        deque.push_back((*curr.as_ptr()).right.as_ref());
                    }
                    at += 1;
                }
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.size, Some(self.size))
    }
}

impl<T> ExactSizeIterator for IterMut<'_, T> {
    fn len(&self) -> usize {
        self.size
    }
}

impl<'a, T> IntoIterator for &'a mut BinaryTree<T> {
    type IntoIter = IterMut<'a, T>;
    type Item = &'a mut T;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

// 3. IntoIter
pub struct IntoIter<T> {
    tree: VecDeque<T>,
}

impl<T> BinaryTree<T> {
    pub fn to_into_iter(self) -> IntoIter<T> {
        IntoIter {
            tree: self.flatten(),
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.tree.pop_front()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.tree.len(), Some(self.tree.len()))
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {
    fn len(&self) -> usize {
        self.tree.len()
    }
}

impl<T> IntoIterator for BinaryTree<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.to_into_iter()
    }
}

// USEFUL TRAITS
impl<T> Default for BinaryTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Debug> Debug for BinaryTree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self).finish()
    }
}

impl<T: Clone> Clone for BinaryTree<T> {
    fn clone(&self) -> Self {
        let mut out = Self::new();
        for e in self {
            out.add(e.clone());
        }
        out
    }
}

impl<T: PartialEq> PartialEq for BinaryTree<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.size != other.size || self.depth != other.depth {
            return false;
        }
        let mut si = self.iter();
        let mut oi = other.iter();
        for _i in 0..self.size {
            if si.next() != oi.next() {
                return false;
            }
        }
        true
    }
}

impl<T: PartialOrd> PartialOrd for BinaryTree<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.size().cmp(&other.size()))
    }
}

// Not all BT is BST
// Make BST as a trait of BT
pub trait BinarySearchTree<T> {
    fn binary_search(&self, elem: T) -> bool;
    fn bst_add(&mut self, elem: T);
}

impl<T: PartialEq + PartialOrd> BinarySearchTree<T> for BinaryTree<T> {
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

    fn bst_add(&mut self, elem: T) {
        //pretty much the same as binary_search with one additional step
        //if there's a hit, do nothing
        //if there isn't add to tree
        unsafe {
            let new = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                elem,
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
            let mut curr_elem;
            let comp = &(*new.as_ptr()).elem;
            let mut depth = 1;

            while (*cur.as_ptr()).left.is_some() || (*cur.as_ptr()).right.is_some() {
                curr_elem = &(*cur.as_ptr()).elem;
                if curr_elem == comp {
                    //no need to add
                    return;
                }
                if curr_elem > comp {
                    //left side
                    if (*cur.as_ptr()).left.is_some() {
                        //move left
                        cur = (*cur.as_ptr()).left.as_ref().unwrap();
                    } else {
                        //append here
                        (*cur.as_ptr()).left = Some(new);
                        self.depth = std::cmp::max(self.depth, depth + 1);
                        self.size += 1;
                        return;
                    }
                } else {
                    //right side
                    if (*cur.as_ptr()).right.is_some() {
                        cur = (*cur.as_ptr()).right.as_ref().unwrap();
                    } else {
                        (*cur.as_ptr()).right = Some(new);
                        self.depth = std::cmp::max(self.depth, depth + 1);
                        self.size += 1;
                        return;
                    }
                }
                depth += 1;
            }
            //check final node (no sibling)
            curr_elem = &(*cur.as_ptr()).elem;
            if curr_elem == comp {
                return;
                //do nothing
            } else if curr_elem > comp {
                //append to left
                (*cur.as_ptr()).left = Some(new);
            } else {
                //append to right
                (*cur.as_ptr()).left = Some(new);
            }
            self.depth = std::cmp::max(self.depth, depth + 1);
            self.size += 1;
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

//THIS IS GONNA BE HARD TO READ GODDAMN
//
// NEETCODE QUESTIONS
// 1. INVERT BINARY TREE
impl<T> Node<T> {
    fn reverse(&mut self) {
        //recursive
        unsafe {
            std::mem::swap(&mut self.left, &mut self.right);
            if let Some(left) = self.left {
                (*left.as_ptr()).reverse();
            }
            if let Some(right) = self.right {
                (*right.as_ptr()).reverse();
            }
        }
    }
}

impl<T> BinaryTree<T> {
    pub fn reverse_tree(&mut self) {
        unsafe {
            if let Some(node) = self.root {
                (*node.as_ptr()).reverse();
            }
        }
    }
}

// 2. Maximum Depth
// I'm out of depth on this one.
// I'll see myself out.
//

// 3. Balanced Binary Tree
impl<T> BinaryTree<T> {
    pub fn is_balanced(&self) -> bool {
        //easy edge cases
        if self.root.is_none() {
            false
        } else {
            unsafe { Node::is_balanced(&(*self.root.as_ref().unwrap().as_ptr())) }
        }
    }
}
impl<T> Node<T> {
    pub fn dfs(node: &Self, depth: &mut i32, curr: &mut i32) {
        unsafe {
            if node.left.is_none() && node.right.is_none() {
                //do nothing
            } else {
                *depth = std::cmp::max(*depth, *curr);
                if node.left.is_some() {
                    Self::dfs(
                        &*node.left.as_ref().unwrap().as_ptr(),
                        depth,
                        &mut (*curr + 1),
                    );
                }

                if node.right.is_some() {
                    Self::dfs(
                        &*node.right.as_ref().unwrap().as_ptr(),
                        depth,
                        &mut (*curr + 1),
                    );
                }
            }
        }
    }

    pub fn is_balanced(&self) -> bool {
        unsafe {
            if self.left.is_none() && self.right.is_none() {
                true
            } else if self.left.is_some() && self.right.is_none() {
                (*self.left.as_ref().unwrap().as_ptr()).left.is_none()
                    && (*self.left.as_ref().unwrap().as_ptr()).right.is_none()
            } else if self.left.is_none() && self.right.is_some() {
                (*self.right.as_ref().unwrap().as_ptr()).left.is_none()
                    && (*self.right.as_ref().unwrap().as_ptr()).right.is_none()
            } else {
                let mut l_depth = 0;
                let mut r_depth = 0;
                let mut curr = 0;
                Node::dfs(
                    &(*self.left.as_ref().unwrap().as_ptr()),
                    &mut l_depth,
                    &mut curr,
                );
                curr = 0;
                Node::dfs(
                    &(*self.right.as_ref().unwrap().as_ptr()),
                    &mut r_depth,
                    &mut curr,
                );

                if l_depth - r_depth > -2 && l_depth - r_depth < 2 {
                    (*self.left.as_ref().unwrap().as_ptr()).is_balanced()
                        && (*self.right.as_ref().unwrap().as_ptr()).is_balanced()
                } else {
                    false
                }
            }
        }
    }
}

// 4. Is same
impl<T: Eq> BinaryTree<T> {
    pub fn is_same_tree(&self, other: &Self) -> bool {
        for (i, j) in self.iter().zip(other) {
            if i != j {
                return false;
            }
        }
        true
    }
}

// 5. is subtree
impl<T: Eq> BinaryTree<T> {
    pub fn is_subtree(&self, other: &Self) -> bool {
        if self.root.is_none() && other.root.is_some() {
            false
        } else if self.root.is_none() && other.root.is_none() {
            true
        } else {
            unsafe {
                (*self.root.as_ref().unwrap().as_ptr())
                    .contains(&(*other.root.as_ref().unwrap().as_ptr()))
            }
        }
    }
}

impl<T: Eq> Node<T> {
    fn contains(&self, other: &Self) -> bool {
        let mut stack = vec![];
        unsafe {
            stack.push(self);
            while let Some(curr) = stack.pop() {
                if curr.is_same(other) {
                    return true;
                }
                if let Some(right) = curr.right {
                    stack.push(&(*right.as_ptr()));
                }
                if let Some(left) = curr.left {
                    stack.push(&(*left.as_ptr()));
                }
            }
            false
        }
    }

    fn is_same(&self, other: &Self) -> bool {
        if self.elem != other.elem {
            false
        } else {
            unsafe {
                let left = self.left;
                let other_left = other.left;
                let right = self.right;
                let other_right = other.right;

                if left.is_none() && other_left.is_none()
                    || right.is_none() && other_right.is_none()
                {
                    if let Some(node) = left {
                        (*node.as_ptr()).is_same(&(*other_left.as_ref().unwrap().as_ptr()))
                    } else {
                        (*right.as_ref().unwrap().as_ptr())
                            .is_same(&(*right.as_ref().unwrap().as_ptr()))
                    }
                } else {
                    //check both
                    (*left.as_ref().unwrap().as_ptr())
                        .is_same(&(*other_left.as_ref().unwrap().as_ptr()))
                        && (*right.as_ref().unwrap().as_ptr())
                            .is_same(&(*right.as_ref().unwrap().as_ptr()))
                }
            }
        }
    }
}

// 6. LCM
// Dear lord have mercy
// I need a cursor
impl<T> BinaryTree<T> {
    pub fn lowest_common_ancestor(&self) -> Option<&T> {
        todo!()
    }
}

// 7. Level order Traversal

impl<T> BinaryTree<T> {
    pub fn level_order_traversal(tree: &Self) -> Vec<Vec<Option<&T>>> {
        //BFS with vectos for each depth
        let mut out = vec![];
        unsafe {
            if tree.is_empty() {
                //do nothing
            } else {
                let mut q = VecDeque::new();
                q.push_back(tree.root.as_ref());
                while !q.is_empty() {
                    let mut level = vec![];
                    let l = q.len();
                    for _ in 0..l {
                        let curr = q.pop_front().unwrap().unwrap();
                        level.push(Some(&(*curr.as_ptr()).elem));
                        if (*curr.as_ptr()).left.is_some() {
                            q.push_back((*curr.as_ptr()).left.as_ref());
                        }
                        if (*curr.as_ptr()).right.is_some() {
                            q.push_back((*curr.as_ptr()).right.as_ref());
                        }
                    }
                    out.push(level);
                }
            }
            out
        }
    }
}

#[cfg(test)]
mod test {
    use std::{thread, time::Duration};

    use crate::BinarySearchTree;

    use super::BinaryTree;

    #[test]
    fn creation_from_iterable() {
        thread::sleep(Duration::from_secs(10));
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
        assert_eq!(bst.depth(), 4);
        assert_eq!(bst.size(), 7);
        assert_eq!(bst.flatten(), vec![10, 5, 11, 3, 6, 13, 1]);
    }

    #[test]
    fn iterators() {
        let bt = BinaryTree::<i32>::new();
        assert_eq!(bt.iter().next(), None);

        let mut bt = BinaryTree::from_iter([1, 2, 3]);
        let mut bt_iter = bt.iter();
        assert_eq!(bt_iter.next(), Some(&1));
        assert_eq!(bt_iter.next(), Some(&2));
        assert_eq!(bt_iter.next(), Some(&3));
        assert_eq!(bt_iter.next(), None);

        for n in &mut bt {
            *n += 1;
        }
        assert_eq!(bt.flatten(), vec![2, 3, 4]);

        let bt = BinaryTree::from_iter([1, 2, 3]);
        let a = bt.into_iter().map(|n| n + 1).collect::<Vec<i32>>();
        assert_eq!(a, vec![2, 3, 4]);
    }

    #[test]
    fn bfs() {
        let bt = BinaryTree::from_iter([1, 2, 3, 4, 5, 6, 7]);
        let vecs = BinaryTree::level_order_traversal(&bt);
        assert_eq!(
            vecs,
            vec![
                vec![Some(&1)],
                vec![Some(&2), Some(&3)],
                vec![Some(&4), Some(&5), Some(&6), Some(&7)]
            ]
        );
    }
}
