fn main() {
    let mut s = String::new();

    let data = "initial contents";

    let s  = data.to_string();

    let s = "initial_contents".to_string();

    let s = String::from("initial contents");

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שלום");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    let mut s = String::from("hello");
    s.push_str("bar");


    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str("s2");
    println!("s2 is {}", s2);

    let mut s = String::from("lo");
    s.push('l');

    println!("s is {}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("s3 is {}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    println!("value of concatenate s is = {}", s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    println!("the value of s using format is {}", s);
    println!("value of s1 : {s1} s2 : {s2} s3 : {s3}");

    // indexing into Strings
    // let s1 = String::from("hello");
    // let h = s1[0];

    let hello = "Здравствуйте";
    let answer = &hello[0..1];

    println!("value of answer is : {}", answer);

    for c in "Здравствуйте".chars() {
        println!("char is : {}", c);
    }

    for c in "Здравствуйте".bytes() {
        println!("char (in byte) is : {}", c);
    }

}
