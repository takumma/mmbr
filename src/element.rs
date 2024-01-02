#[derive(Clone, Debug)]
pub struct Element {
    kind: HtmlElementKind,
    // attributes: Vec<Attribute>,
}

#[derive(Clone, PartialEq, Debug)]
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

    pub fn kind(&self) -> &HtmlElementKind {
        &self.kind
    }

    pub fn to_string(&self) -> String {
        match self.kind {
            HtmlElementKind::Html => String::from("html"),
            HtmlElementKind::Head => String::from("head"),
            HtmlElementKind::Body => String::from("body"),
            HtmlElementKind::Title => String::from("title"),
            HtmlElementKind::P => String::from("p"),
            HtmlElementKind::Div => String::from("div"),
            HtmlElementKind::Span => String::from("span"),
            HtmlElementKind::H1 => String::from("h1"),
            HtmlElementKind::H2 => String::from("h2"),
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "html" => Self::new(HtmlElementKind::Html),
            "head" => Self::new(HtmlElementKind::Head),
            "body" => Self::new(HtmlElementKind::Body),
            "title" => Self::new(HtmlElementKind::Title),
            "p" => Self::new(HtmlElementKind::P),
            "div" => Self::new(HtmlElementKind::Div),
            "span" => Self::new(HtmlElementKind::Span),
            "h1" => Self::new(HtmlElementKind::H1),
            "h2" => Self::new(HtmlElementKind::H2),
            _ => panic!("Unknown element: {}", s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let element = Element::new(HtmlElementKind::Html);
        assert_eq!(element.to_string(), "html");
    }
}
