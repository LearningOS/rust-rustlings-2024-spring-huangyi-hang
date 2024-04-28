/*
    double linked list reverse
    This problem requires you to reverse a doubly linked list
*/
// I AM NOT DONE

use std::fmt::{self, Display, Formatter};
use std::os::unix::raw::pthread_t;
use std::ptr::NonNull;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            prev: None,
            next: None,
        }
    }
}
#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        node.prev = self.end;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }

    pub fn reverse(&mut self) {
        if self.length <= 1 {
            return;
        } else {
            let old_start = self.start;
            let old_end = self.end;

            let mut next_opt = None;
            let mut cur_opt = self.start;

            let last_two = unsafe {
                (*old_end.expect("old end").as_ptr()).prev
            };

            for _ in 0..self.length - 1 {
                if let Some(cur) = cur_opt {
                    unsafe {
                        next_opt = (*cur.as_ptr()).next;
                        (*cur.as_ptr()).next = (*cur.as_ptr()).prev;
                        (*cur.as_ptr()).prev = next_opt;

                        assert!((*cur.as_ptr()).prev == next_opt);
                        assert!((*next_opt.expect("next_opt").as_ptr()).prev == cur_opt);
                    }
                }
                cur_opt = next_opt;
            }

            unsafe {
                (*old_start.expect("old start").as_ptr()).next = None;
                (*old_end.expect("old start").as_ptr()).prev = None;
            }

            self.end = old_start;
            self.start = old_end;
            unsafe {
                (*self.start.expect("start").as_ptr()).next = last_two;
            }
        }
    }
    pub fn reverse2(&mut self) {
        if self.length <= 1 {
            return;
        } else {
            for _ in 0..self.length - 1 {
                let old_start = self.start;
                let old_end = self.end;

                self.end = old_start;
                self.start = unsafe { (*old_start.expect("old start").as_ptr()).next };
                unsafe {
                    (*old_start.expect("old start").as_ptr()).prev = old_end;
                    (*old_end.expect("old end").as_ptr()).next = old_start;
                    (*self.end.expect("old end").as_ptr()).next = None;
                    (*self.start.expect("old end").as_ptr()).prev = None;
                };
            }
        }
    }
    pub fn reverse1(&mut self) {
        if self.length <= 1 {
            return;
        } else {
            for _ in 0..self.length - 1 {
                let old_start = self.start;
                let old_end = self.end;
                self.start = old_end;
                self.end = unsafe { (*old_end.expect("old_start").as_ptr()).prev };
                if let Some(v) = old_end {
                    unsafe {
                        (*v.as_ptr()).next = old_start;
                        (*old_start.expect("old start").as_ptr()).prev = old_end;
                    }
                }
            }
        }
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_reverse_linked_list_one_item() {
        let mut list = LinkedList::<i32>::new();
        let original_vec = vec![2];
        let reverse_vec = vec![2];
        for i in 0..original_vec.len() {
            list.add(original_vec[i]);
        }
        println!("Linked List is {}", list);
        list.reverse();
        println!("Reversed Linked List is {}", list);
        for i in 0..original_vec.len() {
            assert_eq!(reverse_vec[i], *list.get(i as i32).unwrap());
        }
    }

    #[test]
    fn test_reverse_linked_list_two_items() {
        let mut list = LinkedList::<i32>::new();
        let original_vec = vec![2, 3];
        let reverse_vec = vec![3, 2];
        for i in 0..original_vec.len() {
            list.add(original_vec[i]);
        }
        println!("Linked List is {}", list);
        list.reverse();
        println!("Reversed Linked List is {}", list);
        for i in 0..original_vec.len() {
            assert_eq!(reverse_vec[i], *list.get(i as i32).unwrap());
        }
    }

    #[test]
    fn test_reverse_linked_list_three_items() {
        let mut list = LinkedList::<i32>::new();
        let original_vec = vec![1, 2, 3];
        let reverse_vec = vec![3, 2, 1];
        for i in 0..original_vec.len() {
            list.add(original_vec[i]);
        }
        println!("Linked List is {}", list);
        list.reverse();
        println!("Reversed Linked List is {}", list);
        for i in 0..original_vec.len() {
            assert_eq!(reverse_vec[i], *list.get(i as i32).unwrap());
        }
    }

    #[test]
    fn test_reverse_linked_list_1() {
        let mut list = LinkedList::<i32>::new();
        let original_vec = vec![2, 3, 5, 11, 9, 7];
        let reverse_vec = vec![7, 9, 11, 5, 3, 2];
        for i in 0..original_vec.len() {
            list.add(original_vec[i]);
        }
        println!("Linked List is {}", list);
        list.reverse();
        println!("Reversed Linked List is {}", list);
        for i in 0..original_vec.len() {
            assert_eq!(reverse_vec[i], *list.get(i as i32).unwrap());
        }
    }

    #[test]
    fn test_reverse_linked_list_2() {
        let mut list = LinkedList::<i32>::new();
        let original_vec = vec![34, 56, 78, 25, 90, 10, 19, 34, 21, 45];
        let reverse_vec = vec![45, 21, 34, 19, 10, 90, 25, 78, 56, 34];
        for i in 0..original_vec.len() {
            list.add(original_vec[i]);
        }
        println!("Linked List is {}", list);
        list.reverse();
        println!("Reversed Linked List is {}", list);
        for i in 0..original_vec.len() {
            assert_eq!(reverse_vec[i], *list.get(i as i32).unwrap());
        }
    }
}
