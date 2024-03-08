use std::io::{self, Stdin};
use fancy::printcoln;

fn read_from_console()->String{
    let mut buff=String::new();

    let _ = io::stdin().read_line(&mut buff);
    return buff;

}
fn print(list:&mut Vec<String>){
    println!("\n\n");
    println!("<=== To Do List ===>");
    for i in list {
        println!("{}",i);
    }
    println!("\n");
}

fn input(list:&mut Vec<String>){

    list.push(read_from_console().trim().to_owned());
    print(list);
}

fn mark_done(list:&mut Vec<String>){
    println!("Enter the task number");
    let indx = ((read_from_console().trim().to_owned().parse::<i32>().unwrap())-1) as usize;
    let mut s= list.get(indx.clone()).unwrap();
    if s.contains("✅")  {
        let x = s.trim_end_matches("✅");
        list.insert(indx,x.to_string());
        let _ = list.remove(indx+1 as usize);
    }else {
        let s= list.get(indx.clone()).unwrap();
        list.insert(indx, s.to_owned()+&"✅");
        let _ = list.remove(indx+1);
    }
    // s.push_str("✅");
}

fn delete_task(list:&mut Vec<String>){
    println!("✅Enter the task number");
    list.remove(((read_from_console().trim().to_owned().parse::<i32>().unwrap())-1) as usize);
    print(list)
}

fn main() {
    println!("<=== Welcome ===>");
    printcoln!("[bold|cyan]Hello world[magenta]!");
    let mut list: Vec<String> = Vec::new();
    list.push("abc".to_owned());
    loop {
        println!("Select a choice:\n1. Add task\n2. Delete task\n3. Mark Unmark\n4. Show\n5. Exit");
        
        match read_from_console().as_str().trim() {
            "1" =>input(&mut list),
            "2" =>delete_task(&mut list),
            "3" =>mark_done(&mut list),
            "4" =>print(&mut list),
            "5" => break,
            _ => println!("Please select a correct choice")
        }
    }
    print(&mut list);
}
