use std::fmt;

#[derive(Debug, PartialEq, Eq)]
enum BST<T> {
    Leaf(T),
    // value, left ,       right
    Branch(T, Box<BST<T>>, Box<BST<T>>),
    Nil,
}

impl<T> BST<T> {

    //new, empty BST
    fn new() -> BST<T> {
        BST::Nil
    }

    fn insert(self, node: BST<T>) -> BST<T> {
        match self {
            _ => self
            }
    }
}

impl<T: fmt::Display> fmt::Display for BST<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut str = String::from_str("[ ");

        str.push_str(" ]");

        f.write_str(str.as_slice())
    }
}

#[cfg(test)]
mod test {
    use super::{BST};

    fn single_value_fixture() -> BST<i32> {
        BST::Leaf(32)
    }

    #[test]
    fn test_creation() {
        let t = single_value_fixture();
        assert_eq!(BST::Leaf(32), t);
    }


}