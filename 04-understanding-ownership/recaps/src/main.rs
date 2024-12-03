fn main() {
    let words = vec!["hello".to_string()];
    let d = new_document(words);

    // .to_vec() converts &[String] to Vec<String> by cloning each string
    let words_copy = get_words(&d).to_vec();
    let mut d2 = new_document(words_copy);
    add_word(&mut d2, "world".to_string());

    // The modification to `d2` does not affect `d`
    assert!(!get_words(&d).contains(&"world".into()));

    let mut s = String::from("Hello");
    let s_ref = &mut s;
    println!("{s}");
    s_ref.push_str(" world");


}

type Document = Vec<String>;

fn new_document(words: Vec<String>) -> Document {
    words
}

fn add_word(this: &mut Document, word: String) {
    this.push(word);
}

fn get_words(this: &Document) -> &[String] {
    this.as_slice()
}

