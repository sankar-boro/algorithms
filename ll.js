class Node {
  constructor(name) {
    this.name = name;
    this.next = null;
  }

  append(_node) {
    this.next = _node;
  }
}

class LinkedList {
  constructor() {
    this.head = null;
    this.tail = null;
    this.len = 0;
  }

  push(node) {
    if (this.head === null) {
      this.head = node;
      this.tail = node;
      this.len += 1;
      return;
    }

    if (this.tail && this.tail.next === null) {
      this.tail.append(node);
    }

    this.tail = node;
    this.len += 1;
  }

  delete() {
    if (this.head === null) return;

    let head = this.head;
    let next = this.head.next;

    if (next === null) {
      this.head = null;
      this.tail = null;
      this.len = 0;
      return;
    }

    for (let index = 0; index < this.len; index++) {
      if (head.next === this.tail) {
        head.next = null;
        this.len--;
      } else {
        head = head.next;
      }
    }

    this.tail = head;
  }

  pop() {
    this.head = this.head.next;
    this.len--;
  }

  append(node) {
    let newNode = node;
    newNode.next = this.head;
    this.head = newNode;
    this.len++;
  }
}

let ll = new LinkedList();
let one = new Node("sankar");
let two = new Node("Arun");
let three = new Node("Bhabesh");
ll.push(one);
ll.push(two);
ll.append(three);
// ll.push(three);
console.log(ll);
