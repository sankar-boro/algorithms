#[allow(unused_variables)]
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

	fn push_back(&mut self, new_node: Rc<RefCell<Node>>) {
		match &self.tail {
			Some(node) => {
				let mut node = node.as_ref().borrow_mut();
				node.create_next(Rc::clone(&new_node));
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

	fn push_front(&mut self, new_node: Rc<RefCell<Node>>) {
		match &self.head {
			Some(head) => {
				new_node.borrow_mut().create_next_from_copy(head);
				self.head = Some(new_node);
				self.len += 1;
			}
			None => {
				self.head = Some(Rc::clone(&new_node));
				self.tail = Some(Rc::clone(&new_node));
				self.len += 1;
			}
		}
	}

  	fn pop_front(&mut self) {
		let head = &self.head;
		let mut new_head: Option<Rc<RefCell<Node>>> = None;

		match head {
			Some(head) => {
				let node = head.as_ref().borrow();
				if let Some(node) = &node.next {
					let a = Rc::clone(&node);
					new_head = Some(a);
				}
			}
			None => {}
		}

		self.head = new_head;
		self.len -= 1;
  	}

	fn pop_back(&mut self) {
		let mut start_node: Option<Rc<RefCell<Node>>> = None;
		let mut delete_node: Option<Rc<RefCell<Node>>> = None;

		if let Some(x) = &self.head {
			start_node = Some(Rc::clone(x));
		}

		'a: loop {
			let mut temp: Option<Rc<RefCell<Node>>> = None;

			match &start_node {
				Some(a) => {
					let clone_a = a.borrow_mut();
					if clone_a.has_next() {
						delete_node = Some(Rc::clone(a));
						let n = clone_a.next();
						temp = n;
					} else {
						break 'a; 
					}
				}
				None => {
					break 'a;
				}
			}

			start_node = temp;
		}

		if let Some(x) = &delete_node {
			let mut x = x.borrow_mut();
			x.delete_next();
			self.len -= 1;
		}

		self.tail = delete_node;
  	}
}

impl Node {
	fn new(name: &str) -> Self {
		Self {
			name: name.to_string(),
			next: None
		}
	}

	fn create_next(&mut self, node: Rc<RefCell<Node>>) {
		self.next = Some(node);
	}

	fn create_next_from_copy(&mut self, node: &Rc<RefCell<Node>>) {
		self.next = Some(Rc::clone(node));
	}

	fn has_next(&self) -> bool {
		if let Some(_) = self.next {
			return true;
		}
		false
	}

	fn delete_next(&mut self) {
		self.next = None;
	}

	fn next(&self) -> Option<Rc<RefCell<Node>>> {
		if let Some(next) = &self.next {
			return Some(Rc::clone(next));
		}

		None
	}
}



fn main() {
  let mut ll = LinkedList::new();
  let node = Rc::new(RefCell::new(Node::new("sankar")));
  ll.push_back(node);
  let node = Rc::new(RefCell::new(Node::new("arun")));
  ll.push_back(node);
  let node = Rc::new(RefCell::new(Node::new("boro")));
  ll.push_back(node);
  let node = Rc::new(RefCell::new(Node::new("bipul")));
  ll.push_back(node);
//   ll.pop_back();
  println!("Head: {:?}", ll.head);
  println!("Tail: {:?}", ll.tail);
}