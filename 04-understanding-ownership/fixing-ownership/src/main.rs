fn main() {
    caller_stringify();
}

// fn return_a_string() -> &String {
//     let s = String::from("Hello world");
//     &s
// }


// fn stringify_name_with_title(name: &Vec<String>) -> String {
//     let mut name_clone = name.clone();
//     name_clone.push(String::from("Esq."));
//     let full = name_clone.join(" ");
//     full
// }

fn stringify_name_with_title(name: &Vec<String>) -> String {
    let mut full = name.join(" ");
    full.push_str(" Esq.");
    full
}
fn caller_stringify() {
    let name = vec![String::from("Ferris")];
    let first = &name[0];
    stringify_name_with_title(&name);
    println!("{}", first);
    println!("{:?}", name);

}