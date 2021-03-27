use domain::{
    user as domain_user,
    mongo as domain_mongo
};
use std::str::FromStr;

fn main() {
    let mongo = domain_mongo::MongoDomain::new(domain::user::User::collection_name());
    let user = domain_user::User::new(String::from("John"), String::from("Doe"), 10);

    match mongo.insert(&user) {
        Ok(_) => println!("User was inserted"),
        Err(error) => println!("User was not inserted: {:?}", error)
    }

    match mongo.get_one_user(domain_mongo::ObjectId::from_str("605f3f77003f938b00690bcb").unwrap()) {
        Some(user) => println!("User found: {:#?}", user),
        None => println!("Cannot found user")
    }
}
