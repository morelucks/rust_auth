#[derive(Debug)]
pub struct User {
    pub username: String,
    pub email: String,
    pub sign_in_count: u64,
    pub active: bool,
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
