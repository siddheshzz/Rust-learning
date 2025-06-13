use std::fmt::format;

fn main(){
    // format macro 

    let hello = "hello";
    let world = "world";

    let hello_world = format!("{},{}", hello, world);
    println!("{}",hello_world);

    let favorite_num = format!("My fav num is {}", 23);

    let play_work = format!("{0}, {0}, {1}","play","work");
    println!("{}",play_work);

    
}