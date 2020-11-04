RUST crate for generating html (or other DOM) text as String objects.

# Example

A simple html page can be created by following codes:

```
use dom_renderer::*;
let html = html_basic!(
    title: "Page Title",
    body:
        end_elem!("h1"; "Section"),
        end_elem!("p"; "Text goes here..."),
);
assert_eq!("<!DOCTYPE html><html><head><meta charset=\"utf-8\"><title>Page Title</title></head><body><h1>Section</h1><p>Text goes here...</p></body></html>", html.render());

let th = elem!("tr";
            end_elem!("th"; "Item1"),
            end_elem!("th"; "Item2"),
            end_elem!("th"; "Item3"),
);
let tr1 = elem!("tr";
            end_elem!("td"; "value 11"),
            end_elem!("td"; "value 12"),
            end_elem!("td"; "value 13"),
);
let tr2 = elem!("tr";
            end_elem!("td"; "value 21"),
            end_elem!("td"; "value 22"),
            end_elem!("td"; "value 23"),
);
let tbl = elem!("table"; ("border", "1"); th, tr1, tr2);
assert_eq!("<table border=\"1\"><tr><th>Item1</th><th>Item2</th><th>Item3</th></tr><tr><td>value 11</td><td>value 12</td><td>value 13</td></tr><tr><td>value 21</td><td>value 22</td><td>value 23</td></tr></table>", tbl.render());
```

DOM Nodes are represented by `enum DomNode`, which can be created by macros.

Child nodes can be changed using as_XXX() methods. 

```
use dom_renderer::*;

let mut tbl = elem!("table");
let tbl_elem = tbl.as_elem_mut().unwrap();
tbl_elem.attributes.push(("border", String::from("1")));
// header
let mut tr = elem!("tr");
let headers = (1..=3)
    .map(|i| format!("Item{}", i))
    .map(|x| end_elem!("th"; x))
    .collect();
tr.as_elem_mut().unwrap().child_nodes = headers;
tbl_elem.child_nodes.push(tr);
// data
for i in 1..=2 {
    let mut tr = elem!("tr");
    let data = (1..=3)
        .map(|j| format!("value {}{}", i, j))
        .map(|x| end_elem!("td"; x))
        .collect();
    tr.as_elem_mut().unwrap().child_nodes = data;
    tbl_elem.child_nodes.push(tr);
}
assert_eq!("<table border=\"1\"><tr><th>Item1</th><th>Item2</th><th>Item3</th></tr><tr><td>value 11</td><td>value 12</td><td>value 13</td></tr><tr><td>value 21</td><td>value 22</td><td>value 23</td></tr></table>", tbl.render());
```
