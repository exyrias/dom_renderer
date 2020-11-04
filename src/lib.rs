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
//!
//! let th = elem!("tr";
//!             end_elem!("th"; "Item1"),
//!             end_elem!("th"; "Item2"),
//!             end_elem!("th"; "Item3"),
//! );
//! let tr1 = elem!("tr";
//!             end_elem!("td"; "value 11"),
//!             end_elem!("td"; "value 12"),
//!             end_elem!("td"; "value 13"),
//! );
//! let tr2 = elem!("tr";
//!             end_elem!("td"; "value 21"),
//!             end_elem!("td"; "value 22"),
//!             end_elem!("td"; "value 23"),
//! );
//! let tbl = elem!("table"; ("border", "1"); th, tr1, tr2);
//! assert_eq!("<table border=\"1\"><tr><th>Item1</th><th>Item2</th><th>Item3</th></tr><tr><td>value 11</td><td>value 12</td><td>value 13</td></tr><tr><td>value 21</td><td>value 22</td><td>value 23</td></tr></table>", tbl.render());
//! ```
//! 
//! DOM Nodes are represented by `enum DomNode`, which can be created by macros.
//! 
//! Child nodes can be changed using as_XXX() methods. 
//! 
//! ```
//! use dom_renderer::*;
//! 
//! let mut tbl = elem!("table");
//! let tbl_elem = tbl.as_elem_mut().unwrap();
//! tbl_elem.attributes.push(("border", String::from("1")));
//! // header
//! let mut tr = elem!("tr");
//! let headers = (1..=3)
//!     .map(|i| format!("Item{}", i))
//!     .map(|x| end_elem!("th"; x))
//!     .collect();
//! tr.as_elem_mut().unwrap().child_nodes = headers;
//! tbl_elem.child_nodes.push(tr);
//! // data
//! for i in 1..=2 {
//!     let mut tr = elem!("tr");
//!     let data = (1..=3)
//!         .map(|j| format!("value {}{}", i, j))
//!         .map(|x| end_elem!("td"; x))
//!         .collect();
//!     tr.as_elem_mut().unwrap().child_nodes = data;
//!     tbl_elem.child_nodes.push(tr);
//! }
//! assert_eq!("<table border=\"1\"><tr><th>Item1</th><th>Item2</th><th>Item3</th></tr><tr><td>value 11</td><td>value 12</td><td>value 13</td></tr><tr><td>value 21</td><td>value 22</td><td>value 23</td></tr></table>", tbl.render());
//! ```
//!

pub mod doc_type;
pub mod document;
pub mod element;
pub mod macros;

pub use doc_type::DomDocType;
pub use document::DomDocument;
pub use element::{DomElem, DomEmptyElem};
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
            Text(n) => n.clone(),
        }
    }
}

impl DomNode {
    pub fn as_doc(&self) -> Option<&DomDocument> {
        match *self {
            Document(ref e) => Some(e),
            _ => None,
        }
    }

    pub fn as_doc_mut(&mut self) -> Option<&mut DomDocument> {
        match *self {
            Document(ref mut e) => Some(e),
            _ => None,
        }
    }

    pub fn as_doctype(&self) -> Option<&DomDocType> {
        match *self {
            DocumentType(ref e) => Some(e),
            _ => None,
        }
    }

    pub fn as_doctype_mut(&mut self) -> Option<&mut DomDocType> {
        match *self {
            DocumentType(ref mut e) => Some(e),
            _ => None,
        }
    }

    pub fn as_empty(&self) -> Option<&DomEmptyElem> {
        match *self {
            EmptyElement(ref e) => Some(e),
            _ => None,
        }
    }

    pub fn as_empty_mut(&mut self) -> Option<&mut DomEmptyElem> {
        match *self {
            EmptyElement(ref mut e) => Some(e),
            _ => None,
        }
    }

    pub fn as_elem(&self) -> Option<&DomElem> {
        match *self {
            Element(ref e) => Some(e),
            _ => None,
        }
    }

    pub fn as_elem_mut(&mut self) -> Option<&mut DomElem> {
        match *self {
            Element(ref mut e) => Some(e),
            _ => None,
        }
    }

    pub fn as_text(&self) -> Option<&String> {
        match *self {
            Text(ref e) => Some(e),
            _ => None,
        }
    }

    pub fn as_text_mut(&mut self) -> Option<&mut String> {
        match *self {
            Text(ref mut e) => Some(e),
            _ => None,
        }
    }
}
