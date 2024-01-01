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
