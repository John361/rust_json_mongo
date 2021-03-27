use mongodb::{
    bson::{
        Bson,
        oid::ObjectId,
        Document as BsonDocument,
        to_bson as to_bson
    },
    sync::{Client, Collection, Database},
    error as mongoError
};
use serde::{Serialize};

const MONGO_DB: &str = "rust";
const MONGO_URI: &str = "mongodb://rustUser:rustPass@localhost/rust?w=majority";

pub struct MongoDomain {
    collection: String
}

impl MongoDomain {
    pub fn new(collection_name: String) -> MongoDomain {
        MongoDomain {
            collection: collection_name
        }
    }
}

impl MongoDomain {
    fn connect(&self) -> Database {
        let client: Client = Client::with_uri_str(MONGO_URI)
            .expect("Cannot connect to database");

        client.database(MONGO_DB)
    }

    fn get_collection(&self) -> Collection {
        self.connect().collection(self.collection.as_str())
    }

    pub fn insert(&self, serializable: &impl Serialize) -> Result<bool, mongoError::Error> {
        let bson: Bson = to_bson(&serializable).unwrap_or_else(|error| {
            panic!("Given object cannot be serialized: {:?}", error);
        });

        let document: BsonDocument = bson.as_document().unwrap_or_else(|| {
            panic!("Given object cannot be converted as document");
        }).clone();

        match self.get_collection().insert_one(document, None) {
            Ok(_) => Ok(true),
            Err(error) => Err(error)
        }
    }
}
