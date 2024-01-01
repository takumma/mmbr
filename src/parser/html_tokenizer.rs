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

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
