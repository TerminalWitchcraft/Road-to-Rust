extern crate rand;

use rand::Rng;
use std::thread;
use std::time::Duration;

//closure are type of lazy evaluators, similar to lambda in python
//It can also be used where singleton classes are required!

fn main() {
    let simulated_user_specified_value = rand::thread_rng().gen_range(1, 101);
    let simulated_random_number = rand::thread_rng().gen_range(1,5);
    generate_workout(simulated_user_specified_value,
                     simulated_random_number);
}

struct Catcher<T>
where T: Fn(u32) -> u32 {
    calculation: T,
    value: Option<u32>
}

impl<T> Catcher<T>
where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Catcher<T> {
        Catcher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_rs = Catcher::new(|num| {
        println!("Calculating slowly..");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!(
            "Today, do {} pushups",
            expensive_rs.value(intensity)
                );
        println!(
            "Today do {} situps",
            expensive_rs.value(intensity)
                );
    } else {
        if random_number == 3 {
            println!(
                "Take a break"
                    );
        } else {
            println!(
                "Today run for {} seconds",
                expensive_rs.value(intensity)
                    );
        }
    }
}
