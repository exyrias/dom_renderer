use crate::Render;

#[derive(Debug, Clone)]
pub struct DomDocType {
    pub doc_type: &'static str,
}

impl Render for DomDocType {
    fn render(&self) -> String {
        format!("<!DOCTYPE {}>", self.doc_type)
    }
}

impl DomDocType {
    pub fn new(doc_type: &'static str) -> Self {
        DomDocType { doc_type }
    }
}
