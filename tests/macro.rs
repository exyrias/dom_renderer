#[cfg(test)]
mod macro_test {
    use dom_renderer::*;

    #[test]
    fn doctype() {
        assert_eq!("<!DOCTYPE html>", doctype!("html").render());
    }

    #[test]
    fn domdoc() {
        let d1 = domdoc!(doctype!("html"), elem!("html"));
        let d2 = domdoc!(doctype!("html"), elem!("html"), );
        assert_eq!("<!DOCTYPE html><html></html>", d1.render());
        assert_eq!("<!DOCTYPE html><html></html>", d2.render());
    }

    #[test]
    fn domtxt() {
        assert_eq!("test", domtxt!("test").render());
    }

    #[test]
    fn empty() {
        assert_eq!("<br>", empty!("br").render());
    }

    #[test]
    fn empty_with_attr() {
        let e1 = empty!("input";
            ("type", "text"),
        );
        let e2 = empty!("input";
            ("type", "text")
        );
        let e3 = empty!("input";
            ("name", "name1"),
            ("type", "text"),
        );
        let e4 = empty!("input";
            ("name", "name1"),
            ("type", "text")
        );
        assert_eq!("<input type=\"text\">", e1.render());
        assert_eq!("<input type=\"text\">", e2.render());
        assert_eq!("<input name=\"name1\" type=\"text\">", e3.render());
        assert_eq!("<input name=\"name1\" type=\"text\">", e4.render());
    }


    #[test]
    fn elem() {
        assert_eq!("<html></html>", elem!("html").render());
    }

    #[test]
    fn elem_with_attr() {
        let e1 = elem!("div";
            ("id", "id1"),
            ("class", "class1")
        );
        let e2 = elem!("div";
            ("id", "id1"),
            ("class", "class1"),
        );
        assert_eq!("<div id=\"id1\" class=\"class1\"></div>", e1.render());
        assert_eq!("<div id=\"id1\" class=\"class1\"></div>", e2.render());
    }

    #[test]
    fn elem_with_child() {
        let e1 = elem!("div";
            elem!("h1"; domtxt!("header")),
            elem!("p"; domtxt!("text"))
        );
        let e2 = elem!("div";
            elem!("h1"; domtxt!("header")),
            elem!("p"; domtxt!("text")),
        );
        assert_eq!("<div><h1>header</h1><p>text</p></div>", e1.render());
        assert_eq!("<div><h1>header</h1><p>text</p></div>", e2.render());
    }

    #[test]
    fn elem_with_attr_child() {
        let e1 = elem!("div";
            ("id", "id1"),
            ("class", "class1");
            domtxt!("text")
        );
        let e2 = elem!("div";
            ("id", "id1"),
            ("class", "class1");
            domtxt!("text"),
        );
        assert_eq!("<div id=\"id1\" class=\"class1\">text</div>", e1.render());
        assert_eq!("<div id=\"id1\" class=\"class1\">text</div>", e2.render());
    }

    #[test]
    fn end_elem() {
        let e1 = end_elem!("div");
        assert_eq!("<div></div>", e1.render());

        let e1 = end_elem!("div"; ("id", "id1"));
        let e2 = end_elem!("div"; ("id", "id1"),);
        assert_eq!("<div id=\"id1\"></div>", e1.render());
        assert_eq!("<div id=\"id1\"></div>", e2.render());

        let e1 = end_elem!("div"; ("id", "id1"); "text");
        assert_eq!("<div id=\"id1\">text</div>", e1.render());

        let e1 = end_elem!("div"; "text");
        assert_eq!("<div>text</div>", e1.render());
    }

    #[test]
    fn html_macro() {
        let html1 = html!(
            ("lang", "en");
            elem!("head"),
            elem!("body")
        );
        let html2 = html!(
            ("lang", "en");
            elem!("head"),
            elem!("body"),
        );
        let expect = "<!DOCTYPE html><html lang=\"en\"><head></head><body></body></html>";
        assert_eq!(expect, html1.render());
        assert_eq!(expect, html2.render());

        let html1 = html!(
            elem!("head"),
            elem!("body")
        );
        let html2 = html!(
            elem!("head"),
            elem!("body"),
        );
        let expect = "<!DOCTYPE html><html><head></head><body></body></html>";
        assert_eq!(expect, html1.render());
        assert_eq!(expect, html2.render());
    }

    #[test]
    fn html_basic(){
        let html1 = html_basic!(
            title: "Page Title",
            body:
                end_elem!("h1"; "Section"),
                end_elem!("p"; "Text goes here...")
        );
        let html2 = html_basic!(
            title: "Page Title",
            body:
                end_elem!("h1"; "Section"),
                end_elem!("p"; "Text goes here..."),
        );
        let expect = "<!DOCTYPE html><html><head><meta charset=\"utf-8\"><title>Page Title</title></head><body><h1>Section</h1><p>Text goes here...</p></body></html>";
        assert_eq!(expect, html1.render());
        assert_eq!(expect, html2.render());
    }
}