use std::fmt;
use std::cmp;

#[derive(Debug, PartialEq, Eq, Clone)]
enum BST<T> {
    Leaf(T),
    // value, left ,       right
    Branch(T, Box<BST<T>>, Box<BST<T>>),
    Nil,
}

impl<T: Ord + Clone> BST<T> {

    //new, empty BST
    fn new() -> BST<T> {
        BST::Nil
    }

    fn insert(&self, node: BST<T>) -> BST<T> {
        match self {
            &BST::Nil => node,
            &BST::Branch(ref value, ref left, ref right) => {
                BST::Nil
            },
            &BST::Leaf(ref value) => {
                match node {
                        BST::Nil => self.clone(),
                        BST::Leaf(ref new_value) | BST::Branch(ref new_value, _, _) => {
                            match new_value.cmp(value) {
                                cmp::Ordering::Less => BST::Branch(value.clone(), Box::new(node.clone()), Box::new(BST::Nil)),
                                _ => BST::Branch(value.clone(), Box::new(BST::Nil), Box::new(node.clone())),
                                }
                        },
                }
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

    #[test]
    fn test_insertion_single_left_leaf() {

        let t = single_value_fixture();
        let node = BST::Leaf(15);

        let expected = BST::Branch(32, Box::new(BST::Leaf(15)), Box::new(BST::Nil));

        let t = t.insert(node);

        assert_eq!(t, expected);
    }
}