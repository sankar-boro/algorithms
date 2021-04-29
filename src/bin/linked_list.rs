#[allow(unused_variables, unused_assignments)]
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
		if self.len == 1 {
			self.head = None;
			self.tail = None;
			self.len = 0;
			return;
		}

		let mut node: Option<Rc<RefCell<Node>>> = None;

		// if has_next node has child node, then we want to remove the child node from this node.
		// but if it does not have any, then we want to directly set this node as the tail of linked list.
		let mut has_next: Option<Rc<RefCell<Node>>> = None;

		if let Some(x) = &self.head {
			// a node exists
			node = Some(Rc::clone(x));
			self.len -= 1;
		} else {
			// node not found. nothing to remove, return
			return;
		}

		// If linked list length is greater then or equal to one, this loop and match condition will execute
		'a: loop {
			// next node at every loop should be None, because we haven't found any next node yet.
			// let mut next_node: Option<Rc<RefCell<Node>>> = None;
			
			node = match &node {
				Some(_node) => {
					let this = _node.borrow_mut();
					if this.has_next() {
						// ok this node has a child, but the child may not have another child node.
						// so we may want to remove the child of current node and therefore we need to set this 
						// node wherein we can remove an element from.
						has_next = Some(Rc::clone(_node));

						// if this node has linked node, then set node to this.next
						// remember, this is not parent node, this is child node of current node.
						this.next()
					} else {
						// lets say this is the first element and we do not have next node
						// from here we do not want to iter over next element, so we break from loop.
						// we do not want to set anything as whatever needs to be set has aldready been done.
						break 'a; 
					}
				}
				None => {
					break 'a;
				}
			}
		}

		if let Some(x) = &has_next {
			let mut x = x.borrow_mut();
			x.delete_next();
		}

		self.tail = has_next;
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
  ll.push_front(node);
  let node = Rc::new(RefCell::new(Node::new("bipul")));
  ll.push_back(node);
  ll.pop_back();
  println!("Head: {:?}", ll.head);
  println!("Tail: {:?}", ll.tail);
  println!("Len: {}", ll.len);
}