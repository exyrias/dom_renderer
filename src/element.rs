//! provides DOM Elements

use crate::{DomNode, Render};

/// DOM Empty Element
#[derive(Debug, Clone)]
pub struct DomEmptyElem {
    pub tag: &'static str,
    pub attributes: Vec<(&'static str, String)>,
}

impl DomEmptyElem {
    pub fn new(tag: &'static str, attr: Vec<(&'static str, String)>) -> Self {
        DomEmptyElem {
            tag,
            attributes: attr,
        }
    }
}

impl Render for DomEmptyElem {
    fn render(&self) -> String {
        let attr_str = self
            .attributes
            .iter()
            .map(|(a, v)| format!(" {}=\"{}\"", a, v))
            .collect::<Vec<_>>()
            .join("");
        format!("<{}{}>", self.tag, attr_str)
    }
}

/// DOM Element with child elements
#[derive(Debug, Clone)]
pub struct DomElem {
    pub tag: &'static str,
    pub attributes: Vec<(&'static str, String)>,
    pub child_nodes: Vec<DomNode>,
}

impl DomElem {
    pub fn new(
        tag: &'static str,
        attr: Vec<(&'static str, String)>,
        child_nodes: Vec<DomNode>,
    ) -> Self {
        DomElem {
            tag,
            attributes: attr,
            child_nodes: child_nodes,
        }
    }
}

impl Render for DomElem {
    fn render(&self) -> String {
        let attr_str = self
            .attributes
            .iter()
            .map(|(a, v)| format!(" {}=\"{}\"", a, v))
            .collect::<Vec<_>>()
            .join("");
        let inner_html = self
            .child_nodes
            .iter()
            .map(|x| x.render())
            .collect::<Vec<_>>()
            .join("");
        format!(
            "<{tag}{attr}>{child_nodes}</{tag}>",
            tag = self.tag,
            attr = attr_str,
            child_nodes = inner_html
        )
    }
}
