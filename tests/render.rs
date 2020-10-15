
#[cfg(test)]
mod render {
    use dom_renderer::*;

    #[test]
    fn domdoc() {
        let n = Document(DomDocument::new(vec![
            DocumentType(DomDocType::new("html")),
            Element(DomElem::new("html", Vec::new(), Vec::new())),
        ]));
        assert_eq!("<!DOCTYPE html><html></html>", n.render());
    }

    #[test]
    fn domdoctype() {
        let n = DocumentType(DomDocType::new("html"));
        assert_eq!("<!DOCTYPE html>", n.render());
    }

    #[test]
    fn domemptyelem() {
        let n = EmptyElement(DomEmptyElem::new("img",
            vec![("src", "image.jpg")]
        ));
        assert_eq!("<img src=\"image.jpg\">", n.render());
    }

    #[test]
    fn domelem() {
        let n = Element(DomElem::new("div",
            vec![("id", "id1"), ("class", "class1")],
            vec![
                Text("test"),
                EmptyElement(DomEmptyElem::new("br", Vec::new())),
            ]
        ));
        assert_eq!("<div id=\"id1\" class=\"class1\">test<br></div>", n.render());
    }

    #[test]
    fn domtext() {
        let n = Text("text");
        assert_eq!("text", n.render());
    }
}