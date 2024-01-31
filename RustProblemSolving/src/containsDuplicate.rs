use std::collections::HashMap;

fn main(){
    let nums = [1,2,3,1];
    let mut a = HashMap::new();
    for i in nums{
        if a.contains_key(&i){
            println!("{}",true);
        }
        a.insert(i,1);
    }
    println!("{}",false);
}