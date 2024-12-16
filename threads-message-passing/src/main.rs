use std::{sync::mpsc, thread, time::Duration, vec}; // multiple producer, single consumer

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx2 = tx.clone();

    thread::spawn(move || {
        // let msg = String::from("Hello from the other side");
        // tx.send(msg).unwrap();
        // println!("Sent: {}", msg); // This will give error because msg is moved to tx

        let vals = vec![
            String::from("Hi"),
            String::from("from"),
            String::from("the"),
            String::from("other"),
            String::from("side"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("More"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    // let received = rx.recv().unwrap();

    // println!("Got: {}", received);

    for received in rx {
        println!("Got: {}", received);
    }
}
