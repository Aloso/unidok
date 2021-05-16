macro_rules! elem {
    (< $name:ident $($rest:tt)*) => {
        elem!(name: ElemName::$name, [] $($rest)*)
    };
    (< {$name:expr} $($rest:tt)*) => {
        elem!(name: $name, [] $($rest)*)
    };

    (name: $name:expr, [] { $attrs:expr } $($rest:tt)*) => {
        elem!(name: $name, attrs: $attrs, $($rest)*)
    };

    (name: $name:expr, [$($attrs:expr),*] $key:ident = $val:literal $($rest:tt)*) => {
        elem!(name: $name, [$($attrs,)* attr!($key = $val)] $($rest)*)
    };
    (name: $name:expr, [$($attrs:expr),*] $key:ident = {$val:expr} $($rest:tt)*) => {
        elem!(name: $name, [$($attrs,)* attr!($key = $val)] $($rest)*)
    };
    (name: $name:expr, [$($attrs:expr),*] $key:ident $($rest:tt)*) => {
        elem!(name: $name, [$($attrs,)* attr!($key)] $($rest)*)
    };

    (name: $name:expr, [$($attrs:expr),*] $($rest:tt)*) => {
        elem!(name: $name, attrs: vec![$($attrs),*], $($rest)*)
    };

    (name: $name:expr, attrs: $attrs:expr, /> $($rest:tt)*) => {
        elem!(name: $name, attrs: $attrs, content: None, $($rest)*)
    };
    (name: $name:expr, attrs: $attrs:expr, > { $content:expr } $($rest:tt)*) => {
        elem!(name: $name, attrs: $attrs, content: Some($content), $($rest)*)
    };
    (name: $name:expr, attrs: $attrs:expr, > [ $($contents:expr),* ] $($rest:tt)*) => {
        elem!(name: $name, attrs: $attrs, content: Some(vec![ $($contents),* ]), $($rest)*)
    };

    (name: $name:expr, attrs: $attrs:expr, content: $content:expr, $($rest:tt)*) => {
        Element {
            name: $name,
            attrs: $attrs,
            content: $content,
            $($rest)*
        }
    };
}

macro_rules! attr {
    ($key:ident = $value:expr) => {
        Attr { key: stringify!($key), value: Some($value.into()) }
    };
    ($key:ident) => {
        Attr { key: stringify!($key), value: None }
    };
}
