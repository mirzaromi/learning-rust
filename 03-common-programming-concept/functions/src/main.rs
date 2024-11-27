fn main() {
    // println!("Hello, world!");

    // another_function(15);

    // print_labeled_measuremenet(5, 'm');

    // let value = return_value();

    let value = plus_one(10);
    println!("the value of the function is {}", value);
}

// fn another_function(x: i32) {
//     println!("The value of x is {x}!");
// }

// fn print_labeled_measuremenet(value: i32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}");
// }

// fn return_value() -> i32 {
    

//     let y = {
//         let x = 3;
//         x + 1
//     };

//     7
// }

fn plus_one(val: i32) -> i32 {
    val+1
}