//! provides DOM DocumentType

use crate::Render;

/// DOM DocumentType node
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
