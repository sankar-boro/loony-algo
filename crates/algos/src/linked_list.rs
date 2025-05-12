use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node<T: Clone> {
    value:T,
    next: Option<Rc<RefCell<Node<T>>>> 
}

#[derive(Debug)]
struct LinkedList<T: Clone> {
    len: i32,
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>
}

impl<T: Clone> LinkedList<T> {
    fn new() -> LinkedList<T> {
        LinkedList { len: 0, head: None, tail: None }
    }

    pub fn prepend(&mut self, val: T) {
        if let None = &self.head {
            let x = Rc::new(RefCell::new(Node { value: val, next: None }));
            self.head = Some(x.clone());
            self.tail = Some(x.clone());
            self.len += 1;
        } else if let Some(_) = &self.head {
            let head = self.head.take();
            self.head = Some(Rc::new(RefCell::new(Node { value: val, next: head })));
            self.len += 1;
        }
    }

    pub fn append(&mut self, val: T) {
        let y = Rc::new(RefCell::new(Node { value: val, next: None }));

        if let Some(_) = &self.tail {
            let tail = self.tail.take().unwrap();
            let mut tail = tail.borrow_mut();
            tail.next = Some(y.clone());
            self.tail = Some(y);
            self.len += 1;
        } else {
            self.head = Some(y.clone());
            self.tail = Some(y);
        }
    }

    pub fn len(&mut self) -> i32 {
        self.len
    }

    pub fn get_values(&mut self) -> Vec<T> {
        let mut r: Vec<T> = vec![];
        let mut d = self.head.clone();

        while let Some(v) = d {
            let x = v.borrow();
            r.push(x.value.clone());
            d = x.next.clone();
        }

        r
    }
}

pub fn run() {
    let mut list: LinkedList<i32> = LinkedList::new();
    list.prepend(9);
    list.prepend(11);
    list.prepend(3);

    list.append(5);

    let data = list.get_values();
    println!("{:?}", data);

}