#![allow(dead_code)]

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct MyString {
    vec: Vec<u8>,
}

impl MyString {
    fn new() -> Self {
        Self { vec: Vec::new() }
    }
}

impl From<&str> for MyString {
    fn from(value: &str) -> Self {
        Self {
            vec: value.as_bytes().to_vec(),
        }
    }
}

impl std::fmt::Display for MyString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Convert the Vec<u8> to a &str and then use the formatter to write it out
        // This assumes the Vec<u8> is valid UTF-8. If not, this could panic or behave unexpectedly.
        let s = std::str::from_utf8(&self.vec).map_err(|_| std::fmt::Error)?;
        f.write_str(s)
    }
}

impl std::fmt::Debug for MyString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

// Implementing Deref will make MyString essentially act like a
// a str. It automatically dereferences MyString to &str with coercion.
// With that, MyString will be able to access all of &str's methods.
// impl std::ops::Deref for MyString {
//     type Target = str;

//     fn deref(&self) -> &Self::Target {
//         unsafe { core::str::from_utf8_unchecked(&self.vec) }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let s: MyString = "sfldsf".into();

        eprintln!("s = {:#?}", s);
    }
}
