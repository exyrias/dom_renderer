pub mod doc_type;
pub mod element;
pub mod doccument;
pub mod macros;

pub use doc_type::DomDocType;
pub use element::{DomEmptyElem, DomElem};
pub use doccument::DomDocument;
pub use macros::*;

pub trait Render {
    fn render(&self) -> String;
}

#[derive(Debug, Clone)]
pub enum DomNode {
    Document(DomDocument),
    DocumentType(DomDocType),
    EmptyElement(DomEmptyElem),
    Element(DomElem),
    Text(String),
}
pub use DomNode::*;

impl Render for DomNode {
    fn render(&self) -> String {
        match self {
            Document(n) => n.render(),
            DocumentType(n) => n.render(),
            EmptyElement(n) => n.render(),
            Element(n) => n.render(),
            Text(n) => n.to_string(),
        }
    }
}
