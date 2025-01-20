use std::collections::HashMap;

fn main() {
    // let v: Vec<i32> = Vec::new();
    // let v = vec![1, 2, 3, 4];

    // let mut v = Vec::new();
    
    // v.push(1);
    // v.push(2);
    // v.push(3);
    // v.push(4);

    // let v = vec![1, 2, 3, 4, 5];
    // let third: &i32 = &v[2];
    // println!("The third element is {}", third);

    // match v.get(2) {
    //     Some(third) => println!("The third element is {}", third),
    //     None => println!("The third element is not found"),
        
    // }

   let w = "redis";
   
//    for b in w.bytes() {
//     println!("{}", b);
//    }

//    for b in w.chars() {
//     println!("{}", b);
//    }

//    let mut scores = HashMap::new();

//    scores.insert(String::from("Blue"), 10);
//    scores.insert(String::from("Yellow"), 50);

    // let teams = vec![String::from("Blue"), String::from("Yellow")];
    // let intial_scores = vec![10, 50];

    // let scores: HashMap<_, _> = 
    //     teams.iter().zip(intial_scores.iter()).collect();

    let field_name = String::from("Favorite Color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    println!("{}: {}", field_name, field_value);


}
