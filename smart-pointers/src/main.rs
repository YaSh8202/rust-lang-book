use std::ops::Deref;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn print_list(list: List) {
    match list {
        Cons(i, l) => {
            println!("{}", i);
            print_list(*l);
        },
        Nil => {
            println!("Nil");
        }
    }
}


struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x:T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T>{
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // print_list(list);

    let x = 5;

    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

}
