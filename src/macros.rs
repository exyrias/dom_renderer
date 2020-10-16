#[macro_export]
macro_rules! domdoc {
    ($($x:expr),*) => {
        Document(DomDocument::new(vec![$($x),*]))
    };
    ($($x:expr,)*) => { $crate::domdoc!($($x),*) };
}

#[macro_export]
macro_rules! doctype {
    ($x:expr) => { DocumentType(DomDocType::new($x)) };
}

#[macro_export]
macro_rules! domtxt {
    ($x:expr) => {
        Text(String::from($x))
    };
}

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