#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: usize,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Users {
    pub users: Vec<User>
}

impl Users {
    pub fn get_users(self) -> Vec<User> {
        self.users
    }
}
