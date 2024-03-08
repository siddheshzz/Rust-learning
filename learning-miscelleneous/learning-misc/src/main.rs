//Learnings:
/*
1. Type Conversion
2. User input using stdin


*/



use::std::io::stdin;
//recursive
fn factorial(num:i32) ->i32{
    if num==0{
        return 1;
    }
    num*factorial(num-1)
}


fn main() {
    println!("Factorial");


    //Write a programme which finds the factorial of a number entered by the user. (check for all conditions).
    let mut buffer = String::new();

    println!("Enter the Number");

    stdin().read_line(&mut buffer).expect("Failed to read the input");
    let mut num:i32=buffer.trim().parse::<i32>().expect("The provided input is not a whole number");
    let prev_num = num;

    let mut result:i32 = 1;
    while num>0 {
        result = result*num;
        num = num-1;
        
    }

    println!("The factorial of {} is {}",prev_num,result);

    println!("Recursicve call for {} is {}",prev_num, factorial(prev_num));
}
