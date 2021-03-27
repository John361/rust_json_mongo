use mongodb::{
    bson::{doc, Bson},
    sync::{Client, Collection, Database}
};

pub mod user;
pub mod mongo;

const MONGO_DB: &str = "rust";
const MONGO_URI: &str = "mongodb://rustUser:rustPass@localhost/rust?w=majority";


fn connect_mongo() {
    let client: Client = Client::with_uri_str(MONGO_URI).expect("Cannot connect to database");
    let database: Database = client.database(MONGO_DB);
    let collection: Collection = database.collection("books");

    let docs = vec![
        doc! { "title": "1984", "author": "George Orwell" },
        doc! { "title": "Animal Farm", "author": "George Orwell" },
        doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
    ];

    collection.insert_many(docs, None).expect("Cannot insert into collection");

    let cursor = collection.find(doc! { "author": "George Orwell" }, None).expect("Cannot found document");
    for result in cursor {
        match result {
            Ok(document) => {
                if let Some(title) = document.get("title").and_then(Bson::as_str) {
                    println!("title: {}", title);
                } else {
                    println!("no title found");
                }
            }
            Err(e) => println!("Error {:?}", e)
        }
    }
}
