#[derive(Debug)]
pub struct User {
    pub username: String,
    pub email: String,
    pub sign_in_count: u64,
    pub active: bool,
}

pub struct ManageUser {
    pub users: Vec<User>,
    pub id: u8,
}

impl ManageUser {
    pub fn new(users: Vec<User>, id: u8) -> Self {
        Self { users, id }
    }
}
impl User {
    pub fn new(username: String, email: String, sign_in_count: u64, active: bool) -> Self {
        Self {
            username,
            email,
            sign_in_count,
            active,
        }
    }
}
