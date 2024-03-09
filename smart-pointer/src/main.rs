use std::{ops::Deref, rc::Rc};

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[derive(Debug)]
enum List2 {
    Cons2(i32, Rc<List2>),
    Nil2,
}

use crate::List::{Cons, Nil};
use crate::List2::{Cons2, Nil2};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    // let b = Box::new(5);
    // println!("Hello, world! b = {b}");

    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // println!("List: {:?}", list);

    // let x = 5;
    // let y = &x;

    // assert_eq!(5, x);
    // println!("Llegue al valor: {y}");
    // println!("Llegue al valor: {}", *y);
    // assert_eq!(5, *y);
    // --------------------------------------
    // let x = 5;
    // let y = MyBox::new(x);
    // assert_eq!(5, x);
    // assert_eq!(5, *y);

    // let m = MyBox::new(String::from("Rust"));
    // hello(&m);
    // --------------------------------------
    let a = Rc::new(Cons2(5, Rc::new(Cons2(10, Rc::new(Nil2)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons2(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons2(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}
