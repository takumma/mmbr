use std::rc::{Rc, Weak};
use std::cell::RefCell;
use crate::element::Element;

pub struct Node {
  kind: NodeKind,
  parent: Option<Weak<RefCell<Node>>>,
  first_child: Option<Rc<RefCell<Node>>>,
  last_child: Option<Weak<RefCell<Node>>>,
  next_sibling: Option<Rc<RefCell<Node>>>,
  previous_sibling: Option<Weak<RefCell<Node>>>,
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