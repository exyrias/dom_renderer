#[cfg(test)]
mod macro_test {
    use dom_renderer::*;

    #[test]
    fn str() {
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
    fn html_macro() {
        let html1 = html_simple!(
            title: "test",
            body:
                elem!("h1"; domtxt!("section1")),
                elem!("p"; domtxt!("paragraph1"))
        );
        let html2 = html_simple!(
            title: "test",
            body:
                elem!("h1"; domtxt!("section1")),
                elem!("p"; domtxt!("paragraph1")),
        );
        assert_eq!("<!DOCTYPE html><html><head><meta charset=\"utf-8\"><title>test</title></head><body><h1>section1</h1><p>paragraph1</p></body></html>", html1.render());
        assert_eq!("<!DOCTYPE html><html><head><meta charset=\"utf-8\"><title>test</title></head><body><h1>section1</h1><p>paragraph1</p></body></html>", html2.render());
    }
}