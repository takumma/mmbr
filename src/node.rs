use crate::element::Element;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub struct Node {
    pub kind: NodeKind,
    pub parent: Option<Weak<RefCell<Node>>>,
    pub first_child: Option<Rc<RefCell<Node>>>,
    pub last_child: Option<Weak<RefCell<Node>>>,
    pub next_sibling: Option<Rc<RefCell<Node>>>,
    pub previous_sibling: Option<Weak<RefCell<Node>>>,
}

impl Node {
    pub fn new(kind: NodeKind) -> Self {
        Self {
            kind,
            parent: None,
            first_child: None,
            last_child: None,
            next_sibling: None,
            previous_sibling: None,
        }
    }

    pub fn first_child(&self) -> Option<Rc<RefCell<Node>>> {
        self.first_child.as_ref().map(|n| n.clone())
    }
}

pub enum NodeKind {
    Document,
    Element(Element),
    Text(String),
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
