use std::{cell::RefCell, rc::Rc};

use super::html_tokenizer::*;

use crate::{node::{Node, NodeKind}, element::Element};

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

    fn is_whitespace(&self, c: char) -> bool {
        c == ' ' || c == '\n' || c == '\t'
    }
    
    // create a text node
    fn create_char(&self, c: char) -> Rc<RefCell<Node>> {
        let s = String::from(c);
        return Rc::new(RefCell::new(Node::new(NodeKind::Text(s))));
    }

    fn current_node(&self) -> &Rc<RefCell<Node>> {
        match self.stack_of_open_elements.last() {
            Some(n) => n,
            None => &self.root,
        }
    }

    // insert a character into the tree
    fn insert_char(&mut self, c: char) {
        let current_node = self.current_node();

        match current_node.borrow_mut().kind() {
            NodeKind::Text(ref mut s) => {
                s.push(c);
                return;
            }
            _ => {}
        }

        let node = self.create_char(c);

        current_node.borrow_mut().append_child_node(node);
        self.stack_of_open_elements.push(node);
    }

    fn append_element(&mut self, tag_name: String) {
        let new_node = Rc::new(RefCell::new(Node::new(NodeKind::Element(
            Element::from_str(&tag_name),
        ))));

        let current_node = self.current_node();

        current_node.borrow_mut().append_child_node(new_node);
        self.stack_of_open_elements.push(new_node);
    }

    pub fn construct_tree(&mut self) -> Rc<RefCell<Node>> {
        let mut token = self.tokenizer.next();

        while token.is_some() {
            match self.insertion_mode {
                InsertionMode::Initial => {
                    self.insertion_mode = InsertionMode::BeforeHtml;
                    continue;
                }
                // https://html.spec.whatwg.org/multipage/parsing.html#the-before-html-insertion-mode
                InsertionMode::BeforeHtml => match token {
                    Some(HtmlToken::Char(c)) if self.is_whitespace(c) => {
                        token = self.tokenizer.next();
                        continue;
                    }
                    Some(HtmlToken::StartTag(ref tag_name)) if tag_name == "html" => {
                        self.append_element(tag_name.to_owned());

                        self.insertion_mode = InsertionMode::BeforeHead;
                        token = self.tokenizer.next();
                        continue;
                    }
                    Some(HtmlToken::EndTag(ref s)) if s != "haed" || s != "body" || s != "html" || s != "br" => {
                        token = self.tokenizer.next();
                        continue;
                    }
                    Some(HtmlToken::Eof) | None => {
                        return self.root.clone();
                    }
                    _ => {
                        self.insertion_mode = InsertionMode::BeforeHead;
                        continue;
                    }
                },
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