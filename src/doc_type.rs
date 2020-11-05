//! provides DOM DocumentType

use crate::Render;

/// DOM DocumentType node
/// 
/// # Example
/// 
/// ```
/// use dom_renderer::*;
/// 
/// let expect = r#"<!DOCTYPE html>"#;
/// 
/// let doctype = DocumentType(DomDocType::new("html"));
/// assert_eq!(expect, doctype.render());
/// 
/// assert_eq!(expect, doctype!("html").render());
/// ```
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
