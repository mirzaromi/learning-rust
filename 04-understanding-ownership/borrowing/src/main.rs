fn main() {
    // dereference();
    // unique_reference();

    // let mut vec = vec!['m', 'i', 'r', 'z', 'a'];
    // ascii_capitalize(&mut vec);
    // println!("Capitalized vector: {:?}", vec);

    // first_or_caller();
    borrows3();
}

// fn borrows() {
//     let mut v: Vec<i32> = vec![1, 2, 3];
//     let mut num: &mut i32 = &mut v[2];
//     *num = 10;
//     v[2] = 10;
//     let new_num = v[0];
//     println!("Third element is {}", v[2]);
//     // println!("Third element is {}", *num);

// }

fn borrows3() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &mut i32 = &mut v[2];
    *num += 1;
    let val2 = v[10];
    println!("Third val2 is {}", val2);

    println!("Third element is {}", *num);
    println!("Vector is now {:?}", v);

}

// fn borrows2() {
//     let mut x = 5;
//     let y = &mut x; 
//     *y += 1;        
//     println!("{}", x); 
//     *y += 1;        
// }

// fn borrows2() {
//     let mut x = 5;
//     let y = & x; // Mutable borrow of `x`
//     println!("{}", x); // Works because the mutable borrow `y` is no longer used
//     println!("{}", y);
// }

// fn dereference() {
//     let x: Box<i32> = Box::new(10);
//     let x_abs1 = i32::abs(*x);
//     let x_abs2 = x.abs();
//     assert_eq!(x_abs1, x_abs2);
    
//     let r: &Box<i32> = &x;
//     let r_abs1 = i32::abs(**r);
//     let r_abs2 = r.abs();
//     assert_eq!(r_abs1, r_abs2);

//     let s = String::from("Hello");
//     let s_len1 = str::len(&s);
//     let s_len2 = s.len();
//     assert_eq!(s_len1, s_len2);
// }

// fn unique_reference() {
//     let mut v: Vec<i32> = vec![1, 2, 3];
//     let num: &mut i32 = &mut v[2];

//     *num += 1;

//     println!("Third element is {}", num);
//     println!("Vector is now {:?}", v);
// }

// fn unique_reference() {
//     let mut v: i32 = 10;
//     let num: &mut i32 = &mut v;

//     *num += 1;
//     println!("Third element is {}", num);
//     println!("Vector is now {:?}", v);
// }

// fn ascii_capitalize(v: &mut Vec<char>) {
//     let c = &v[0];
//     if c.is_ascii_lowercase() {
//         let up = c.to_ascii_uppercase();
//         v[0] = up;
//     } else {
//         println!("Already capitalized: {:?}", v);
//     }
// }


// fn first_or_caller() {
//     let strings = vec![];
//     let default = String::from("default");
//     let s = first_or(&strings, &default);
//     drop(default);

//     println!("{}", s);


// }

// fn first_or<'a>(strings: &'a Vec<String>, default: &'a String) -> &'a String {
//     if strings.len() > 0 {
//         &strings[0]
//     } else {
//         default
//     }
// }