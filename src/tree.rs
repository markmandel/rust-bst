use std::cmp;

//Immutable Binary Search Tree. All operations return a brand new BST.
#[derive(Debug, PartialEq, Eq, Clone)]
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

    //returns the value that is Ordering::Equal to v.
    pub fn get(&self, v: &T) -> Option<T> {
        match self {
            &BST::Leaf(ref value) if value == v => Some(value.clone()),
            &BST::Branch(ref value, ref left, ref right) => {
                match v.cmp(value) {
                    cmp::Ordering::Equal => Some(value.clone()),
                    cmp::Ordering::Less => left.get(v),
                    cmp::Ordering::Greater => right.get(v),
                }
            },
            _ => None,
        }
    }

    pub fn delete(self, v: T) -> BST<T> {
        match self {
            BST::Nil => self,
            BST::Leaf(ref value) => {
                match v.cmp(value) {
                    cmp::Ordering::Equal => BST::Nil,
                    _ => self.clone()
                }
            },
            BST::Branch(value, left, right) => {
                match v.cmp(&value) {
                    cmp::Ordering::Equal => {
                        if *left != BST::Nil && *right == BST::Nil {
                            //if I only have a left value, replace me
                            *left
                        } else if *left == BST::Nil && *right != BST::Nil {
                            //if I only have a right value, replace me
                            *right
                        } else {
                            //if i have 2 values, grab the left most value of the right branch
                            //TODO: Implement - Min and Max
                            BST::Nil
                        }
                    },
                    cmp::Ordering::Less => BST::Branch(value, Box::new(left.delete(v)), right),
                    cmp::Ordering::Greater => BST::Branch(value, left, Box::new(right.delete(v))),
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

    #[test]
    fn test_getting_values() {
        let t: BST<i32> = BST::new()
        .insert_value(10)
        .insert_value(5)
        .insert_value(3)
        .insert_value(15)
        .insert_value(12)
        .insert_value(14);

        //not there
        let result = t.get(&27);
        assert_eq!(None, result);

        let result = t.get(&10);
        assert_eq!(Some(10), result);

        let result = t.get(&5);
        assert_eq!(Some(5), result);

        let result = t.get(&12);
        assert_eq!(Some(12), result);

        let result = t.get(&14);
        assert_eq!(Some(14), result);
    }
}