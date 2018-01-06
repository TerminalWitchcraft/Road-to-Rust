use std::fs::File;
use std::io;
use std::io::Read;
use std::io::ErrorKind;

fn main() {
    let file_name = "hello.txt";
    let data = match read_username_from_file() {
        Ok(file) => file,
        Err(e) => panic!("Error: {:?}", e),
    };
    
    //let f = File::open(&file_name);
    //let f = match f {
    //    Ok(file)            => file,
    //    Err(ref error) if error.kind() == ErrorKind::NotFound => {
    //        match File::create(&file_name) {
    //            Ok(file)     => file,
    //            Err(e)       => panic!("Could not create file! {:?}", e),
    //        }
    //    },
    //    Err(e) => panic!("There was some other problem! {:?}", e),
    //};
    //panic!("crash and burn");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
