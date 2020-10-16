//! provides macros to create DOM Eelement instances

/// creates DOM Document
/// 
/// Use this to create a root node.
#[macro_export]
macro_rules! domdoc {
    ($($x:expr),*) => {
        Document(DomDocument::new(vec![$($x),*]))
    };
    ($($x:expr,)*) => { $crate::domdoc!($($x),*) };
}

/// creates DOM DocumentType
/// # Example
/// ```
/// use dom_renderer::*;
/// assert_eq!("<!DOCTYPE html>", doctype!("html").render());
/// ```
#[macro_export]
macro_rules! doctype {
    ($x:expr) => { DocumentType(DomDocType::new($x)) };
}

/// creates DOM Text Node
/// # Example
/// ```
/// use dom_renderer::*;
/// assert_eq!("Text", domtxt!("Text").render());
/// ```
#[macro_export]
macro_rules! domtxt {
    ($x:expr) => {
        Text(String::from($x))
    };
}

/// creates DOM Empty Element
/// 
/// # Example
/// Tag and attribute lists are separated by ';'
/// 
/// Attributes can be omitted.
/// 
/// ```
/// use dom_renderer::*;
/// 
/// let br = empty!("br");
/// assert_eq!("<br>", br.render());
/// 
/// let input = empty!("input";
///     ("type", "text"),
///     ("name", "user_name"),
/// );
/// assert_eq!("<input type=\"text\" name=\"user_name\">", input.render());
/// ```
#[macro_export]
macro_rules! empty {
    ($x:expr) => { EmptyElement(DomEmptyElem::new( $x, Vec::new())) };
    ($x:expr; $(($a:expr,$v:expr)),+) => {
        EmptyElement(DomEmptyElem::new(
            $x,
            vec![ $(($a, String::from($v)),)* ]
        ))
    };
    ($x:expr; $(($a:expr,$v:expr),)+) => { $crate::empty!($x; $(($a,$v)),*) };
}

/// creates DOM Element
/// 
/// # Example
/// Tag, attribute lists, and child elements are separated by ';'
/// 
/// Attributes and child elements can be omitted.
/// 
/// ```
/// use dom_renderer::*;
/// let div = elem!("div";
///     ("id", "id1"),
///     ("class", "class1");
///     domtxt!("text"),
///     empty!("br"),
///     domtxt!("text"),
/// );
/// assert_eq!("<div id=\"id1\" class=\"class1\">text<br>text</div>", div.render());
/// ```
#[macro_export]
macro_rules! elem {
    ($x:expr; $(($a:expr,$v:expr)),+; $($e:expr),+) => {
        Element(DomElem::new(
            $x,
            vec![ $(($a, String::from($v)),)* ],
            vec![ $($e,)* ],
        ))
    };
    ($x:expr; $(($a:expr,$v:expr)),+; $($e:expr,)+) => { $crate::elem!($x; $(($a,$v)),*; $($e),*) };
    ($x:expr; $(($a:expr,$v:expr)),+) => {
        Element(DomElem::new(
            $x,
            vec![ $(($a, String::from($v)),)* ],
            Vec::new(),
        ))
    };
    ($x:expr; $(($a:expr,$v:expr),)+) => { $crate::elem!($x; $(($a,$v)),*) };
    ($x:expr; $($e:expr),+) => {
        Element(DomElem::new(
            $x,
            Vec::new(),
            vec![ $($e,)* ],
        ))
    };
    ($x:expr; $($e:expr,)+) => { $crate::elem!($x; $($e),*) };
    ($x:expr) => {
        Element(DomElem::new(
            $x,
            Vec::new(),
            Vec::new(),
        ))
    };
}

/// crates DOM Element that have a single text node.
/// # Example
/// Tag, attribute lists, and text are separated by ';'
/// 
/// Attributes and child elements can be omitted.
/// ```
/// use dom_renderer::*;
/// assert_eq!("<title>Title</title>", end_elem!("title"; "Title").render());
/// ```
#[macro_export]
macro_rules! end_elem {
    ($x:expr) => { $crate::elem!($x) };
    ($x:expr; $(($a:expr,$v:expr)),+) => { $crate::elem!($x; $(($a,$v)),*) };
    ($x:expr; $(($a:expr,$v:expr),)+) => { $crate::elem!($x; $(($a,$v)),*) };
    ($x:expr; $(($a:expr,$v:expr)),+; $e:expr) => {
        $crate::elem!($x; $(($a,$v)),*; $crate::domtxt!($e))
    };
    ($x:expr; $e:expr) => { $crate::elem!($x; $crate::domtxt!($e)) };
}

/// creates HTML Document contains DocumenType and HTML element
/// # Example
/// ```
/// use dom_renderer::*;
/// let html = html!(
///     ("lang", "en");
///     elem!("head";
///         empty!("meta"; ("charset", "utf-8")),
///         end_elem!("title"; "Page Title")
///     ),
///     elem!("body";
///         end_elem!("h1"; "Section"),
///         end_elem!("p"; "Text goes here..."),
///     ),
/// );
/// let expect = "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\"><title>Page Title</title></head><body><h1>Section</h1><p>Text goes here...</p></body></html>";
/// assert_eq!(expect, html.render());
/// ```
#[macro_export]
macro_rules! html {
    ($($x:expr),*) => {
        $crate::domdoc!(
            $crate::doctype!("html"),
            $crate::elem!("html"; $($x),*)
        )
    };
    ($($x:expr,)*) => { $crate::html!($($x),*) };
    ($(($a:expr,$v:expr)),*; $($x:expr),*) => {
        $crate::domdoc!(
            $crate::doctype!("html"),
            $crate::elem!("html"; $(($a,$v)),*; $($x),*)
        )
    };
    ($(($a:expr,$v:expr)),*; $($x:expr,)*) => {
        $crate::html!($(($a,$v)),*; $($x),*)
    };
}

/// creates HTML Document contains DocumenType and HTML element
/// # Example
/// ```
/// use dom_renderer::*;
/// let html = html_basic!(
///     title: "Page Title",
///     body:
///         end_elem!("h1"; "Section"),
///         end_elem!("p"; "Text goes here..."),
/// );
/// let expect = "<!DOCTYPE html><html><head><meta charset=\"utf-8\"><title>Page Title</title></head><body><h1>Section</h1><p>Text goes here...</p></body></html>";
/// assert_eq!(expect, html.render());
/// ```
#[macro_export]
macro_rules! html_basic {
    (title: $t:expr, body: $($b:expr),*) => {
        $crate::html!(
            $crate::elem!("head";
                $crate::empty!("meta"; ("charset", "utf-8")),
                $crate::end_elem!("title"; $t),
            ),
            $crate::elem!("body"; $($b),*)
        )
    };
    (title: $t:expr, body: $($b:expr,)+) => {
        $crate::html_basic!(title: $t, body: $($b),+)
    };
}