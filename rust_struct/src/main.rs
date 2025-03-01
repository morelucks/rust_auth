#[allow(dead_code)] 
use rust_struct::User;
fn main() {

    let user1 = User::new(
        "Lucky".to_string(),
        "l@gmail".to_string(),
        2,
        true,
    );
    println!("USER...");

    println!("{:?}", user1);


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
