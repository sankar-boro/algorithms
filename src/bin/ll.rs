#[derive(Debug, Clone)]
struct User {
  name: String,
  next: Option<Box<User>>,
}

impl User {
  fn new(name: String) -> Self {
    Self {
      name,
      next: None,
    }
  }

  fn append(&mut self, name: User) {
    self.next = Some(Box::new(name));
  }
}

#[derive(Debug)]
struct LinkedList {
  head: Option<Box<User>>,
  tail: Option<Box<User>>,
  node: Option<Box<User>>,
  len: usize,
}

impl LinkedList {
  fn new(user: User) -> Self {
    let user = Some(Box::new(user));
    LinkedList {
      head: user.clone(),
      tail: user.clone(),
      node: user,
      len: 1,
    }
  }

  fn push(&self, user: User) {
    
  }

  fn append(&self, user: User) {}

  fn prepend(&self, user: User) {}
}

fn main() {
  let users = User::new(String::from("sankar"));
  let ll = LinkedList::new(users);
  println!("{:?}", ll);
}