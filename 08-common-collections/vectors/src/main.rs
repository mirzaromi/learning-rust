fn main() {
    let vector: Vec<i32> = Vec::new();

    let vec2 = vec![1,2,3];

    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    let mut v = vec![1,2,3,4,5];

    // let third: &i32 = &v[2];
    
    let third_again: Option<&i32> = v.get(10);

    match third_again {
        Some(third_again) => println!("The third element is {third_again}"),
        None => println!("There is no third element.")
    }

    // println!("The third value is {third}");

    v.push(6);

    // println!("The third value is {}", third);

    for i in &v {
        println!("value = {}", i);
    }

    for i in &mut v {
        println!("value = {}", i);
        *i += 50;
        println!("new Value = {}", i);

    }

}
