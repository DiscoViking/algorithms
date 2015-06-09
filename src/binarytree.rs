use std::cmp::Ordering;

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
