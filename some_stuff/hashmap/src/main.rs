use std::collections::HashMap;

fn main() {
    let mut scores_map = HashMap::new();

    scores_map.insert(String::from("Blue"), 10);
    scores_map.insert(String::from("Yellow"), 50);

    let team_name1 = String::from("Blue");
    let team_name2 = String::from("Bluee");

    let score1 = scores_map.get(&team_name1).copied().unwrap_or(0);
    let score2 = scores_map.get(&team_name2).unwrap_or(&0);

    println!("Hashmap is {:?}", scores_map);
    println!("First num is {}", score1);
    println!("Second num is {}", score2);
}
