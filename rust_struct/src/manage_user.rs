use crate::user::User;
pub struct ManageUser {
    pub users: Vec<User>,
    pub id: u8,
}

impl ManageUser {
    pub fn new(users: Vec<User>, id: u8) -> Self {
        Self { users, id }
    }

   pub  fn add_user(&mut self,  user: User){
        self.users.push(user);
    }
}