use mongodb::{
    bson::{doc, Bson},
    sync::{Client, Collection, Database}
};
use mongodb::results::InsertOneResult;

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
    fn connect(&self) -> mongodb::sync::Database {
        let client: mongodb::sync::Client = Client::with_uri_str(MONGO_URI)
            .expect("Cannot connect to database");

        client.database(MONGO_DB)
    }

    fn get_collection(&self) -> mongodb::sync::Collection {
        self.connect().collection(self.collection.as_str())
    }

    pub fn insert(&self, serializable: &impl Serialize) -> Result<mongodb::bson::oid::ObjectId, mongodb::error::Error> {
        let bson: mongodb::bson::Bson = mongodb::bson::to_bson(serializable)?;
        let document: &Document = bson.as_document()?;
        let result: mongodb::results::InsertOneResult = self.get_collection().insert_one(document, None)?;
        let id = result.inserted_id.as_object_id()?;

        Ok(id)
    }
}
