
#[cfg(test)]
mod ref_test {
    use dom_renderer::*;

    #[test]
    fn element() {
        let a = end_elem!("a");
        let b = empty!("b");

        assert_eq!("<a></a>", a.as_elem().unwrap().render());
        assert!(b.as_elem().is_none());

        let mut c = end_elem!("c");
        c.as_elem_mut().unwrap().child_nodes.push(domtxt!("text"));
        assert_eq!("<c>text</c>", c.as_elem().unwrap().render());
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
        assert_eq!("<a><b>BValue</b><c>CValue<child></c><d>DValue</d></a>", n.render());
    }
}
