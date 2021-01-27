extern crate frank_jwt;
#[macro_use]
extern crate serde_json;

use std::env;

use frank_jwt::{Algorithm, encode, decode, ValidationOptions};

fn main() {
    //RS256
    let payload = json!({
    "key1": "val1",
    "key2": "val2"});

    let header = json!({});
    let mut keypath = env::current_dir().unwrap();
    keypath.push("key_test");
    keypath.push("jwtRS256.key");
    let jwt = encode(header, &keypath.to_path_buf(), &payload, Algorithm::RS256);
    println!("{}", jwt.clone().unwrap());

    let mut keypath_pub = env::current_dir().unwrap();
    keypath_pub.push("key_test");
    keypath_pub.push("jwtRS256.key.pub");
    let result = decode(&jwt.unwrap(), &keypath_pub.to_path_buf(), Algorithm::RS256, &ValidationOptions::dangerous());
    println!("{}", &result.unwrap().1);
}
