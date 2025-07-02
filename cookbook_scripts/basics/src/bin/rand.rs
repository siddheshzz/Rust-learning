fn main(){
    let random_number = rand::random::<i32>();
    println!("Random i32 number {}", random_number);

    let random_num2:i32 = rand::random();
    println!("Random i32 number {}", random_num2);

    //Every primitive datatype can be randomized

    let random_char = rand::random::<char>();
    println!("Random char {}", random_char);

    let mut rng = rand::rng();

    

}