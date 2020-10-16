#[macro_export]
macro_rules! domtxt {
    ($x:expr) => {
        Text(String::from($x))
    };
}

#[macro_export]
macro_rules! empty {
    ($x:tt; $(($a:tt,$v:expr)),+) => {
        EmptyElement(DomEmptyElem::new(
            $x,
            vec![ $(($a, String::from($v)),)* ]
        ))
    };
    ($x:tt; $(($a:tt,$v:expr),)+) => { $crate::empty!($x; $(($a,$v)),*) };
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
            vec![ $(($a, String::from($v)),)* ],
            vec![ $($e,)* ],
        ))
    };
    ($x:tt; $(($a:tt,$v:expr)),+; $($e:expr,)+) => { $crate::elem!($x; $(($a,$v)),*; $($e),*) };
    ($x:tt; $(($a:tt,$v:expr)),+) => {
        Element(DomElem::new(
            $x,
            vec![ $(($a, String::from($v)),)* ],
            Vec::new(),
        ))
    };
    ($x:tt; $(($a:tt,$v:expr),)+) => { $crate::elem!($x; $(($a,$v)),*) };
    ($x:tt; $($e:expr),+) => {
        Element(DomElem::new(
            $x,
            Vec::new(),
            vec![ $($e,)* ],
        ))
    };
    ($x:tt; $($e:expr,)+) => { $crate::elem!($x; $($e),*) };
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
        $crate::html!(
            $crate::elem!("html";
                $crate::elem!("head";
                    $crate::empty!("meta"; ("charset", "utf-8")),
                    $crate::elem!("title"; $crate::domtxt!($t)),
                ),
                $crate::elem!("body"; $($b),*)
            )
        )
    };
    (title: $t:expr, body: $($b:expr,)+) => {
        $crate::html_simple!(title: $t, body: $($b),+)
    };
}