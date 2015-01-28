use std::fmt;

struct Tree<T> {
    value: Option<T>,
    left: Option<Box<Tree<T>>>,
    right: Option<Box<Tree<T>>>,
    parent: Option<Box<Tree<T>>>,
}

impl<T: fmt::Display> fmt::Display for Tree<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut str = String::from_str("[ ");

        match self.value {
            None => str.push_str("<NONE>"),
            Some(ref n) => str.push_str(format!("{}", n).as_slice()),
            };

        match self.left {
            None => {},
            Some(ref t) => str.push_str(format!(" (L: {})", t).as_slice()),
            }

        str.push_str(" ]");

        f.write_str(str.as_slice())
    }
}

#[cfg(test)]
mod test {
    use super::{Tree};

    fn single_value_fixture() -> Tree<i32> {
        Tree{value: Some(32), left: None, right: None, parent: None}
    }

    #[test]
    fn test_creation() {
        let t = single_value_fixture();
        assert_eq!(Some(32), t.value);
    }

    #[test]
    fn test_string() {
        //empty tree
        let mut t: Tree<i32> = Tree{value: None, left: None, right: None, parent: None};
        assert_eq!("[ <NONE> ]", format!("{}", t));

        //simple result
        t = single_value_fixture();

        assert_eq!("[ 32 ]", format!("{}", t));

        //has a left value
        let l = Tree{value: Some(15), left: None, right: None, parent: None};

        t.left = Some(Box::new(l));

        assert_eq!("[ 32 (L: [ 15 ]) ]", format!("{}", t));

        //has a right value

        //is a bit more complicated
    }
}