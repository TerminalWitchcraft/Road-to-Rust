extern crate rand;

use rand::Rng;
use std::thread;
use std::collections::HashMap;
use std::time::Duration;
use std::collections::hash_map::Entry;

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
    // Change to hashmap
    calculation: T,
    //value: Option<u32>
    value: HashMap<u32, u32>,
}

impl<T> Catcher<T>
where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Catcher<T> {
        Catcher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        // IMP NOTE
        // The issue with borrow checker can be soved by writing
        // #![feature(nll)] 
        // at the start(it is a limitation now, in future will be removed)
        //match self.value {
        //    Some(v) => v,
        //    None => {
        //        let v = (self.calculation)(arg);
        //        self.value = Some(v);
        //        v
        //    }
        //}
        //self.value.entry(arg).or_insert((self.calculation)(arg));
        //self.value.get(&arg).or_else

        //match self.value.get(&arg) {
        //    Some(v) => *v,
        //    None => {
        //        let v = (self.calculation)(arg);
        //        //self.value.insert(arg, v);
        //        self.value.entry(arg).or_insert_with(|| v);
        //        v
        //    }
        //}
        let v = match self.value.entry(arg) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert((self.calculation)(arg))
        };
        *v
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
