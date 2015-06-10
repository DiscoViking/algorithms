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

    pub fn remove(&mut self, val: T) -> Result<T, T> {
        // Deal with the empty case first.
        if self.val.is_none() {
            return Err(val);
        }

        match val.cmp(self.val.as_ref().unwrap()) {
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
                        (v, None, self.right.take())
                    },
                };

                self.val = new_val;
                self.left = new_left;
                self.right = new_right;

                result
            }
        }
    }

    // Remove the rightmost value in tree and return its value.
    fn collapse_rightmost(&mut self) -> Option<T> {
        let (val, del_right) = match self.right {
            None => (self.val.take(), false),
            Some(ref mut t) => {
                let v = t.collapse_rightmost();
                (v, t.val.is_none())
            },
        };

        if del_right {
            self.right = self.right.take().unwrap().left.take();
        }

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
    assert_eq!(t.right.unwrap().val.unwrap(), 5);
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
