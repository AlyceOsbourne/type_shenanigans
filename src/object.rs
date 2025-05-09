use std::any::type_name;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;

pub struct Object<T: 'static>{
    pub(crate) __tags: PhantomData<T>
}
impl <T> Object<T> {
    pub fn new() -> Object<T> {
        Object {
            __tags: PhantomData
        }
    }
}
fn short_type_name(full: &str) -> String {
    let mut out = String::new();
    let mut depth = 0;
    let mut ident = String::new();

    for c in full.chars() {
        match c {
            '<' => {
                if !ident.is_empty() {
                    let last = ident.rsplit("::").next().unwrap();
                    out.push_str(last);
                    ident.clear();
                }
                out.push('<');
                depth += 1;
            }
            '>' => {
                if !ident.is_empty() {
                    let last = ident.rsplit("::").next().unwrap();
                    out.push_str(last);
                    ident.clear();
                }
                out.push('>');
                depth -= 1;
            }
            ',' => {
                if !ident.is_empty() {
                    let last = ident.rsplit("::").next().unwrap();
                    out.push_str(last);
                    ident.clear();
                }
                out.push(',');
                out.push(' ');
            }
            ':' => {
                ident.push(':'); // part of "::"
            }
            _ => {
                if c.is_alphanumeric() || c == '_' {
                    ident.push(c);
                } else {
                    if !ident.is_empty() {
                        let last = ident.rsplit("::").next().unwrap();
                        out.push_str(last);
                        ident.clear();
                    }
                    out.push(c);
                }
            }
        }
    }

    if !ident.is_empty() {
        let last = ident.rsplit("::").next().unwrap();
        out.push_str(last);
    }

    out
}
impl<T> Debug for Object<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let full = type_name::<T>();
        write!(f, "Object<{}>", short_type_name(full))
    }
}

