#[derive(Debug)]
pub struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User{
   pub fn new(username:String, email:String, sign_in_count:u64, active:bool)->Self{
    Self{
        username,
        email,
        sign_in_count,
        active,
    }
   }
}