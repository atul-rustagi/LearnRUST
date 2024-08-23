
use std::rc::Rc;
use std::cell::RefCell;

type Link <T> = Option <Rc <RefCell <Node <T>>>>;

pub struct Node <T> {
    val: T,
    next: Link <T>
}

impl <T> Node <T> {
    fn new (val: T) -> Self
    {
        Node {val: val, next: None}
    }
}

pub struct List <T> {
    numelems: u32,
    head: Link <T>,
    tail: Link <T>
}

impl <T> List <T> {
    pub fn new () -> Self
    {
        List {numelems: 0, head: None, tail: None}
    }

    pub fn numelems (&self) -> u32
    {
        self.numelems
    }

    pub fn insert_as_last (&mut self, val: T)
    {
        let node = Rc::new (RefCell::new (Node::new (val)));

        if let Some (end) = self.tail.take () {
            end.borrow_mut ().next = Some (node.clone ());
        } else {
            self.head = Some (node.clone ());
        }

        self.tail = Some (node);

        self.numelems += 1;
    }

    pub fn insert_as_first (&mut self, val: T)
    {
        let node = Rc::new (RefCell::new (Node::new (val)));

        if self.head.is_some () {
            node.borrow_mut ().next = self.head.take ();
        } else {
            self.head = Some (node.clone ());
        }

        self.head = Some (node);

        self.numelems += 1;
    }

    pub fn insert (&mut self, val: T)
    {
        self.insert_as_last (val);
    }

    pub fn delete_first (&mut self) -> Link <T>
    {
        if self.head.is_none () {
            return None;
        }

        let first = self.head.take ().unwrap ();

        if Rc::ptr_eq(&first, &self.tail.as_ref ().unwrap ()) {
            self.tail = None;
            self.numelems -= 1;
            return Some (first);
        }

        let next_node = first.borrow_mut ().next.take ();
        self.head = next_node;
        self.numelems -= 1;

        Some (first)
    }

    pub fn delete_last (&mut self) -> Link <T>
    {
        if self.tail.is_none () {
            return None;
        }

        let last = self.tail.take ().unwrap ();

        if Rc::ptr_eq (&last, &self.head.as_ref ().unwrap ()) {
            self.head = None;
            self.numelems -= 1;
            return Some (last);
        }

        let mut current = self.head.clone ().unwrap ();
        loop {
            let next = current.borrow ().next.clone ().unwrap ();
            if Rc::ptr_eq (&next, &last) {
                break;
            }
            current = next;
        }

        current.borrow_mut ().next = None;
        self.tail = Some (current);
        self.numelems -= 1;

        Some (last)
    }

    // implement peek, peek mut, iter, iter mut
}

impl <T> Drop for List <T> {
    fn drop (&mut self)
    {
        let mut next = self.head.take ();

        while let Some (node) = next {
            next = node.borrow_mut ().next.take ();
            self.numelems -= 1;
        }

        self.tail = None;
    }
}

impl <T: std::fmt::Debug> List <T> {
    pub fn print (&self)
    {
        let mut current = self.head.as_ref ().map (|node| node.clone ());

        while let Some (node) = current {
            let node_ref = node.borrow ();
            println! ("{:?}", node_ref.val);
            current = node_ref.next.as_ref ().map (|next_node| next_node.clone ());
        }
    }
}

mod test {
    #[test]
    fn basics () {
        use crate::linked_list::List;

        let mut list = List::new ();

        list.insert (40);
        list.insert_as_first (30);
        list.insert (50);
        list.insert_as_first (20);
        list.insert_as_last (60);
        list.insert_as_first (10);

        list.print ();
        println! ("number of elements: {}", list.numelems ());

        drop (list);
    }
}
