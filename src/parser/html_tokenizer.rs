pub enum State {
    Data,
    TagOpen,
    EndTagOpen,
    TagName,
}

#[derive(Debug, PartialEq)]
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
    current_token: Option<HtmlToken>,
}

impl HtmlTokenizer {
    pub fn new(html: String) -> Self {
        Self {
            state: State::Data,
            pos: 0,
            input: html.chars().collect(),
            current_token: None,
        }
    }

    fn is_eof(&self) -> bool {
        self.pos > self.input.len()
    }

    fn create_start_tag_token(&mut self) {
        self.current_token = Some(HtmlToken::StartTag(String::new()));
    }

    fn create_end_tag_token(&mut self) {
        self.current_token = Some(HtmlToken::EndTag(String::new()));
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
                        self.create_start_tag_token();
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
                        self.create_end_tag_token();
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_next() {
        let html = String::from("<html>");
        let mut tokenizer = super::HtmlTokenizer::new(html);
        assert_eq!(tokenizer.next(), Some(super::HtmlToken::Char('<')));
        assert_eq!(tokenizer.next(), Some(super::HtmlToken::StartTag("html".to_string())));
        assert_eq!(tokenizer.next(), Some(super::HtmlToken::Char('>')));
        assert_eq!(tokenizer.next(), Some(super::HtmlToken::Eof));
        assert_eq!(tokenizer.next(), None);
    }
}
