use std::cell::RefCell;

struct Example {
    data : RefCell<i32>
}

fn main() {
    let example = Example {
        data: RefCell::new(10)
    };
    
    *example.data.borrow_mut() = 20;
    
    println!("Data = {}", example.data.borrow());
}
