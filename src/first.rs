// Bad Stack implementation.
//
use std::mem;

pub struct List {
  head: Link
}

// Enums will store tag type+ elem for max(variant size). This null pointer 
// optimization only occurs when an enum has two variants, one of which can
// resolve to empty/zero. E.g. strings, Box, etc. are not candidates.
enum Link{
  Empty,
  More(Box<Node>),
}

// Avoids recurisve definition and allows const ptr size.
struct Node {
  elem: i32,
  next: Link,
}

impl List {
  pub fn new() -> Self {
    List { head: Link::Empty }
  }

  pub fn push(&mut self, elem: i32) {
    let new_node = Box::new(Node {
      elem: elem,
      next: mem::replace(&mut self.head, Link::Empty), // Steal value out of borrow.
    });

    self.head = Link::More(new_node);
  }

  pub fn pop_node(&mut self) -> Link {

  }

  pub fn pop(&mut self) -> Option<i32> {
    match mem::replace(&mut self.head, Link::Empty) {
      Link::Empty => None,
      Link::More(node) => {
        self.head = node.next;
        Some(node.elem)
      }
    } // By removing ; we implicitly return the evaluated block.
  }
}

impl Drop for List {
  fn drop(&mut self){
    let mut cur_link = mem::replace(&mut self.head, Link::Empty);
    // `while let` == "do this thing until this pattern doesn't match"
    while let Link::More(mut boxed_node) = cur_link {
      cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
      // boxed_node goes out of scope and gets dropped here;
      // but its Node's `next` field has been set to Link::Empty
      // so no unbounded recursion occurs.
    }
  }
}

#[cfg(test)]
mod test {
  use super::List;

  #[test]
  fn basics() {
      let mut list = List::new();

      // Check empty list behaves right
      assert_eq!(list.pop(), None);

      // Populate list
      list.push(1);
      list.push(2);
      list.push(3);

      // Check normal removal
      assert_eq!(list.pop(), Some(3));
      assert_eq!(list.pop(), Some(2));

      // Push some more just to make sure nothing's corrupted
      list.push(4);
      list.push(5);

      // Check normal removal
      assert_eq!(list.pop(), Some(5));
      assert_eq!(list.pop(), Some(4));

      // Check exhaustion
      assert_eq!(list.pop(), Some(1));
      assert_eq!(list.pop(), None);
  }
}
