
#[cfg(test)]
mod ref_test {
    use dom_renderer::*;

    #[test]
    fn document() {
        let a = domdoc!(end_elem!("a"));
        let b = empty!("b");

        assert_eq!("<a></a>", a.as_doc().unwrap().render());
        assert!(b.as_doc().is_none());

        let mut a = a;
        a.as_doc_mut().unwrap().nodes.push(domtxt!("text"));
        assert_eq!("<a></a>text", a.as_doc().unwrap().render());
    }

    #[test]
    fn document_type() {
        let a = doctype!("html");
        let b = empty!("b");

        assert_eq!("<!DOCTYPE html>", a.as_doctype().unwrap().render());
        assert!(b.as_doctype().is_none());

        let mut a = a;
        a.as_doctype_mut().unwrap().doc_type = "root";
        assert_eq!("<!DOCTYPE root>", a.as_doctype().unwrap().render());
    }

    #[test]
    fn empty() {
        let a = empty!("a");
        let b = end_elem!("b");

        assert_eq!("<a>", a.as_empty().unwrap().render());
        assert!(b.as_empty().is_none());

        let mut a = a;
        a.as_empty_mut().unwrap().attributes.push(("id",String::from("id1")));
        assert_eq!("<a id=\"id1\">", a.as_empty().unwrap().render());
    }

    #[test]
    fn element() {
        let a = end_elem!("a");
        let b = empty!("b");

        assert_eq!("<a></a>", a.as_elem().unwrap().render());
        assert!(b.as_elem().is_none());

        let mut a = a;
        a.as_elem_mut().unwrap().child_nodes.push(domtxt!("text"));
        assert_eq!("<a>text</a>", a.as_elem().unwrap().render());
    }

    #[test]
    fn text() {
        let a = domtxt!("text");
        let b = empty!("b");

        assert_eq!("text", a.as_text().unwrap());
        assert!(b.as_text().is_none());

        let mut a = a;
        a.as_text_mut().unwrap().replace_range(..4, "TEXT");
        assert_eq!("TEXT", a.as_text().unwrap());
    }

    #[test]
    fn inner_node_update() {
        let mut n = elem!("a";
            end_elem!("b"; "BValue"),
            end_elem!("c"; "CValue"),
            end_elem!("d"; "DValue"),
        );
        let elem_n = n.as_elem_mut().unwrap();
        assert_eq!("a", elem_n.tag);

        let elem_c = elem_n.child_nodes[1].as_elem_mut().unwrap();
        assert_eq!("c", elem_c.tag);
        elem_c.child_nodes.push(empty!("child"));
        elem_c.attributes.push(("attr", String::from("ATTR")));
        elem_c.child_nodes.push(end_elem!("child2"; "Child"));
        elem_c.child_nodes[2].as_elem_mut().unwrap().child_nodes[0].as_text_mut().unwrap().push_str("Value");
        assert_eq!("<a><b>BValue</b><c attr=\"ATTR\">CValue<child><child2>ChildValue</child2></c><d>DValue</d></a>", n.render());
    }
}
