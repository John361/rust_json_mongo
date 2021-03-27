use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub first_name: String,
    pub last_name: String,
    is_active: bool
}

impl User {
    pub fn new(first_name: String, last_name: String, age: u8) -> User {
        if age > 110 {
            panic!("I am in panic just seeing this age!");
        }

        User {
            id: Some(ObjectId::new()),
            first_name,
            last_name,
            is_active: true
        }
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn collection_name() -> String {
        String::from("users")
    }
}
