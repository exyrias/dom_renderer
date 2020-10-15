use crate::{Render, DomNode};

#[derive(Debug, Clone)]
pub struct DomDocument<'a> {
    pub nodes: Vec<DomNode<'a>>,
}

impl<'a> DomDocument<'a> {
    pub fn new(nodes: Vec<DomNode<'a>>) -> Self {
        DomDocument { nodes: nodes }
    }
}

impl<'a> Render for DomDocument<'a> {
    fn render(&self) -> String {
        self.nodes
            .iter()
            .map(|x| x.render())
            .collect::<Vec<_>>()
            .join("")
    }
}
