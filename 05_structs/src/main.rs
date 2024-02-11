// #[derive(Debug)]
// // Initilization of struct
// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u32,
//     active: bool,
// }

// fn main() {
//     // creating user by using Structs
//     let mut user1 = User {
//         username: String::from("Dev"),
//         email: String::from("dev.test@test.com"),
//         sign_in_count: 1,
//         active: true,
//     };
//     // Retreving data from stucts
//     let name = user1.username;

//     // Changing data from stucts
//     user1.username = String::from("test");

//     let user2 = build_user(String::from("test1"), String::from("test1@test.com"));
//     let user3 = User{
//         username:String::from("test3"),
//         email:String::from("test3@test.com"),
//         ..user2
//     };

//     println!("user2: {:#?}", user2);
//     println!("user3: {:#?}", user3);
// }

// fn build_user(username: String, email: String) -> User {
//     User {
//         username,
//         email,
//         active: true,
//         sign_in_count: 1,
//     }
// }

struct Reactange {
    width: u32,
    height: u32,
}

impl Reactange {
    fn area(&self) -> u32 {
        &self.width * &self.height
    }
    fn can_hold(&self, other: &Reactange) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// ----------------------------------------------------------------------------------------
fn main() {
    let width1 = 30;
    let height1 = 50;

    let rect = Reactange {
        width: 30,
        height: 50,
    };
    let react1 =Reactange{
        width:100,
        height:70,
    };
    let react3 =Reactange{
        width:10,
        height:20,
    };

    println!("The area of teh rectange is {} sqaure pixels.", rect.area());
    println!("rect can hold react1:{}", rect.can_hold(&react1));
    println!("rect can hold react3:{}", rect.can_hold(&react3));
}

// fn area(rect:&Reactange)->u32 {
//     rect.width *rect.height
// }
// fn area(width: u32, height: u32)->u32 {
//     width * height
// }
