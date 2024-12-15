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


fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    print_list(list);
}
