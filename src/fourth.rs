use std::cell::RefCell;
use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

pub struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem: elem,
            next: None,
            prev: None,
        }))
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }
    pub fn push_front(&mut self, elem:T) {
      // new node => +2 links, everything else +0 links
      let new_head = Node::new(elem);
      match self.head.take() {
        Some(old_head) => {
          // populated, connect to head
          old_head.borrow_mut().prev = Some(new_head.clone());
          new_head.borrow_mut().next = Some(old_head);
          self.head = Some(new_head);


        }
        None => {
          // empty, assign head + tail
          self.head = Some(new_head.clone());
          self.tail = Some(new_head);
        }
      }
    }

    pub fn pop_front(&mut self) -> Option<T> {
      self.head.take().map(|old_head| {
          match old_head.borrow_mut().next.take() {
            Some(new_head) => { // assign next to new_head
              new_head.borrow_mut().prev.take(); // remove prev aka current head
              self.head = Some(new_head); // reassign with next
            }
            None => {
              self.tail.take(); // remove tail for clarity
            }
          }
          Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
      })
    }

    // TODO paused here (https://rust-unofficial.github.io/too-many-lists/fourth-peek.html)
    pub fn peek_front(&self) -> Option<&T> {
      self.head.as_ref().map(|node| {
        &node.borrow().elem
      })
    }
}

impl<T> Drop for List<T> {
  fn drop(&mut self){
    while self.pop_front().is_some(){}
  }
}


#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop_front(), None);

        // Populate list
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_front(4);
        list.push_front(5);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }
}
