fn main() {
    // let val = 10;

    // if val < 5 {
    //     println!("this is true condition!");
    // } else {
    //     println!("this is false condition!");
    // }
    // if_not_equal();
    // multiple_ifs(5);
    // assign_if_on_let();

    // let x;
    // if true {
    //     x = 5
    // } else {
    //     x = 7
    // }
    // println!("the value of x is : {x}");
    // loops();

    // loop_with_return_value();
    // loop_in_loop();
    // while_loop();
    // loop_through_collection();
    // for_each();
    for_loop();
}

// fn if_not_equal() {
//     let val = 7;

//     if val != 5 {
//         println!("the value is not equal to 5");
//     }
// }

// fn multiple_ifs(val: i32) {
//     if val % 4 == 0 {
//         println!("Number is divisible by 4");
//     } else if val % 3 == 0 {
//         println!("Number is divisible by 3");
//     } else if val % 2 == 0 {
//         println!("Number is divisible by 2");
//     } else {
//         println!("Number is not divisible by 2, 3, and 4");
//     }
// }

// fn assign_if_on_let() {
//     let condition = true;
//     let value = if condition { 5 } else { 6 };

//     println!("the value is : {}", value);
// }


// fn loops() {
//     loop {
//         println!("again!");
//     }
// }

// fn loop_with_return_value() {
//     let mut counter = 0;

//     let result = loop {
//         counter+=1;

//         if counter == 10 {
//             break counter * 2
//         }
//     };

//     println!("the result is {result}");
// }


// fn loop_in_loop() {
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining = 10;

//         loop {
//             println!("remaining = {remaining}");

//             if remaining == 9 {
//                 break;
//             }
//             if count == 8 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }
//         count+=1;
//     }
//     println!("End count = {count}");
// }

// fn while_loop() {
//     let mut number = 3;

//     while number != 0 {
//         println!("{number}");

//         number -= 1;
//     }

//     println!("LIFTOFF!!!");
// }

// fn loop_through_collection() {
//     let a = [10,20,30,40];
//     let mut index = 0;

//     while index < 5 {
//         println!("the value is : {}", a[index]);

//         index +=1;
//     }
// }

// fn for_each() {
//     let a = [10, 20, 30, 40, 50, 60];

//     for element in a {
//         println!("the value is : {element}");
//     }
// }

// fn for_loop() {
//     for number in (1..=10).rev().step_by(2) {
//         println!("{number}");
//     }
//     println!("LIFTOFF!!!!");
// }