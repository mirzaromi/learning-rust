fn main() {
    let c = CustomSmartPointer {
        data: String::from("My Stuff")
    };
    
    let d = CustomSmartPointer {
        data: String::from("other stuff")
    };
    
    println!("CustomSmartPointerCreated");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");

}

struct CustomSmartPointer {
    data: String
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("dropping CustomSmartPointer with data `{}`", self.data)
    }
}

