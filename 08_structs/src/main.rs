// Regular struct with named fields
#[derive(Debug)]
struct User {
    active: bool,
    email: String,
    name: String
}

impl User {
    // can act as a constructor as in OOP - any function name can be used
    fn new(email: String, name: String) -> User {
        User {
            active: false,
            email: email,
            name: name
        }
    }

    fn id(&self) -> String {
        let mut id_string = self.name.clone();
        let email : &str = &self.email.clone()[..];

        id_string.push_str(":");
        id_string.push_str(email);

        return id_string;
    }

    fn same_user(&self, user2: &Self) -> bool {
        self.id() == user2.id()
    }
}

// Tuple struct without named fields
struct Color(i32, i32, i32);

fn main() {
    let user1 = User {
        active: false,
        email: String::from("test.user__@example.com"),
        name: String::from("LEO")
    };

    println!("User 1's ID is: {}", user1.id());
    println!("Checking if User 1 matches User 1: {}", user1.same_user(&user1));

    let mut user2 = User::new(String::from("t.email@gmail.com"), String::from("Kay"));
    user2.active = true;

    println!("Checking if User 1 matches User 2: {}", user1.same_user(&user2));

    let user3 = build_active_user(String::from("test-user@email.com"), String::from("IK"));
    println!("User 3: {:?}", user3);

    // using the `update` syntax.
    // `user1` would no longer be usable because at least one field (email | name) being moved from
    // `user1` to user`4` are of types (String in this case) that don't implement the `Copy` trait.
    let user4 = User {
        active: dbg!(2 > 1),
        ..user1
    };

    println!("User 4's name is: {}", user4.name);
    dbg!(&user4);

    // Tuple struct
    let white = Color(255, 255, 255);
    println!("The mid value in the white color is: {}", white.1);

    let Color(_r, _g, _b) = white;
}

// using the `field init shorthand` syntax for matching variable names
fn build_active_user(email: String, name: String) -> User {
    User {
        active: true,
        email,
        name
    }
}
