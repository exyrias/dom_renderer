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
pub enum DomNode<'a> {
    Document(DomDocument<'a>),
    DocumentType(DomDocType<'a>),
    EmptyElement(DomEmptyElem<'a>),
    Element(DomElem<'a>),
    Text(&'a str),
}
pub use DomNode::*;

impl<'a> Render for DomNode<'a> {
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
