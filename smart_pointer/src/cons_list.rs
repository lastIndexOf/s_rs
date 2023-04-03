use std::rc::Rc;

use List::{Cons, Nil};

#[derive(Debug)]
pub enum List {
    Cons(i32, Rc<List>),
    Nil,
}

impl List {
    fn change_value(&mut self, val: i32) {
        match self {
            Cons(ref mut x, _) => {
                *x = val;
            }
            Nil => {}
        }
    }
}

pub fn test_cons_list() {
    let mut a = Cons(5, Rc::new(Cons(10, Rc::new(Nil))));

    a.change_value(100);

    let temp = Rc::new(Cons(2, Rc::new(a)));

    let b = Cons(1, Rc::clone(&temp));

    let c = Cons(3, Rc::clone(&temp));

    println!("b = {:?}", b);
    println!("c = {:?}", c);

    let mut d = String::new();
    // let mut e = &mut d;
    let f = &mut d;

    // println!("b = {:?}", e);
    println!("b = {:?}", f);
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    trait Send {
        fn send(&self, message: &str);
    }

    struct MessageSender {
        messages: RefCell<Vec<String>>,
    }

    impl MessageSender {
        fn new(messages: Vec<String>) -> Self {
            Self {
                messages: RefCell::<Vec<String>>::new(messages),
            }
        }

        fn borrow_error(&self) {
            // 会抛出错误
            // let mut borrow_1 = self.messages.borrow_mut();
            // println!("{:?}", borrow_1);

            // let borrow_2 = self.messages.borrow();
            // println!("{:?}", borrow_2);
        }
    }

    impl Send for MessageSender {
        fn send(&self, message: &str) {
            self.messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn test_send_message() {
        let sender = MessageSender::new(Vec::new());

        sender.send("Hello");
        sender.send("World");

        sender.borrow_error();

        assert_eq!(sender.messages.borrow().len(), 2);
        assert_eq!(*sender.messages.borrow(), vec!["Hello", "World"]);
    }

    #[test]
    fn test_borrow() {
        let mut a: Vec<&str> = vec![];

        a.push("hello");

        let c = &a;

        println!("{:?}", c);
    }
}
