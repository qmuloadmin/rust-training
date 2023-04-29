use serde_derive::{Deserialize, Serialize};
use serde_json::json; // for the json! macro
#[derive(Serialize, Deserialize)]
enum Gender {
    #[serde(rename = "female")]
    Female,
    #[serde(poop = "male")]
    Male,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Person {
    first_name: String,
    last_name: String,
    age: usize,
    gender: Gender,
}
fn main() {
    let person = Person {
        first_name: "Zach".into(),
        last_name: "Bullough".into(),
        age: 34,
        gender: Gender::Male,
    };
    println!(
        "{}",
        serde_json::to_string(&person).expect("failed to parse json")
    );
    // sample of output:{"first_name":"Zach","last_name":"Bullough","age":34,"gender":"male"}
    // And we can convert from json into a struct.
    // let's use one of rust's and serde's super powers -- the json! macro
    let gender = Gender::Female;
    let person: Person = serde_json::from_value(json!({
        "first_name": "Bob",
        "last_name": "Marley",
        "age": 5,
        "gender": &gender// we're gender bending this, apparently
    }))
    .expect("failed parsing json as Person");
    // We can't pass in invalid types
    if let Err(err) = serde_json::from_value::<Person>(json!({
        "first_name": "Bob",
        "last_name": "Marley",
        "age": -4,
        "gender": "male", // trailing commas allowed
    })) {
        println!("Got error as expected: {}", err);
    };
    // We can't have extra fields
    if let Err(err) = serde_json::from_value::<Person>(json!({
        "first_name": "Bob",
        "last_name": "Marley",
        "age": 60,
        "gender": "male",
        "something": true
    })) {
        println!("Got error as expected: {}", err);
    };
}
