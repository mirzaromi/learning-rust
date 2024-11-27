use std::io;

fn main() {
    // let mut x = 5;
    // println!("The value of x is {x}");

    // x = 10;
    // println!("The value of x is {x}");

    // let x = 5;

    // let x = x + 1;

    // {
    //     let x = x*2;
    //     println!("the value of x in the inner scope is {x}");
    // }

    // println!("the value of x is {x}");

    // let tup = (500, 6.4, 1);

    // let (x, y, z) = tup;
    
    // let x = tup.0;

    // let y = tup.1;

    // let z = tup.2;

    // println!("the value of x is {x}");
    // println!("the value of y is {y}");
    // println!("the value of z is {z}");

    // let tup: (i32, i32) = (0, 10);

    // let (mut x, mut y) = tup;

    // x += 5;
    // y += 5;

    // tup.0 += 5;
    // tup.1 += 5;

    // println!("the value of x = {} and the value of y = {}", x, y);

    // let arr = [1,2,3,4,5];

    // let months = ["January", "Februray", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    // let new_arr: [i32; 5] = [1,2,3,4,5];

    // let arr_2 = [2;5];

    // let index = 1;

    // println!("the value on index {index} = {}", arr_2[index]);


    accessing_array();
}

fn accessing_array() {
    let arr = [1,2,3,4,5];

    println!("please enter an array index!");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("failed to read line");

    let index: usize = index.trim()
                            .parse()
                            .expect("Index entered was not a number!");
    
    let element = arr[index];

    println!("the value of element at {index} is : {element}");
}
