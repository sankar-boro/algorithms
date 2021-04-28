use std::{cell::{RefCell}, rc::Rc};

#[derive(Debug, Clone)]
struct Node {
  name: String,
  next: Option<Rc<RefCell<Node>>>
}

#[derive(Debug)]
struct LinkedList {
  head: Option<Rc<RefCell<Node>>>,
  tail: Option<Rc<RefCell<Node>>>,
  len: usize,
}

impl LinkedList {
  fn new() -> Self {
    Self {
      head: None,
      tail: None,
      len: 0,
    }
  }

  fn push(&mut self, new_node: Rc<RefCell<Node>>) {
      match &self.tail {
          Some(node) => {
              let mut node = node.as_ref().borrow_mut();
              node.append_node(Rc::clone(&new_node));
          }
          None => {
              self.head = Some(Rc::clone(&new_node));
              self.tail = Some(new_node);
              self.len += 1;
              return;
          }
      }

      self.tail = Some(new_node);    
      self.len+=1;
  }

  fn prepend(&mut self, new_node: Rc<RefCell<Node>>) {
    let mut a = new_node.as_ref().borrow_mut();
    a.next = self.head.clone();
    let b = Rc::new(RefCell::new(a.clone()));
    self.head = Some(b);
    self.len += 1;
  }

  fn pop(&mut self) {
    let head = self.head.clone();
    match head {
        Some(head) => {
          let node = head.as_ref().borrow_mut();
          let next_node = node.next.clone();
          self.head = next_node;
          self.len -= 1;
        }
        None => {}
    }
  }
}

impl Node {
  fn new(name: &str) -> Self {
    Self {
      name: name.to_string(),
      next: None
    }
  }

  fn append_node(&mut self, node: Rc<RefCell<Node>>) {
    self.next = Some(node);
  }
}



fn main() {
  let mut ll = LinkedList::new();
  let node = Rc::new(RefCell::new(Node::new("sankar")));
  ll.push(node);
  let node = Rc::new(RefCell::new(Node::new("arun")));
  ll.push(node);
  let node = Rc::new(RefCell::new(Node::new("boro")));
  ll.push(node);
  let node = Rc::new(RefCell::new(Node::new("bipul")));
  ll.prepend(node);
  ll.pop();
  println!("{:?}", ll);
}