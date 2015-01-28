use std::fmt;

struct Tree<T> {
    value: T,
    left: Option<Box<Tree<T>>>,
    right: Option<Box<Tree<T>>>,
    parent: Option<Box<Tree<T>>>,
}

impl<T: fmt::Display> fmt::Display for Tree<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut str = format!("[ {}", self.value);

        str = str + " ]";

        f.write_str(str.as_slice())
    }
}

#[cfg(test)]
mod test {
    use super::{Tree};

    fn single_value_fixture() -> Tree<i32> {
        Tree{value: 32, left: None, right: None, parent: None}
    }

    #[test]
    fn test_creation() {
        let t = single_value_fixture();
        assert_eq!(32, t.value);
    }

    #[test]
    fn test_string() {
        //simple result
        let t = single_value_fixture();

        assert_eq!("[ 32 ]", format!("{}", t));

        //has a left value

        //has a right value

        //is a bit more complicated
    }
}