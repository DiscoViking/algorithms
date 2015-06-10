use std::cmp::Ordering;

#[derive(Debug)]
pub struct BinaryTree<T> where T: Ord + Copy {
    val: Option<T>,
    left: Option<Box<BinaryTree<T>>>,
    right: Option<Box<BinaryTree<T>>>,
}

impl<T> BinaryTree<T> where T: Ord + Copy {
    pub fn new() -> BinaryTree<T> {
        BinaryTree{
            val: None,
            left: None,
            right: None,
        }
    }

    /// Insert an item into the tree.
    /// If a matching value already existed in the tree,
    /// returns an error containing the existing value.
    pub fn insert(&mut self, val: T) -> Result<T, T> {
        // Deal with the empty case first. 
        if self.val.is_none() {
            self.val = Some(val);
            return Ok(val);
        }

        // Get the right subtree to put the value in.
        let mut subtree = match val.cmp(self.val.as_ref().unwrap()) {
            Ordering::Less => &mut self.left,
            Ordering::Greater => &mut self.right,
            Ordering::Equal => return Err(self.val.unwrap()),
        };

        match *subtree {
            None => {
                // No subtree, create one containing the value.
                let mut t = BinaryTree::<T>::new();
                t.val = Some(val);
                *subtree = Some(Box::new(t));
            },
            Some(ref mut t) => {
                // Recursively insert into subtree.
                try!(t.insert(val));
            },
        };

        Ok(val)
    }

    // Removes matching node from the tree.
    // Returns it if successful, errors with given value otherwise.
    pub fn remove(&mut self, val: T) -> Result<T, T> {
        // Deal with the empty case first.
        if self.val.is_none() {
            return Err(val);
        }

        let result = match val.cmp(self.val.as_ref().unwrap()) {
            Ordering::Less => {
                match self.left {
                    None => Err(val),
                    Some(ref mut t) => t.remove(val),
                }
            }

            Ordering::Greater => {
                match self.right {
                    None => Err(val),
                    Some(ref mut t) => t.remove(val),
                }
            }

            Ordering::Equal => {
                let result = Ok(self.val.take().unwrap());

                let (new_val, new_left, new_right) = match self {
                    // No subtrees.
                    // Tree is empty.
                    &mut BinaryTree{left: None, right: None, ..} => {
                        (None, None, None)
                    },

                    // Only one subtree.
                    // Just replace this node with it.
                    &mut BinaryTree{left: None, right: Some(ref mut t), ..} |
                    &mut BinaryTree{left: Some(ref mut t), right: None, ..} => {
                        (t.val.take(), t.left.take(), t.right.take())
                    },

                    // Have both subtrees.
                    // This is the difficult case.
                    &mut BinaryTree{
                        left: Some(ref mut l),
                        right: Some(_),
                        ..
                    } => {
                        let v = l.collapse_rightmost();
                        let copy = BinaryTree{val: l.val.take(), left: l.left.take(), right: l.right.take()};
                        (v, Some(Box::new(copy)), self.right.take())
                    },
                };

                self.val = new_val;
                self.left = new_left;
                self.right = new_right;

                result
            }
        };

        self.prune();

        result
    }

    // Remove the rightmost value in tree and return its value.
    fn collapse_rightmost(&mut self) -> Option<T> {
        let val = match self.right {
            None => self.val.take(),
            Some(ref mut t) => t.collapse_rightmost(),
        };

        self.prune();

        val
    }

    // Deletes any empty children.
    fn prune(&mut self) {
        let del_left = match self.left {
            None => false,
            Some(ref t) => t.val.is_none(),
        };

        let del_right = match self.right {
            None => false,
            Some(ref t) => t.val.is_none(),
        };

        if del_left {
            self.left = None;
        }
        if del_right {
            self.right = None;
        }
    }
}

#[test]
fn insert() {
    let mut t = BinaryTree::<i32>::new();
    assert!(t.insert(3).is_ok());
    assert!(t.insert(5).is_ok());
    assert_eq!(t.val.unwrap(), 3);
    assert_eq!(t.right.as_ref().unwrap().val.unwrap(), 5);
}

#[test]
fn duplicate() {
    let mut t = BinaryTree::<i32>::new();
    assert!(t.insert(3).is_ok());
    assert!(t.insert(3).is_err());
    assert!(t.insert(3).is_err());
    assert!(t.insert(5).is_ok());
    assert!(t.insert(5).is_err());
    assert!(t.insert(3).is_err());
    assert!(t.insert(5).is_err());
}

#[test]
fn remove_no_children() {
    let mut t = BinaryTree::<i32>::new();
    // Set the tree up how we want.
    assert!(t.insert(3).is_ok());
    assert!(t.insert(5).is_ok());

    // Check it's how we expect.
    assert_eq!(t.val.unwrap(), 3);
    assert_eq!(t.right.as_ref().unwrap().val.unwrap(), 5);

    // Remove the leaf node.
    assert!(t.remove(5).is_ok());

    // Check the root no longer points anywhere.
    assert!(t.left.is_none());
    assert!(t.right.is_none());

    // Remove the root.
    assert!(t.remove(3).is_ok());

    // Check it's now empty.
    assert!(t.val.is_none());
}

#[test]
fn remove_left_child() {
    let mut t = BinaryTree::<i32>::new();
    // Set the tree up how we want.
    assert!(t.insert(3).is_ok());
    assert!(t.insert(1).is_ok());

    // Check it's how we expect.
    assert_eq!(t.val.unwrap(), 3);
    assert_eq!(t.left.as_ref().unwrap().val.unwrap(), 1);

    // Remove the root node.
    assert!(t.remove(3).is_ok());

    // Check the root now contains 5, and nothing else.
    assert_eq!(t.val.unwrap(), 1);
    assert!(t.left.is_none());
    assert!(t.right.is_none());
}

#[test]
fn remove_right_child() {
    let mut t = BinaryTree::<i32>::new();
    // Set the tree up how we want.
    assert!(t.insert(3).is_ok());
    assert!(t.insert(5).is_ok());

    // Check it's how we expect.
    assert_eq!(t.val.unwrap(), 3);
    assert_eq!(t.right.as_ref().unwrap().val.unwrap(), 5);

    // Remove the root node.
    assert!(t.remove(3).is_ok());

    // Check the root now contains 5, and nothing else.
    assert_eq!(t.val.unwrap(), 5);
    assert!(t.left.is_none());
    assert!(t.right.is_none());
}

#[test]
fn remove_both_children() {
    let mut t = BinaryTree::<i32>::new();
    // Set the tree up how we want.
    assert!(t.insert(3).is_ok());
    assert!(t.insert(5).is_ok());
    assert!(t.insert(1).is_ok());

    // Check it's how we expect.
    assert_eq!(t.val.unwrap(), 3);
    assert_eq!(t.left.as_ref().unwrap().val.unwrap(), 1);
    assert_eq!(t.right.as_ref().unwrap().val.unwrap(), 5);

    // Remove the root node.
    assert!(t.remove(3).is_ok());

    // Check the tree is now how we expect.
    assert_eq!(t.val.unwrap(), 1);
    assert_eq!(t.right.as_ref().unwrap().val.unwrap(), 5);
    assert!(t.left.is_none());
}

#[test]
fn remove_nonexistent() {
    let mut t = BinaryTree::<i32>::new();
    assert!(t.remove(14).is_err());
    assert!(t.insert(3).is_ok());
    assert!(t.insert(5).is_ok());
    assert!(t.insert(1).is_ok());
    assert!(t.remove(14).is_err());
    assert!(t.remove(0).is_err());
    assert!(t.insert(14).is_ok());
    assert!(t.remove(14).is_ok());
}

#[test]
fn remove_left() {
    let mut t = BinaryTree::<i32>::new();
    // Set the tree up how we want.
    assert!(t.insert(3).is_ok());
    assert!(t.insert(5).is_ok());
    assert!(t.insert(1).is_ok());

    assert!(t.remove(1).is_ok());
}

#[test]
fn remove_recursive_collapse() {
    let mut t = BinaryTree::<i32>::new();
    // Set the tree up how we want.
    assert!(t.insert(5).is_ok());
    assert!(t.insert(3).is_ok());
    assert!(t.insert(1).is_ok());
    assert!(t.insert(4).is_ok());
    assert!(t.insert(8).is_ok());

    // Check the tree is how we want.
    assert_eq!(t.val.unwrap(), 5);
    assert_eq!(t.left.as_ref().unwrap().val.unwrap(), 3);
    assert_eq!(t.left.as_ref().unwrap().left.as_ref().unwrap().val.unwrap(), 1);
    assert_eq!(t.left.as_ref().unwrap().right.as_ref().unwrap().val.unwrap(), 4);
    assert_eq!(t.right.as_ref().unwrap().val.unwrap(), 8);

    // Remove the root.
    assert!(t.remove(5).is_ok());

    // Check the tree is how we expect.
    assert_eq!(t.val.unwrap(), 4);
    assert_eq!(t.left.as_ref().unwrap().val.unwrap(), 3);
    assert_eq!(t.left.as_ref().unwrap().left.as_ref().unwrap().val.unwrap(), 1);
    assert!(t.left.as_ref().unwrap().right.is_none());
    assert_eq!(t.right.as_ref().unwrap().val.unwrap(), 8);
}
