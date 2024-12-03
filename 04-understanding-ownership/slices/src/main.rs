fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    // s.clear(); // error!

    println!("the first word is: {word}");

    let word2 = "testing";
    // *word2 = "testing again";

    println!("the second word is: {word2}");



    let s = String::from("hello");
    let s2: &String = &s;
    let s3: &str = &s[..];
    println!(
        "&String={} &str={}",
        std::mem::size_of::<&String>(),
        std::mem::size_of::<&str>(),
    );
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
