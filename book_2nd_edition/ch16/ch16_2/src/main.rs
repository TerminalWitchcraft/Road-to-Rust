use std::sync::mpsc;
use std::thread;
use std::time::Duration;


fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let val = vec![
          String::from("Hii!!!"),
          String::from("Hello"),
          String::from("there"),
          String::from("wow"),
          String::from("damn"),
        ];
        for item in val {
            tx.send(item).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
        // tx.send(val).unwrap();
        // println!("The value is: {:?}", val);
        // thread::sleep(Duration::from_millis(200));
    });
    thread::spawn(move || {
        let val = vec![
          String::from("One!!!"),
          String::from("Two"),
          String::from("Three"),
          String::from("Four"),
          String::from("Five"),
        ];
        for item in val {
            tx1.send(item).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
        // tx.send(val).unwrap();
        // println!("The value is: {:?}", val);
        // thread::sleep(Duration::from_millis(200));
    });

    for receive in rx {
        println!("Got: {:?}", receive);
    }
    // let received = rx.recv().unwrap();
    // println!("Got {:?}", received);

}
