use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use std::cell::RefCell;
use std::env;
use std::rc::{Rc, Weak};

fn input() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    args
}

fn main() {
    let html = input();
    println!("{:?}", html);

    let app = Application::builder().application_id("mmbr").build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("mmbr")
        .build();

    window.present();
}

pub enum State {
    Data,
    TagOpen,
    EndTagOpen,
    TagName,
}

pub enum HtmlToken {
    Char(char),
    StartTag(String),
    EndTag(String),
    Eof,
}

pub struct HtmlTokenizer {
    input: Vec<char>,
    state: State,
    pos: usize,
}

impl HtmlTokenizer {
    pub fn new(html: String) -> Self {
        Self {
            state: State::Data,
            pos: 0,
            input: html.chars().collect(),
        }
    }

    fn is_eof(&self) -> bool {
        self.pos > self.input.len()
    }
}

impl Iterator for HtmlTokenizer {
    type Item = HtmlToken;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_eof() {
            return None;
        };

        loop {
            let c = self.input[self.pos];
            self.pos += 1;

            match self.state {
                State::Data => {
                    if c == '<' {
                        self.state = State::TagOpen;
                        continue;
                    }
                    if self.is_eof() {
                        return Some(HtmlToken::Eof);
                    }

                    return Some(HtmlToken::Char(c));
                }
                State::TagOpen => {
                    if c == '/' {
                        self.state = State::EndTagOpen;
                        continue;
                    }
                    if c.is_alphabetic() {
                        self.state = State::TagName;
                        continue;
                    }
                    if self.is_eof() {
                        return Some(HtmlToken::Eof);
                    }

                    self.state = State::Data;
                }
                State::EndTagOpen => {
                    if c.is_alphabetic() {
                        self.state = State::TagName;
                        continue;
                    }
                    if self.is_eof() {
                        return Some(HtmlToken::Eof);
                    }
                }
                State::TagName => {
                    if c == '>' {
                        self.state = State::Data;
                        continue;
                    }
                    if c.is_alphabetic() {
                        continue;
                    }
                    if self.is_eof() {
                        return Some(HtmlToken::Eof);
                    }
                
                }
            }
        }
    }
}   

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

pub struct Element {
    kind: HtmlElementKind,
    // attributes: Vec<Attribute>,
}

pub enum HtmlElementKind {
    Html,
    Head,
    Body,
    Title,
    P,
    Div,
    Span,
    H1,
    H2,
}

impl Element {
    pub fn new(kind: HtmlElementKind) -> Self {
        Self { kind }
    }
}

pub struct HtmlPerser {
    root: Rc<RefCell<Node>>,
    tokenizer: HtmlTokenizer,
    stack_of_open_elements: Vec<Rc<RefCell<Node>>>,
}

impl HtmlPerser {
    pub fn new(tokenizer: HtmlTokenizer) -> Self {
        Self {
            root: Rc::new(RefCell::new(Node::new(NodeKind::Document))),
            tokenizer,
            stack_of_open_elements: Vec::new(),
        }
    }

    fn create_char(&self, c: char) -> Node {
        let s = String::from(c);
        return Node::new(NodeKind::Text(s));
    }

    fn insert_char(&mut self, c: char) {
        let current_node = match self.stack_of_open_elements.last() {
            Some(n) => n,
            None => &self.root,
        };

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

    pub fn construct_tree(&mut self) -> Rc<RefCell<Node>> {
        let mut token = self.tokenizer.next();

        while token.is_some() {
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
mod test {
    #[test]
    fn test() {}
}