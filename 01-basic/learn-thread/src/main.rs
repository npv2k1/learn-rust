use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn learn_chanel() {
    fn main() {
        let (tx, rx) = mpsc::channel();

        tx.send(10).unwrap();
        tx.send(20).unwrap();

        println!("Received: {:?}", rx.recv());
        println!("Received: {:?}", rx.recv());

        let tx2 = tx.clone();
        tx2.send(30).unwrap();
        println!("Received: {:?}", rx.recv());
    }
}

fn unbounded_chanel() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let thread_id = thread::current().id();
        for i in 1..10 {
            tx.send(format!("Message {i}")).unwrap();
            println!("{thread_id:?}: sent Message {i}");
        }
        println!("{thread_id:?}: done");
    });
    thread::sleep(Duration::from_millis(100));

    for msg in rx.iter() {
        println!("Main: got {}", msg);
    }
}

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("Count in thread: {i}!");
            thread::sleep(Duration::from_millis(5));
        }
    });

    for i in 1..5 {
        println!("Main thread: {i}");
        thread::sleep(Duration::from_millis(5));
    }
}
