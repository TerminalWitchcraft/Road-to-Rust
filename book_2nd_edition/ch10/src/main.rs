fn largest<T>(list: &[T]) -> &T 
    where T: PartialOrd {
    let mut largest = 0;
    for (i, v) in list.iter().enumerate() {
        if v > &list[largest] {
            largest = i;
        }
    }
    &list[largest]
}


fn main() {
    //let number_list = vec![34, 50, 77, 12];
    let number_list: Vec<i32> = Vec::new();
    let largest_num = largest(&number_list);
    println!("The largest number is {}", largest_num);
    let char_list = vec!['a', 'b', 'd', 'q'];
    //note, if we had renamed largest_num to largest,
    //it would cause an error!
    let largest = largest(&char_list);
    println!("The largest char is {}", largest);
}
