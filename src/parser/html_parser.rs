use std::{cell::RefCell, rc::Rc};

use super::html_tokenizer::*;

use crate::node::{Node, NodeKind};

pub enum InsertionMode {
    Initial,
    BeforeHtml,
    BeforeHead,
    InHead,
    InBody,
    Text,
    AfterBody,
}

pub struct HtmlPerser {
    root: Rc<RefCell<Node>>,
    tokenizer: HtmlTokenizer,
    stack_of_open_elements: Vec<Rc<RefCell<Node>>>,
    insertion_mode: InsertionMode,
}

impl HtmlPerser {
    pub fn new(tokenizer: HtmlTokenizer) -> Self {
        Self {
            root: Rc::new(RefCell::new(Node::new(NodeKind::Document))),
            tokenizer,
            stack_of_open_elements: Vec::new(),
            insertion_mode: InsertionMode::Initial,
        }
    }

    // create a text node
    fn create_char(&self, c: char) -> Node {
        let s = String::from(c);
        return Node::new(NodeKind::Text(s));
    }

    fn current_node(&self) -> Rc<RefCell<Node>> {
        match self.stack_of_open_elements.last() {
            Some(n) => n.clone(),
            None => self.root.clone(),
        }
    }

    // insert a character into the tree
    fn insert_char(&mut self, c: char) {
        let current_node = self.current_node();

        match current_node.borrow_mut().kind {
            NodeKind::Text(ref mut s) => {
                s.push(c);
                return;
            }
            _ => {}
        }

        let node = Rc::new(RefCell::new(self.create_char(c)));

        if current_node.borrow().first_child().is_some() {
            current_node
                .borrow()
                .first_child()
                .unwrap()
                .borrow_mut()
                .next_sibling = Some(node.clone());
            node.borrow_mut().previous_sibling =
                Some(Rc::downgrade(&current_node.borrow().first_child().unwrap()));
        } else {
            current_node.borrow_mut().first_child = Some(node.clone());
        }

        current_node.borrow_mut().last_child = Some(Rc::downgrade(&node));
        node.borrow_mut().parent = Some(Rc::downgrade(&current_node));

        self.stack_of_open_elements.push(node);
    }

    fn append_element(&mut self, tag_name: String) {
        let node = Rc::new(RefCell::new(Node::new(NodeKind::Element(
            Element::from_str(&tag_name),
        ))));

        let current_node = match self.stack_of_open_elements.last() {
            Some(n) => n,
            None => &self.root,
        };

        if current_node.borrow().first_child().is_some() {
            current_node
                .borrow()
                .first_child()
                .unwrap()
                .borrow_mut()
                .next_sibling = Some(node.clone());
            node.borrow_mut().previous_sibling =
                Some(Rc::downgrade(&current_node.borrow().first_child().unwrap()));
        } else {
            current_node.borrow_mut().first_child = Some(node.clone());
        }

        current_node.borrow_mut().last_child = Some(Rc::downgrade(&node));
        node.borrow_mut().parent = Some(Rc::downgrade(&current_node));

        self.stack_of_open_elements.push(node);
    }

    pub fn construct_tree(&mut self) -> Rc<RefCell<Node>> {
        let mut token = self.tokenizer.next();

        while token.is_some() {
            match self.insertion_mode {
                InsertionMode::Initial => {
                    self.insertion_mode = InsertionMode::BeforeHtml;
                    continue;
                }
                InsertionMode::BeforeHtml => match token {
                    Some(HtmlToken::Char(c)) => {
                        if self.is_whitespace(c) {
                            token = self.tokenizer.next();
                            continue;
                        } else {
                            self.insertion_mode = InsertionMode::BeforeHead;
                            continue;
                        }
                    }
                    Some(HtmlToken::StartTag(tag_name)) => {
                        if tag_name == "html" {
                            let node = Rc::new(RefCell::new(Node::new(NodeKind::Element(
                                Element::from_str(&tag_name),
                            ))));

                            self.root.borrow_mut().first_child = Some(node.clone());
                            self.root.borrow_mut().last_child = Some(Rc::downgrade(&node));
                            node.borrow_mut().parent = Some(Rc::downgrade(&self.root));

                            self.stack_of_open_elements.push(node);
                            self.insertion_mode = InsertionMode::BeforeHead;
                            token = self.tokenizer.next();
                            continue;
                        } else {
                            self.insertion_mode = InsertionMode::BeforeHead;
                            continue;
                        }
                    }
                    Some(HtmlToken::EndTag(tag_name)) => {
                        if tag_name == "html" {
                            self.insertion_mode = InsertionMode::BeforeHead;
                            continue;
                        } else {
                            self.insertion_mode = InsertionMode::BeforeHead;
                            continue;
                        }
                    }
                    Some(HtmlToken::Eof) => {
                        return self.root.clone();
                    }
                    _ => {}
                },
                _ => {}
            }
            match token {
                Some(HtmlToken::Char(c)) => {
                    self.insert_char(c);
                    token = self.tokenizer.next();
                    continue;
                }
                Some(HtmlToken::StartTag(tag_name)) => {
                    let node = Rc::new(RefCell::new(Node::new(NodeKind::Text(tag_name))));

                    let current_node = match self.stack_of_open_elements.last() {
                        Some(n) => n,
                        None => &self.root,
                    };

                    if current_node.borrow().first_child().is_some() {
                        current_node
                            .borrow()
                            .first_child()
                            .unwrap()
                            .borrow_mut()
                            .next_sibling = Some(node.clone());
                        node.borrow_mut().previous_sibling =
                            Some(Rc::downgrade(&current_node.borrow().first_child().unwrap()));
                    } else {
                        current_node.borrow_mut().first_child = Some(node.clone());
                    }

                    current_node.borrow_mut().last_child = Some(Rc::downgrade(&node));
                    node.borrow_mut().parent = Some(Rc::downgrade(&current_node));

                    self.stack_of_open_elements.push(node);
                    token = self.tokenizer.next();
                    continue;
                }
                Some(HtmlToken::EndTag(tag_name)) => {
                    let mut i = self.stack_of_open_elements.len() - 1;
                    loop {
                        if i == 0 {
                            break;
                        }

                        let node = self.stack_of_open_elements[i].clone();
                        if let NodeKind::Text(ref text) = node.borrow().kind {
                            if text == &tag_name {
                                self.stack_of_open_elements.remove(i);
                                break;
                            }
                        }

                        i -= 1;
                    }

                    token = self.tokenizer.next();
                    continue;
                }
                Some(HtmlToken::Eof) => {
                    return self.root.clone();
                }
                _ => {}
            }
        }

        self.root.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
    }
}