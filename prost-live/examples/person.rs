use prost::Message;
use prost_live::pb::*;

fn main() {
    let phones = vec![PhoneNumber::new("123-111-123", PhoneType::Mobile)];
    let person = Person::new("xiao qiao", 1, "qiao4812@gmail.com", phones);
    let v1 = person.encode_to_vec();
    let person1 = Person::decode(v1.as_slice()).unwrap();
    let person11 = Person::decode(v1.as_ref()).unwrap();
    assert_eq!(person, person1);
    assert_eq!(person, person11);
    // println!("person1: {person1:?}");
    // let v2 = person.encode_length_delimited_to_vec();
    // println!("{person:?}, {v1:?}(len: {}), {v2:?}", v1.len());

    let json = serde_json::to_string_pretty(&person1).unwrap();
    println!("{json}");
}
