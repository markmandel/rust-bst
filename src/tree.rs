use std::fmt;

struct Tree<T> {
    value: T,
    left: Option<Box<Tree<T>>>,
    right: Option<Box<Tree<T>>>,
    parent: Option<Box<Tree<T>>>,
}

impl<T> fmt::Display for Tree<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Whoo!")
    }
}

#[cfg(test)]
mod test {
    use super::{Tree};

    #[test]
    fn test_creation() {
        let t: Tree<i32> = Tree{value: 32, left: None, right: None, parent: None};
        assert_eq!(32, t.value);
    }

    #[test]
    fn test_string() {
        let t: Tree<i32> = Tree{value: 32, left: None, right: None, parent: None};
        assert_eq!("[ 32 ]", format!("{}", t))
    }
}