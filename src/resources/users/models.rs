use std::fmt::Debug;

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: usize,
    pub name: String,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Users {
    pub users: Vec<User>
}
