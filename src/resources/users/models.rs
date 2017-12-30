extern crate serde_derive;

#[derive(Serialize, Deserialize)]
pub struct User {
    name: String
}

#[derive(Serialize, Deserialize)]
pub struct Users {
    users: Vec<User>
}
