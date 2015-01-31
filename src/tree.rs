use std::cmp;

#[derive(Debug, PartialEq, Eq, Clone)]
//Immutable Binary Search Tree. All operations return a brand new BST.
pub enum BST<T> {
    Leaf(T),
    // value, left ,       right
    Branch(T, Box<BST<T>>, Box<BST<T>>),
    Nil,
}

impl<T: Ord + Clone> BST<T> {

    //new, empty BST
    pub fn new() -> BST<T> {
        BST::Nil
    }

    //convenience function to just insert a value, without having to worry about enums
    pub fn insert_value(&self, v: T) -> BST<T> {
        self.insert(&BST::Leaf(v))
    }

    //insert a whole BST enum type
    fn insert(&self, node: &BST<T>) -> BST<T> {
        match self {
            &BST::Nil => node.clone(),
            &BST::Branch(ref value, ref left, ref right) => {
                match node {
                    &BST::Nil => self.clone(),
                    &BST::Leaf(ref new_value) | &BST::Branch(ref new_value, _, _) => {
                        match new_value.cmp(value) {
                            cmp::Ordering::Less => BST::Branch(value.clone(), Box::new(left.insert(node)), right.clone()),
                            _ => BST::Branch(value.clone(), left.clone(), Box::new(right.insert(node))),
                        }
                    },
                }
            },
            &BST::Leaf(ref value) => {
                match node {
                    &BST::Nil => self.clone(),
                    &BST::Leaf(ref new_value) | &BST::Branch(ref new_value, _, _) => {
                        match new_value.cmp(value) {
                            cmp::Ordering::Less => BST::Branch(value.clone(), Box::new(node.clone()), Box::new(BST::Nil)),
                            _ => BST::Branch(value.clone(), Box::new(BST::Nil), Box::new(node.clone())),
                            }
                    },
                }
            },
        }
    }

    //returns the BST node that matches this value.
    //if none is found BST::Nil is returned
    pub fn get(&self, v: &T) -> BST<T> {
        match self {
            &BST::Leaf(ref value) if value == v => self.clone(),
            &BST::Branch(ref value, ref left, ref right) => {
                match value.cmp(v) {
                    cmp::Ordering::Equal => self.clone(),
                    cmp::Ordering::Less => left.get(&v.clone()),
                    cmp::Ordering::Greater => right.get(&v.clone())
                }
            },
            _ => BST::Nil,
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

        let t = t.insert(&node);

        assert_eq!(t, expected)
    }

    #[test]
    fn test_insertion_single_left_leaf() {

        let t = single_value_fixture();
        let node = BST::Leaf(15);

        let expected = BST::Branch(32, Box::new(node.clone()), Box::new(BST::Nil));

        let t = t.insert(&node);

        assert_eq!(t, expected);
    }

    #[test]
    fn test_insertion_single_right_leaf() {

        let t = single_value_fixture();
        let node = BST::Leaf(45);

        let expected = BST::Branch(32, Box::new(BST::Nil), Box::new(node.clone()));

        let t = t.insert(&node);

        assert_eq!(t, expected);
    }

    #[test]
    fn test_complicated_value_bst() {
        let t: BST<i32> = BST::new()
            .insert_value(10)
            .insert_value(5)
            .insert_value(3)
            .insert_value(15)
            .insert_value(12)
            .insert_value(14);

        //lazy assertion, because it's easier to type
        assert_eq!("Branch(10, Branch(5, Leaf(3), Nil), Branch(15, Branch(12, Nil, Leaf(14)), Nil))", format!("{:?}", t))
    }
}