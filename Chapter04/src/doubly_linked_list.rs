use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
struct Node {
    value: String,
    next: Link,
    prev: Link,
}

type Link = Option<Rc<RefCell<Node>>>;

impl Node {
    fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value: value,
            next: None,
            prev: None,
        }))
    }
}

#[derive(Clone)]
pub struct BetterTransactionLog {
    head: Link,
    tail: Link,
    pub length: u64,
}

impl BetterTransactionLog {
    pub fn new_empty() -> BetterTransactionLog {
        BetterTransactionLog {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn append(&mut self, value: String) {
        let new = Node::new(value);
        match self.tail.take() {
            Some(old) => {
                old.borrow_mut().next = Some(new.clone());
                new.borrow_mut().prev = Some(old);
            }
            None => self.head = Some(new.clone()),
        };
        self.length += 1;
        self.tail = Some(new);
    }

    pub fn pop(&mut self) -> Option<String> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                next.borrow_mut().prev = None;
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            self.length -= 1;
            Rc::try_unwrap(head)
                .ok()
                .expect("Something is terribly wrong")
                .into_inner()
                .value
        })
    }

    pub fn back_iter(self) -> ListIterator {
        ListIterator::new(self.tail)
    }

    pub fn iter(&self) -> ListIterator {
        ListIterator::new(self.head.clone())
    }
}

impl IntoIterator for BetterTransactionLog {
    type Item = String;
    type IntoIter = ListIterator;

    fn into_iter(self) -> Self::IntoIter {
        ListIterator::new(self.head)
    }
}

pub struct ListIterator {
    current: Link,
}

impl ListIterator {
    fn new(start_at: Link) -> ListIterator {
        ListIterator { current: start_at }
    }
}

impl Iterator for ListIterator {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let current = &self.current;
        let mut result = None;
        self.current = match current {
            Some(current) => {
                let current = current.borrow();
                result = Some(current.value.clone());
                current.next.clone()
            }
            None => None,
        };
        result
    }
}

impl DoubleEndedIterator for ListIterator {
    fn next_back(&mut self) -> Option<String> {
       let current = &self.current;
        let mut result = None;
        self.current = match current {
            Some(ref current) => {
                let current = current.borrow();
                result = Some(current.value.clone());
                current.prev.clone()
            },
            None => None
        };
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::doubly_linked_list;
    use super::*;
    #[test]
    fn test_forward_iterator() {
        let mut log = BetterTransactionLog::new_empty();

        log.append("hello,".to_string());
        log.append("I".to_string());
        log.append("am".to_string());
        log.append("33".to_string());
        log.append("years".to_string());
        log.append("old".to_string());

        for i in log {
            print!("{} ", i);
        }
    }

    #[test]
    fn test_backward_iterator() {
        let mut log = BetterTransactionLog::new_empty();

        log.append("Hola".to_string());
        log.append("Amigo".to_string());
        log.append("Adiós".to_string());


        let mut iter = log.back_iter();

        while let Some(element) = iter.next_back() {
            print!("{} ", element);
        }
    }
}
