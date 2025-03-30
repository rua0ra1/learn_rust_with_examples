/* // struct allow to group realted data together

use std::string;

struct User{
    username: String,
    email: String,
    sign_in_count:u64,
    active: bool,

}

fn main() {
    let mut user1 =User{
        email:String::from("email id"),
        username: String::from("ram charan"),
        sign_in_count: 1,
        active:true,
    };

    let name=user1.username;
    user1.username=String::from("wallce123");

    let user2=build_user(
        String::from("icherry"), 
        String::from("icherry"));

    // rust will allow you to use othr object values

    let user3=User {
        email:String::from("ichrryme "),
        username:String::from("ram charan"),
        ..user1
    };

    // lets create a tuple struct 
    struct Color (i32,i32,i32);
    struct Point(i32,i32,i32);

}


fn build_user(email: String, username: String)-> User{
    User{
        email,
        username, // it is equal to username:username but it is confusing so avoid it
        active:true,
        sign_in_count:1,
    }
}

 */

 #[derive(Debug)]
 struct Rectangle{
    width:u32,
    height:u32,
 }

 // adding the method to the rectangle struct
 impl Rectangle{ // impl block house the functions and methods of user type
     fn area(&self)-> u32{
        self.width*self.height
     }

     fn can_hold(&self, other:&Rectangle)-> bool{
        self.width> other.width && self.height > other.height

     }
 }

 impl  Rectangle {

    // associated functions
    fn square(size:u32)-> Rectangle{
        Rectangle{
            width:size,
            height:size
        }
    }
     
 }


fn main(){
    let rect= Rectangle{
        width:30,
        height:50
    };

    let rect2= Rectangle{
        width:30,
        height:50
    };

    let rect3= Rectangle{
        width:40,
        height:60
    };

    let rect3 =Rectangle::square(25);
    let flag=rect2.can_hold(&rect3);

    println!(" the rect can hold the rect {} ",flag);



    // print the fields of the struct
    print!("rect : {:#?}", rect);

    println!("
    the area of the rectangle is {} square pixels. ",
    rect.area())
}
