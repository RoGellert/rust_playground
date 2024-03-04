fn main() {
    let hello1 = String::from("こんにちは");
    let hello2 = String::from("안녕하세요");
    let hello3 = String::from("你好");
    let hello4 = String::from("Olá");
    let hello5 = String::from("Здравствуйте");

    println!("Hello in different languages is: 
        \n {}, \n {}, \n {}, \n {}, \n {} \n", hello1, hello2, hello3, hello4, hello5);


    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s)
}
