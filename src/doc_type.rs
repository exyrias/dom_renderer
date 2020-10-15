use crate::Render;

#[derive(Debug, Clone)]
pub struct DomDocType<'a> {
    pub doc_type: &'a str,
}

impl<'a> Render for DomDocType<'a> {
    fn render(&self) -> String {
        format!("<!DOCTYPE {}>", self.doc_type)
    }
}

impl<'a> DomDocType<'a> {
    pub fn new(doc_type: &'a str) -> Self {
        DomDocType { doc_type }
    }
}
