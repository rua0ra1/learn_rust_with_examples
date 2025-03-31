enum IpAddrKind{
    V4(String),
    V6(String),
}

struct IpAddr{
    kind: IpAddrKind,
    address:String,
}

// enum saves wide variety of data
enum Message{
    Quit,
    Move{x:i32,y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}


impl  Message {

    fn some_function(){
        println!("lets get rusty :");
    }
    
}

// match patterns
enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin:Coin)->u8{
    match coin{
        Coin::Penny=>1,
        Coin::Nickel =>5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    // let four =IpAddrKind::V4;
    // let six =IpAddrKind::V6;

    // let localhost = IpAddr{
    //     kind:IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    //let localhost =IpAddrKind::V4(String::from("127.0.0.1"));

    // // there are no null values in rust use option enum instead of null value
    // enum Option<T> {
    //     Some(T),
    //     None
    // }

   /*  // examples of optional enum values

    let some_number= Some(5);
    let some_string =Some("a string");

    let absent_number:Option<i32> = None; 

    // let analyse the another example
    let x =5;
    let y =Some(5);

    let sum=x+y.unwrap_or(0); */

    // let five = Some(5);
    // let six=plus_one(five);
    // let none=plus_one(None);

    let some_value =Some(3);
    match some_value{
        Some(3)=> print!("three"),
        _=> (),
    }
    if let Some(3) = some_value{
        println!("three ");
    }


}

fn plus_one(x: Option<i32>) -> Option<i32>{
    match x{
        // None=> None,
        Some(i)=> Some(i+1),
        _=> None   // use it if x value is anything other than some
    }

}
