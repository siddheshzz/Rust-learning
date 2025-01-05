fn main(){
    for i in (0..10).rev(){
        print!(" {i} ");
    }
    for i in (0..4).rev() {
    print!("{}, ",i);
    }
    
    if let Some(x) = fn1().or(fn2()){
        println!("{x}");
    }
}

fn fn1() -> Option<i8>{
    println!("fn1");
    return Some(1);
}
fn fn2() -> Option<i8>{
    println!("fn 2");
    return None;
}