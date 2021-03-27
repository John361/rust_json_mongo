
use domain;

fn main() {
    domain::user::User::new(String::from("John"), String::from("Doe"), 10);

}

fn populate_json() {
    // TODO: create file, replace if exist
    // TODO: create User struct list with arbitrary value
    // TODO: write list as json into file
}

fn read_json() {
    // TODO: create file, call populate_json if not exists
    // TODO: read all lines as Vec<User>
}

// Create a new collection using given name
// Remove the collection if it already exists
fn create_collection(name: String) {

}

fn insert_db() {
    // TODO: add User struct as parameter
    // TODO: add it in database
}
