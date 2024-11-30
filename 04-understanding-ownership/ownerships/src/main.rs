fn main() {
    // let x = true;

    // read(x);
    // add_suffix_caller();

    dereference();
}

// fn read (y: bool) {
//     if y {
//         println!("y is true!");
//     }
// }

// fn add_suffix_caller() {
//     let first = String::from("Ferris");
//     let first_clone = first.clone();
//     let full = add_suffix(first_clone);
//     println!("{full}, originally {first}");
// }

// fn add_suffix(mut name: String) -> String {
//     name.push_str(" Jr.");
//     name
// }


fn dereference() {
    let x: Box<i32> = Box::new(10);
    let x_abs1 = i32::abs(*x);
    let x_abs2 = x.abs();
    assert_eq!(x_abs1, x_abs2);
    
    let r: &Box<i32> = &x;
    let r_abs1 = i32::abs(**r);
    let r_abs2 = r.abs();
    assert_eq!(r_abs1, r_abs2);

    let s = String::from("Hello");
    let s_len1 = str::len(&s);
    let s_len2 = s.leng();
    assert_eq!(s_len1, s_len2);
}
