use std::thread;
use std::time::Duration;


fn calc(x: u32) -> () {
    println!("Calculating {}", x);
    ()
}

fn main() {
    // let handle = thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("Hi, I am {} from the spawned thread", i);
    //         thread::sleep(Duration::from_millis(100));
    //     }
    // });
    // let handle = thread::spawn(|| {
    //     for i in 1..10 {
    //         calc(i as u32);
    //         thread::sleep(Duration::from_millis(100));
    //     }
    // });
    // for i in 1..5 {
    //     println!("Hi, I am from the main thread..and counting {}", i);
    //     thread::sleep(Duration::from_millis(100));
    // }

    let v = vec![1,2,3];
    let handle = thread::spawn(move || {
        println!("Here is a vector: {:?}", v);
    });
    handle.join().unwrap();
}
