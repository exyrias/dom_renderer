//! Utilities for generating html (or other DOM) strings 
//! 
//! # Example
//! 
//! A simple html page can be created by following codes:
//! 
//! ```
//! use dom_renderer::*;
//! let html = html_basic!(
//!     title: "Page Title",
//!     body:
//!         end_elem!("h1"; "Section"),
//!         end_elem!("p"; "Text goes here..."),
//! );
//! assert_eq!("<!DOCTYPE html><html><head><meta charset=\"utf-8\"><title>Page Title</title></head><body><h1>Section</h1><p>Text goes here...</p></body></html>", html.render());
//! ```
//! 
//! DOM Nodes are represented by `enum DomNode`, which can be created by macros, e.g.,
//! 

pub mod doc_type;
pub mod element;
pub mod document;
pub mod macros;

pub use doc_type::DomDocType;
pub use element::{DomEmptyElem, DomElem};
pub use document::DomDocument;
pub use macros::*;

/// Render recursive elements to String instance
pub trait Render {
    fn render(&self) -> String;
}

/// Node types
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
