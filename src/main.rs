use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};
use std::env;

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
}

pub enum HtmlToken {
    Char(char),
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
                State::Data => match c {
                    _ => {
                        if self.is_eof() {
                            return Some(HtmlToken::Eof);
                        }
                        return Some(HtmlToken::Char(c));
                    }
                },
            }
        }
    }
}
