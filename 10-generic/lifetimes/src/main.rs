use std::fmt::Display;

fn main() {
    // let string1 = String::from("abcd");
    // let string2 = "xyz";
    //
    // let result = longest(string1.as_str(), string2);
    //
    // println!("The longest string is {result}");

    // let string1 = String::from("hello");
    // let result;
    //
    // {
    //     let string2 = String::from("world");
    //     result = longest(string1.as_str(), string2.as_str());
    // } // `string2` is dropped here, but `result` might point to `string2`.

    // println!("The longest string is: {}", result); // Potential dangling reference!

    // let string1 = String::from("hello");
    // let string2 = String::from("world");
    //
    // let result = longest(string1.as_str(), string2.as_str());
    //
    // println!("The longest string is: {}", result); // Safe!

    // let string1 = String::from("long string is long");
    //
    // {
    //     let string2 = String::from("xyz");
    //     let result = longest(string1.as_str(), string2.as_str());
    //     println!("The longest string is {result}");
    // }

    // let novel = String::from("Call me ishamel. Some years ago...");
    // let first_sentence = novel.split('.').next().unwrap();
    // let i = ImportantExcerpt {
    //     part: first_sentence
    // };

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest_with_an_annountcement(string1.as_str(), string2, String::from("processing input"));

    println!("The longest string is {result}");
}

// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() { x } else { y }
// }
//
// struct ImportantExcerpt<'a> {
//     part: &'a str
// }


fn longest_with_an_annountcement<'a, T> (
    x: &'a str,
    y: &'a str,
    ann: T
) -> &'a str
where T:Display {
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}