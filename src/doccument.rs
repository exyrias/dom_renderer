use crate::{Render, DomNode};

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
