struct Tree<T> {
    value: T,
    left: Option<Box<Tree<T>>>,
    right: Option<Box<Tree<T>>>,
    parent: Option<Box<Tree<T>>>,
}


#[cfg(test)]
mod test {
    use super::{Tree};

    fn test_creation() {
        let t: Tree<i32> = Tree{value: 32, left: None, right: None, parent: None};
        assert_eq!(32, t.value)


    }

}