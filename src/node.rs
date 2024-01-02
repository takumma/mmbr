use crate::element::Element;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Clone)]
pub struct Node {
    kind: NodeKind,
    pub parent: Option<Weak<RefCell<Node>>>,
    first_child: Option<Rc<RefCell<Node>>>,
    last_child: Option<Weak<RefCell<Node>>>,
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

    pub fn kind(&self) -> NodeKind {
        self.kind.clone()
    }

    pub fn first_child(&self) -> Option<Rc<RefCell<Node>>> {
        self.first_child.as_ref().map(|n| n.clone())
    }

    pub fn last_child(&self) -> Option<Rc<RefCell<Node>>> {
        self.last_child
            .as_ref()
            .map(|n| n.upgrade().unwrap().clone())
    }

    pub fn append_child_node(&mut self, child_node: Rc<RefCell<Node>>) {
        if self.first_child.is_some() {
            self.first_child.as_ref().unwrap().borrow_mut().next_sibling = Some(child_node.clone());
            child_node.borrow_mut().previous_sibling =
                Some(Rc::downgrade(self.first_child.as_ref().unwrap()));
        } else {
            self.first_child = Some(child_node.clone());
        }

        self.last_child = Some(Rc::downgrade(&child_node));
        child_node.borrow_mut().parent = Some(Rc::downgrade(&Rc::new(RefCell::new(self.clone()))));
    }
}

#[derive(Clone, Debug)]
pub enum NodeKind {
    Document,
    Element(Element),
    Text(String),
}

impl PartialEq for NodeKind {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (NodeKind::Document, NodeKind::Document) => true,
            (NodeKind::Element(e1), NodeKind::Element(e2)) => e1.kind() == e2.kind(),
            (NodeKind::Text(s1), NodeKind::Text(s2)) => s1 == s2,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append_child_node() {
        let mut node = Node::new(NodeKind::Document);
        let child_node = Rc::new(RefCell::new(Node::new(NodeKind::Text(String::from(
            "hello",
        )))));
        node.append_child_node(child_node.clone());

        assert_eq!(
            node.first_child().unwrap().borrow().kind(),
            NodeKind::Text(String::from("hello"))
        );
        assert_eq!(
            node.last_child().unwrap().borrow().kind(),
            NodeKind::Text(String::from("hello"))
        );
    }
}
