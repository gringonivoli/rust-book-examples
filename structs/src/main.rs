fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername"),
        email: String::from("sss@sss.com"),
        sign_in_count: 1,
    };
    let user2 = build_user(String::from("otheruser"), String::from("ss@ssss.com"));
    let user3 = User {
        active: false,
        ..user2
    }; // only for update, becoz user2 is no longer valid...
       // println!("User: {user1.username}");
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
