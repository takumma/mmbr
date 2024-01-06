use std::{cell::RefCell, rc::Rc};

use super::html_tokenizer::*;

use crate::{
    element::Element,
    node::{Node, NodeKind},
};

#[derive(Debug)]
pub enum InsertionMode {
    Initial,
    BeforeHtml,
    BeforeHead,
    InHead,
    InBody,
    Text,
    AfterHead,
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
    fn create_char(&self, c: char) -> Node {
        let s = String::from(c);
        return Node::new(NodeKind::Text(s));
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

        let node = Rc::new(RefCell::new(self.create_char(c)));

        current_node.borrow_mut().append_child_node(node.clone());
        self.stack_of_open_elements.push(node);
    }

    fn append_element(&mut self, tag_name: String) {
        let new_node = Rc::new(RefCell::new(Node::new(NodeKind::Element(
            Element::from_str(&tag_name),
        ))));

        let current_node = self.current_node();

        current_node
            .borrow_mut()
            .append_child_node(new_node.clone());
        self.stack_of_open_elements.push(new_node);
    }

    fn pop_until(&mut self, kind: NodeKind) {
        loop {
            let current_element = self.stack_of_open_elements.pop();

            if current_element.unwrap().borrow().kind() == kind {
                self.stack_of_open_elements.pop();
                break;
            }

            self.stack_of_open_elements.pop();
        }
    }

    pub fn construct_tree(&mut self) -> Rc<RefCell<Node>> {
        let mut token = self.tokenizer.next();

        while token.is_some() {
            match self.insertion_mode {
                // https://html.spec.whatwg.org/multipage/parsing.html#the-initial-insertion-mode
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
                    Some(HtmlToken::EndTag(ref s))
                        if s != "haed" || s != "body" || s != "html" || s != "br" =>
                    {
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
                // https://html.spec.whatwg.org/multipage/parsing.html#the-before-head-insertion-mode
                InsertionMode::BeforeHead => {
                    // TODO: implement head tag later
                    self.insertion_mode = InsertionMode::InHead;
                }
                // https://html.spec.whatwg.org/multipage/parsing.html#parsing-main-inhead
                InsertionMode::InHead => {
                    // TODO: implement head tag later
                    self.insertion_mode = InsertionMode::AfterHead;
                }
                // https://html.spec.whatwg.org/multipage/parsing.html#the-after-head-insertion-mode
                InsertionMode::AfterHead => match token {
                    Some(HtmlToken::Char(c)) if self.is_whitespace(c) => {
                        token = self.tokenizer.next();
                        continue;
                    }
                    Some(HtmlToken::StartTag(ref tag_name)) if tag_name == "body" => {
                        self.append_element(tag_name.to_owned());
                        token = self.tokenizer.next();
                        self.insertion_mode = InsertionMode::InBody;
                        continue;
                    }
                    Some(HtmlToken::StartTag(ref tag_name)) if tag_name == "head" => {
                        // ignore token
                        token = self.tokenizer.next();
                        continue;
                    }
                    Some(HtmlToken::EndTag(ref s)) if s != "body" || s != "html" || s != "br" => {
                        // ignore token
                        token = self.tokenizer.next();
                        continue;
                    }
                    Some(HtmlToken::Eof) | None => {
                        return self.root.clone();
                    }
                    _ => {
                        self.insertion_mode = InsertionMode::InBody;
                        continue;
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#parsing-main-inbody
                InsertionMode::InBody => match token {
                    Some(HtmlToken::Char(c)) => {
                        self.insert_char(c);
                        token = self.tokenizer.next();
                        continue;
                    }
                    Some(HtmlToken::StartTag(ref tag_name)) => match tag_name.as_str() {
                        "p" | "div" | "span" | "h1" | "h2" => {
                            self.append_element(tag_name.to_owned());
                            token = self.tokenizer.next();
                            continue;
                        }
                        _ => {
                            println!("Unknown tag: {}", tag_name);
                            token = self.tokenizer.next();
                        }
                    },
                    Some(HtmlToken::EndTag(ref tag_name)) => match tag_name.as_str() {
                        "p" | "div" | "span" | "h1" | "h2" => {
                            self.stack_of_open_elements.pop();
                            token = self.tokenizer.next();
                            continue;
                        }
                        _ => {
                            println!("Unknown tag: {}", tag_name);
                            token = self.tokenizer.next();
                        }
                    },
                    Some(HtmlToken::Eof) | None => {
                        return self.root.clone();
                    }
                },
                InsertionMode::Text => {}
                InsertionMode::AfterBody => {}
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
    fn test() {}
}
