use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    // let m = Mutex::new(5);

    // {
    //     let mut num = m.lock().unwrap();

    //     *num = 6;

    //     println!("num = {:?}", num);
    // }

    // println!("m = {:?}", m);


    let counter: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10{
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move ||{
            let mut num = counter.lock().unwrap();

            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles{
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
