use crate::{DomNode, Render};

#[derive(Debug, Clone)]
pub struct DomEmptyElem<'a> {
    pub tag: &'static str,
    pub attributes: Vec<(&'static str, &'a str)>,
}

impl<'a> DomEmptyElem<'a> {
    pub fn new(tag: &'static str, attr: Vec<(&'static str, &'a str)>) -> Self {
        DomEmptyElem {
            tag,
            attributes: attr,
        }
    }
}

impl<'a> Render for DomEmptyElem<'a> {
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

#[derive(Debug, Clone)]
pub struct DomElem<'a> {
    pub tag: &'static str,
    pub attributes: Vec<(&'static str, &'a str)>,
    pub child_nodes: Vec<DomNode<'a>>,
}

impl<'a> DomElem<'a> {
    pub fn new(
        tag: &'static str,
        attr: Vec<(&'static str, &'a str)>,
        child_nodes: Vec<DomNode<'a>>,
    ) -> Self {
        DomElem {
            tag,
            attributes: attr,
            child_nodes: child_nodes,
        }
    }
}

impl<'a> Render for DomElem<'a> {
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
