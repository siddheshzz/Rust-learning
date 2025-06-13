fn main(){
    by_moving();
    by_cloning();
    by_mutating();
}

fn by_cloning(){
    let hello = "hello".to_string();
    let world = "world!";

    let hello_world =  hello.clone() + world;
    // one can access hello as on line 11 we have used clone of hello keeping original intact


}

fn by_moving(){
    let hello = "hello".to_string();
    let world = "world!";

    let hello_world = hello + world;
    // println!("{}",hello);  ->  gives error stating hello was moved
    // one cannot access hello as it is consumed in  line 21
}

fn by_mutating(){
    let mut hello = "hello".to_string();
    let world = "world";

    hello.push_str(world);

    // one can call and use hello as it is mutable

    
}
