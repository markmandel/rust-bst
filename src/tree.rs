use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
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
            BST::Nil => node,
            BST::Branch(ref value, ref left, ref right) => {
                BST::Nil
            },
                BST::Leaf(ref value) => {
                BST::Nil
            },
        }
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

    #[test]
    fn test_insertion_no_root() {
        let t = BST::new();

        let node = BST::Leaf(43);
        let expected = node.clone();

        let t = t.insert(node);

        assert_eq!(t, expected)
    }
}