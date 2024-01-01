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
    reconsume: bool,
}

impl HtmlTokenizer {
    pub fn new(html: String) -> Self {
        Self {
            state: State::Data,
            pos: 0,
            input: html.chars().collect(),
            current_token: None,
            reconsume: false,
        }
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn consume_input(&mut self) -> char {
        if self.reconsume {
            self.reconsume = false;
            self.input[self.pos - 1]
        } else {
            let c = match self.input.get(self.pos) {
                Some(c) => *c,
                None => '\0',
            };
            self.pos += 1;
            c
        }
    }

    fn is_whitespace(&self, c: char) -> bool {
        c == ' ' || c == '\n' || c == '\t'
    }

    fn create_start_tag_token(&mut self) {
        self.reconsume = true;
        self.current_token = Some(HtmlToken::StartTag(String::new()));
    }

    fn create_end_tag_token(&mut self) {
        self.reconsume = true;
        self.current_token = Some(HtmlToken::EndTag(String::new()));
    }

    fn append_tag_name(&mut self, c: char) {
        match self.current_token {
            Some(HtmlToken::StartTag(ref mut tag_name)) => tag_name.push(c),
            Some(HtmlToken::EndTag(ref mut tag_name)) => tag_name.push(c),
            _ => panic!("Unexpected token: {:?}", self.current_token),
        }
    }
    
}

impl Iterator for HtmlTokenizer {
    type Item = HtmlToken;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos > self.input.len() {
            return None;
        }

        loop {
            
            let c = self.consume_input();
            if c == '\0' {
                return Some(HtmlToken::Eof);
            }

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
                        return self.current_token.take();
                    }
                    if c.is_alphabetic() {
                        self.append_tag_name(c);
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
        assert_eq!(tokenizer.next(), Some(super::HtmlToken::StartTag("html".to_string())));
        assert_eq!(tokenizer.next(), Some(super::HtmlToken::Eof));
        assert_eq!(tokenizer.next(), None);
    }
}
