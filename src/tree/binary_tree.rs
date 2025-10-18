pub struct Tree<T>
where
    T: PartialOrd + PartialEq + Clone,
{
    root: Option<Node<T>>,
}
impl<T> Tree<T>
where
    T: PartialOrd + Clone,
{
    pub fn new(item: T) -> Self {
        Tree::<T> {
            root: Some(Node::<T>::new(item)),
        }
    }

    pub fn add(&mut self, item: T) {
        let mut current_node = self.root.as_ref();
        while current_node.is_some() {
            let c = current_node.unwrap();
            if c.value == item {
                return;
            }

            if c.value < item {
                if c.left_child.is_none() {
                    // c.left_child = Some(Box::new(Node::new(item)));
                    return;
                }
                current_node = c.left_child.as_ref().map(|i| i.as_ref());

                continue;
            }

            ///////
            if c.value > item {
                current_node = c.right_child.as_ref().map(|i| i.as_ref());

                continue;
            }
        }
    }

    pub fn remove(&mut self, _: T) {}

    pub fn has(&self, item: T) -> bool {
        // TODO: Walk through the tree to find if there is a math

        let mut current_node = match &self.root {
            Option::Some(n) => Option::Some(n),
            Option::None => return false,
        };

        while current_node.is_some() {
            if current_node.unwrap().value == item {
                return true;
            }

            if current_node.unwrap().value < item {
                current_node = match &current_node.unwrap().left_child {
                    Some(n) => Some(n.as_ref()),
                    None => {
                        return false;
                    }
                };
            }

            if current_node.unwrap().value > item {
                current_node = match &current_node.unwrap().right_child {
                    Some(n) => Some(n.as_ref()),
                    None => {
                        return false;
                    }
                };
            }
        }

        false
    }
}

impl<T> Default for Tree<T>
where
    T: PartialOrd + PartialEq + Clone + Default,
{
    fn default() -> Self {
        Self {
            root: Option::Some(Node::<T>::default()),
        }
    }
}
struct Node<T>
where
    T: PartialOrd + PartialEq + Clone,
{
    value: T,
    left_child: Option<Box<Node<T>>>,
    right_child: Option<Box<Node<T>>>,
}

impl<T> Node<T>
where
    T: PartialOrd + PartialEq + Clone,
{
    pub fn new(item: T) -> Self {
        Node {
            value: item,
            left_child: Option::None,
            right_child: Option::None,
        }
    }

    pub fn left(&self) -> Option<T> {
        self.left_child.as_ref().map(|i| i.value.clone())
    }
    pub fn right(&self) -> Option<T> {
        self.right_child.as_ref().map(|i| i.value.clone())
    }

    pub fn remove_left(&mut self) {
        self.left_child = Option::None;
    }

    pub fn remove_right(&mut self) {
        self.right_child = Option::None;
    }

    pub fn set_left(&mut self, item: T) {
        self.left_child = Option::Some(Box::new(Node::new(item)));
    }
    pub fn set_right(&mut self, item: T) {
        self.right_child = Option::Some(Box::new(Node::new(item)));
    }
}

impl<T> Default for Node<T>
where
    T: PartialOrd + PartialEq + Clone + Default,
{
    fn default() -> Self {
        Self {
            value: T::default(),
            left_child: Option::None,
            right_child: Option::None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Tree;

    // TODO: Replace this with a fuzz test for better coverage once all functionality for adding is
    // in place
    #[test]
    fn tree_has_item() {
        let root_value = 5;
        let sut = Tree::new(root_value);
        assert!(sut.has(root_value), "root value should match");
        assert!(!sut.has(root_value + 1));
    }
}
