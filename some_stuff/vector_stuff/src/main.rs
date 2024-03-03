fn main() {
    let mut vec: Vec<u32> = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    vec.push(5);

    for i in vec.iter_mut() {
        *i *= *i;
    }

    println!("Vec is {:?}", vec);
}
