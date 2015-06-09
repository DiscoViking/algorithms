//! Simple generic stack implementation.
//! 
//! # Examples
//!
//! ```
//! use algorithms::stack::Stack;
//! 
//! let mut s = Stack::<i32>::new();
//! s.push(5);
//! s.push(147);
//!
//! assert_eq!(s.pop(), Some(147));
//! assert_eq!(s.pop(), Some(5));
//! assert_eq!(s.pop(), None);
//! ```
//! 
//! ```
//! use algorithms::stack::Stack;
//! 
//! let mut s = Stack::<&'static str>::new();
//! s.push("World");
//! s.push("Hello");
//! 
//! let mut v = Vec::<&'static str>::new();
//!
//! for t in s {
//!     v.push(t);
//! }
//!
//! assert_eq!(v[0], "Hello");
//! assert_eq!(v[1], "World");
//! ```

#[derive(Debug)]
pub struct Stack<T> {
    top: Option<Box<StackNode<T>>>,
}

#[derive(Clone,Debug)]
struct StackNode<T> {
    val: T,
    next: Option<Box<StackNode<T>>>,
}

impl <T> StackNode<T> {
    fn new(val: T) -> StackNode<T> {
        StackNode { val: val, next: None }
    }
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack{ top: None }
    }

    pub fn push(&mut self, val: T) {
        let mut node = StackNode::new(val);
        let next = self.top.take();
        node.next = next;
        self.top = Some(Box::new(node));
    }

    pub fn pop(&mut self) -> Option<T> {
        let val = self.top.take();
        match val {
            None => None,
            Some(mut x) => {
                self.top = x.next.take();
                Some(x.val)
            },
        }
    }
}

/// We may iterate over a stack by repeatedly popping items until empty.
impl<T> Iterator for Stack<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

#[test]
fn push_pop_ptrs() {
    #[derive(PartialEq,Eq,Debug)]
    struct TestStruct {
        a: i32,
    }

    let a = TestStruct{ a: 5 };
    let b = TestStruct{ a: 9 };

    let mut s = Stack::<&TestStruct>::new();
    assert_eq!(s.pop(), None);

    s.push(&a);
    s.push(&b);

    assert_eq!(s.pop(), Some(&b));
    assert_eq!(s.pop(), Some(&a));
    assert_eq!(s.pop(), None);
}
