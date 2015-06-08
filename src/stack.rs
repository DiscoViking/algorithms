// Simple stack implementation.

struct Stack<'a, T: 'a> {
    top: Option<Box<StackNode<'a, T>>>,
}

#[derive(Clone)]
struct StackNode<'a, T: 'a> {
    val: &'a T,
    next: Option<Box<StackNode<'a, T>>>,
}

impl <'a, T> StackNode<'a, T> {
    fn new(val: &'a T) -> StackNode<'a, T> {
        StackNode { val: val, next: None }
    }
}

impl<'a, T> Stack<'a, T> {
    fn push(&mut self, val: &'a T) {
        let mut node = StackNode::new(val);
        let next = self.top.take();
        node.next = next;
        self.top = Some(Box::new(node));
    }

    fn pop(&mut self) -> Option<Box<&'a T>> {
        let val = self.top.take();
        match val {
            None => self.top = None,
            Some(x) => self.top = Some(x.next),
        };

        match val {
            None => None,
            Some(x) => Some(Box::new(x.val)),
        }
    }
}
