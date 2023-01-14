
pub fn structs() {
#[derive(Debug)]    // Opt in to use the Debug trait implementation
struct User {
    active: bool,
    username: String,
    email: String, 
    sign_in_count: u64,
}

// Instantiate
// Params can be give at any order
let user1 = User {
    email: String::from("example@mail.com"),
    active: false,
    username: String::from("voidptr_t"),
    sign_in_count: 21,
};

fn _build_user(email: String, username: String) -> User {
    // Like in js
    User {
        email,
        username,
        active: false, 
        sign_in_count: 0,
    }
}

// struct update syntax, again like in js
let _user2 = User {
    email: String::from("example+1@mail.com"),
    ..user1
};

println!("user2 = {:#?}", _user2);

// println!("{}", user1.username);
// results in a compile time error because user1.username has been moved

// tuple structs
struct Color(i32, i32, i32);
let black = Color(0, 0, 0);
println!("R value of the color black = {}", black.0);

// unit-like structs
struct AlwaysEqual;
let _a = AlwaysEqual;  
}