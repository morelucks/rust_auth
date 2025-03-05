#[allow(dead_code)]
use rust_struct::manage_user::ManageUser;
use rust_struct::user:: User;
use sha2::Digest;

fn main() {
    let mut hasher =sha2::Sha256::new();

    let mut usr = ManageUser::new(
        vec![
            User::new("luck".to_string(), "luc@gmail.com".to_string(), 1, true),
            User::new("lk".to_string(), "lc@gmail.com".to_string(), 1, true),
            User::new("ck".to_string(), "c@gmail.com".to_string(), 1, true),
        ],
        2,
    );
    usr.add_user(User::new("luck".to_string(), "luc@gmail.com".to_string(), 1, true));
    
    println!("USER...");
    for user in usr.users{
        // hasher.update(user);
        println!("{:?}", user);

    }

    // println!("{:?}", usr.users[0]);
    // let user1 = User::new(
    //     "Lucky".to_string(),
    //     "l@gmail".to_string(),
    //     2,
    //     true,
    // );
    // println!("USER...");

    // println!("{:?}", usr.users[0]);

    // ..... STRUCT AND OWNERSHIP ..... //

    // A value can only have one owner at a time.
    // When the owner goes out of scope, the value is dropped
    // let s1 = String::from("hello");
    // let s2 = s1;  // Ownership moves from s1 to s2

    // println!("{}", s1);
    //..... grouping related data.....//
    // let user1 = User{
    //     username: String::from("alice"),,
    //     email: String::from("alice@example.com"),
    //     sign_in_count: 1,
    //     active: true,
    // };
    // let user2 = User {
    //     username: String::from("alice"),
    //     email: String::from("alice@example.com").clone(),
    //     ..user1
    // };
    // let user1_name=user1.email.clone();

    // println!("{:?}", user2);
    // println!("{user1.name}");
}
