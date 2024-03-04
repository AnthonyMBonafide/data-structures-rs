use std::mem;

#[derive(Debug)]
pub enum ListNode<T> {
    Empty,
    List {
        value: T,
        next: Option<Box<ListNode>>,
    },
}

impl<T> ListNode<T> {
    fn new() -> Self {
        ListNode::Empty
    }

    fn insert(&mut self, value: T, position: u32) -> Result<(), &str> {
        todo!()
    }

    fn get(&self, position: u32) -> Option<T> {
        todo!()
    }

    fn remove(&mut self, position: u32) {
        todo!()
    }

    fn add(&mut self, value: T) {
        let mut current_node = self;
        match current_node {
            ListNode::Empty => {
                let x = ListNode::List { value, next: None };
                mem::replace(self, x);
            }

            ListNode::List { value: _, next: n } => n = ListNode::List { value, next: None },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linked_list_insert_and_get() {
        let mut ll = ListNode::new();
        assert!(ll.insert("one", 0).is_ok());
        assert!(ll.insert("two", 1).is_ok());
        assert!(ll.insert("three", 2).is_ok());

        assert_eq!(ll.get(0), Some("one"));
        assert_eq!(ll.get(1), Some("two"));
        assert_eq!(ll.get(2), Some("three"));
    }
    #[test]
    fn test_linked_list_insert_invalid_index() {
        let mut ll = ListNode::new();
        assert!(ll.insert("", -1).is_err());
    }

    #[test]
    fn test_linked_list_insert_higher_than_capacity() {
        let mut ll = ListNode::new();
        assert!(ll.insert("five", 5).is_ok());
        assert_eq!(ll.get(0), None);
        assert_eq!(ll.get(1), None);
        assert_eq!(ll.get(2), None);
        assert_eq!(ll.get(3), None);
        assert_eq!(ll.get(4), Some("five"));
    }

    #[test]
    fn test_linked_list_add_and_get() {
        let mut ll = ListNode::new();
        ll.add("one");
        ll.add("two");
        ll.add("three");

        assert_eq!(ll.get(0), Some("one"));
        assert_eq!(ll.get(1), Some("two"));
        assert_eq!(ll.get(2), Some("three"));
    }

    #[test]
    fn test_linked_list_remove() {
        let mut ll = ListNode::new();
        ll.insert("one", 0);
        ll.insert("two", 1);
        ll.insert("three", 2);
        ll.remove(1);
        assert_eq!(ll.get(0), Some("two"));
        assert_eq!(ll.get(1), Some("three"));
    }
}
