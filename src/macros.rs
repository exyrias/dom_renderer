#[macro_export]
macro_rules! domtxt {
    ($x:expr) => {
        Text($x)
    };
}

#[macro_export]
macro_rules! empty {
    ($x:tt; $(($a:tt,$v:expr)),+) => {
        EmptyElement(DomEmptyElem::new(
            $x,
            vec![ $(($a, $v),)* ]
        ))
    };
    ($x:tt; $(($a:tt,$v:expr),)+) => { empty!($x; $(($a,$v)),*) };
    ($x:tt) => {
        EmptyElement(DomEmptyElem::new(
            $x,
            Vec::new()
        ))
    };
}

#[macro_export]
macro_rules! elem {
    ($x:tt; $(($a:tt,$v:expr)),+; $($e:expr),+) => {
        Element(DomElem::new(
            $x,
            vec![ $(($a, $v),)* ],
            vec![ $($e,)* ],
        ))
    };
    ($x:tt; $(($a:tt,$v:expr)),+; $($e:expr,)+) => { elem!($x; $(($a,$v)),*; $($e),*) };
    ($x:tt; $(($a:tt,$v:expr)),+) => {
        Element(DomElem::new(
            $x,
            vec![ $(($a, $v),)* ],
            Vec::new(),
        ))
    };
    ($x:tt; $(($a:tt,$v:expr),)+) => { elem!($x; $(($a,$v)),*) };
    ($x:tt; $($e:expr),+) => {
        Element(DomElem::new(
            $x,
            Vec::new(),
            vec![ $($e,)* ],
        ))
    };
    ($x:tt; $($e:expr,)+) => { elem!($x; $($e),*) };
    ($x:tt) => {
        Element(DomElem::new(
            $x,
            Vec::new(),
            Vec::new(),
        ))
    };
}

#[macro_export]
macro_rules! html {
    ($x:expr) => {
        Document(DomDocument::new(vec![
            DocumentType(DomDocType::new("html")),
            $x
        ]))
    };
}

#[macro_export]
macro_rules! html_simple {
    (title: $t:expr, body: $($b:expr),*) => {
        html!(
            elem!("html";
                elem!("head";
                    empty!("meta"; ("charset", "utf-8")),
                    elem!("title"; domtxt!($t)),
                ),
                elem!("body"; $($b),*)
            )
        )
    };
    (title: $t:expr, body: $($b:expr,)+) => {
        html_simple!(title: $t, body: $($b),+)
    };
}