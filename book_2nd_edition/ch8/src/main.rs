use std::io;


fn avg() {
    println!("Program to calculate the average of integers!");
    let mut list: Vec<f64> = Vec::new();
    let mut answer = String::from("yes");
    while answer.trim() == "yes" {
        println!("Enter a number: ");
        let mut num = String::new();
        io::stdin().read_line(&mut num)
            .expect("Failed to input number");
        let num: f64 = match num.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        list.push(num);
        println!("More?(yes/no)");
        answer.clear();
        io::stdin().read_line(&mut answer)
            .expect("Failed to input");
        //println!("You entered {:?}", answer);
    }
    let avg = average(list);
    println!("Broke from loop");
    println!("The average of the entered numbers are: {}", avg)
}

fn average(x:Vec<f64>) -> f64{
    let mut sum: f64 = 0.0;
    let len: f64 = x.len() as f64;
    for i in &x{
        sum += i;
    }
    sum / len
}

#[derive(Debug)]
struct Employee{
    name: String,
    company: String,
}

fn employee_details(){
    println!("FUnction to enter employee details..!");
    let mut emp_details: Vec<Employee> = Vec::new();
    let mut answer = String::from("yes");
    while answer.trim() == "yes" {
        println!("Enter employee details:");
        let mut name = String::new();
        let mut company = String::new();
        println!("Enter name:");
        io::stdin().read_line(&mut name).expect("Faled to input");
        println!("Enter company:");
        io::stdin().read_line(&mut company).expect("Failed to input");
        let user_data = Employee{
            name: String::from(name.trim()),
            company: String::from(company.trim()),
        };
        emp_details.push(user_data);
        println!("More(yes/no)");
        answer.clear();
        io::stdin().read_line(&mut answer).expect("Failed to input");
    }
    for d in &emp_details{
        println!("{:?}", d);
    }
}

fn pig_latin(){
    println!("Function to return pig latin version of word!: ");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input)
        .expect("Failed to input");
    let input = String::from(user_input.trim());
    let vowels = ["a", "e", "i", "o", "u"];
    //println!("{:?}", &input[0..1]);
    //for i in input.chars(){
    //    println!("Got {}", i);
    //}
    if vowels.contains(&&input[0..1]) {
        let final_str = String::from(&input[1..]) + "-" + "hay";
        println!("{}", final_str);
        //final_str.push("")
        //do somthing
    } else {
        //do another thing
        let final_str = String::from(&input[1..]) + "-" + &&input[0..1] +  "ay";
        println!("{}", final_str);
    }
}

fn main(){
    let mut answer = String::new();
    println!("Enter your choice: ");
    println!("1. Average");
    println!("2. Employee details..");
    println!("3. Pig Latin..");
    io::stdin().read_line(&mut answer).expect("Failed to input");
    let choice: u32 = match answer.trim().parse() {
        Ok(num) => num,
        Err(_) => 1,
    };
    match choice {
        1 => avg(),
        2 => employee_details(),
        3 => pig_latin(),
        _ => avg(),
    }
}
