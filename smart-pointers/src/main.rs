use std::{fmt::Display, ops::{Deref, Drop}};

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
        }
        Nil => {
            println!("Nil");
        }
    }
}

struct MyBox<T: Display>(T);

impl<T> MyBox<T> where T: Display {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> where T: Display {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> Drop for MyBox<T> where T:Display {
   fn drop(&mut self) {
         println!("Dropping MyBox with data: {}", self.0);
   }
}


struct MyStruct{
    data: i32
}

impl Display for MyStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MyStruct with data: {}", self.data)
    }
}

fn main() {
    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // print_list(list);

    let x = 5;

    let y = MyBox::new(x);

    let m = MyStruct{data: 10};

    let m_box = MyBox::new(m);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    // assert_eq!(m, *m_box);

    drop(m_box);

    println!("End of main");


}
