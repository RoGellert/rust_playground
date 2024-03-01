fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn main() {
    let mut s = String::from("Some words");

    let word_index = first_word(&s);

    println!("{}", word_index);

    let some = &s[0..5];
    let words = &s[5..10];

    println!("{} {}", some, words);

    s.clear()
}