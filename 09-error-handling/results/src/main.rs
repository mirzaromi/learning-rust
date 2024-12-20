use std::fs::{self, File};
use std::io::{ErrorKind, Read};

fn main() {
    // let greeting_file_result = File::open("hello.txt");

    // let greeting_file = greeting_file_result.unwrap_or_else(|error| match error.kind() {
    //     ErrorKind::NotFound => match File::create("hello.txt") {
    //         Ok(file) => file,
    //         Err(error) => panic!("Failed to create file: {:?}", error)
    //     },
    //     other_error => panic!("Failed to open file: {:?}", other_error)
    // });

    // let greeting_file = greeting_file_result.unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // });

    // let greeting_file_result = File::open("hello.txt")
    //     .expect("hello.txt should be included in this project");

    let greeting_file = File::open("hello.txt");

}

// fn read_username_from_file() -> Result<String, std::io::Error> {
//     let username_file_result = File::open("hello.txt");
//
//     let mut username_file = match username_file_result {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };
//
//     let mut username = String::new();
//
//     match username_file.read_to_string(&mut username) {
//         Ok(_) => Ok(username),
//         Err(err) => Err(err)
//     }
// }

// with ? operator to propagate error
fn read_username_from_file() -> Result<String, std::io::Error> {
    // let mut username_file = File::open("hello.txt")?;
    // let mut username = String::new();
    // username_file.read_to_string(&mut username)?;
    // Ok(username)

    // let mut username = String::new();
    // File::open("hello.txt")?.read_to_string(&mut username)?;
    // Ok(username)

    fs::read_to_string("hello.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
