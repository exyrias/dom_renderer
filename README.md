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
```

DOM Nodes are represented by `enum DomNode`, which can be created by macros, e.g.,

