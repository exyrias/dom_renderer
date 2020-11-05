//! provides DOM Document (root node)

use crate::{Render, DomNode};

/// DOM Document Node (root node)
/// 
/// # Example
/// ```
/// // Use DomDocument as root node
/// // Macro `domdoc!(..)` extracts DomNode::Document(DomDocument::new(vec![..])).
/// 
/// use dom_renderer::*;
/// 
/// let doc = domdoc!(
///     doctype!("html"),
///     elem!("head";
///         end_elem!("title"; "TITLE")
///     ),
///     end_elem!("body"; "BODY"),
/// );
/// 
/// let expect = "<!DOCTYPE html><head><title>TITLE</title></head><body>BODY</body>";
/// assert_eq!(expect, doc.render());
#[derive(Debug, Clone)]
pub struct DomDocument {
    pub nodes: Vec<DomNode>,
}

impl DomDocument {
    pub fn new(nodes: Vec<DomNode>) -> Self {
        DomDocument { nodes: nodes }
    }
}

impl Render for DomDocument {
    fn render(&self) -> String {
        self.nodes
            .iter()
            .map(|x| x.render())
            .collect::<Vec<_>>()
            .join("")
    }
}
