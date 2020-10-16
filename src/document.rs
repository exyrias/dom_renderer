//! provides DOM Document (root node)

use crate::{Render, DomNode};

/// DOM Document Node (root node)
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
