use std::cmp;
use std::ops::Deref;

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

    //push a value in
    pub fn push(self, v: T) -> BST<T> {
        self.push_node(&BST::Leaf(v))
    }

    //insert a whole BST enum type
    fn push_node(self, node: &BST<T>) -> BST<T> {
        match self {
            BST::Nil => node.clone(),
            BST::Branch(value, left, right) => {
                match node {
                    &BST::Nil => BST::Nil,
                    &BST::Leaf(ref new_value) | &BST::Branch(ref new_value, _, _) => {
                        match new_value.cmp(&value) {
                            cmp::Ordering::Less => BST::Branch(value, Box::new(left.push_node(node)), right),
                            _ => BST::Branch(value, left, Box::new(right.push_node(node))),
                        }
                    },
                }
            },
            BST::Leaf(value) => {
                match node {
                    &BST::Nil => BST::Nil,
                    &BST::Leaf(ref new_value) | &BST::Branch(ref new_value, _, _) => {
                        match new_value.cmp(&value) {
                            cmp::Ordering::Less => BST::Branch(value, Box::new(node.clone()), Box::new(BST::Nil)),
                            _ => BST::Branch(value, Box::new(BST::Nil), Box::new(node.clone())),
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

    //Gets the minimum value
    pub fn min(&self) -> Option<T> {
        match self {
            &BST::Nil => None,
            &BST::Leaf(ref value) => Some(value.clone()),
            &BST::Branch(ref value, ref left, _) if left.deref() == &BST::Nil => Some(value.clone()),
            &BST::Branch(_, ref left, _) => left.min(),
        }
    }

    //Gets the minimum value
    pub fn max(&self) -> Option<T> {
        match self {
                &BST::Nil => None,
                &BST::Leaf(ref value) => Some(value.clone()),
                &BST::Branch(ref value, _, ref right) if right.deref() == &BST::Nil => Some(value.clone()),
                &BST::Branch(_, _, ref right) => right.max(),
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
                            let replacement = match right.min() {
                                None => unreachable!(),
                                Some(value) => value,
                            };

                            BST::Branch(replacement, left, Box::new(right.delete(value)))
                        }
                    },
                    cmp::Ordering::Less => BST::Branch(value, Box::new(left.delete(v)), right).downgrade(),
                    cmp::Ordering::Greater => BST::Branch(value, left, Box::new(right.delete(v))).downgrade(),
                }
            },
        }
    }

    //if I'm a branch with no leaves, then return me as a leaf, otherwise I'll return myself
    fn downgrade(self) -> BST<T> {
        match self {
            BST::Branch(ref value, ref left, ref right) if **left == BST::Nil && **right == BST::Nil => BST::Leaf(value.clone()),
            _ => self
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

        let t = t.push_node(&node);

        assert_eq!(t, expected)
    }

    #[test]
    fn test_insertion_single_left_leaf() {

        let t = single_value_fixture();
        let node = BST::Leaf(15);

        let expected = BST::Branch(32, Box::new(node.clone()), Box::new(BST::Nil));

        let t = t.push_node(&node);

        assert_eq!(t, expected);
    }

    #[test]
    fn test_insertion_single_right_leaf() {

        let t = single_value_fixture();
        let node = BST::Leaf(45);

        let expected = BST::Branch(32, Box::new(BST::Nil), Box::new(node.clone()));

        let t = t.push_node(&node);

        assert_eq!(t, expected);
    }

    #[test]
    fn test_complicated_value_bst() {
        let t: BST<i32> = BST::new()
            .push(10)
            .push(5)
            .push(3)
            .push(15)
            .push(12)
            .push(14);

        //lazy assertion, because it's easier to type
        assert_eq!("Branch(10, Branch(5, Leaf(3), Nil), Branch(15, Branch(12, Nil, Leaf(14)), Nil))", format!("{:?}", t))
    }

    #[test]
    fn test_getting_values() {
        let t: BST<i32> = BST::new()
        .push(10)
        .push(5)
        .push(3)
        .push(15)
        .push(12)
        .push(14);

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

    #[test]
    fn test_min_max() {
        let t: BST<i32> = BST::new()
        .push(10)
        .push(5)
        .push(3)
        .push(15)
        .push(12)
        .push(14);

        assert_eq!(Some(3), t.min());
        assert_eq!(Some(15), t.max());
    }

    #[test]
    fn test_delete() {
        let t: BST<i32> = BST::new()
        .push(10)
        .push(5)
        .push(3);

        println!("Before: {:?}", t);

        let t = t.delete(3);

        let expected: BST<i32> = BST::new()
        .push(10).push(5);

        assert_eq!(t, expected);
    }
}