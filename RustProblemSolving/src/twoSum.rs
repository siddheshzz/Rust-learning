// Approach One:- Brute force O(n^2)
// Pick one value from vec and fimd the required other num to fullfill the sum.
// [-1,3,5,6,10]=> pick -1 search again in [3,5,6,10] if b = target - (-1) is present and return index.

// a + b = target=> b=target -a
// Approach Two:- HashMap O(n)
// Use hashmap to store the num as key and its index as its value,along with that also search if b is present in map.
// There is one more approach to this we can have two hashmaps and find the indexes but if the vec is long then we need to go with secound approach


use std::collections::HashMap;
fn main() {
    println!("Hello, world!");
    let nums = vec![3,3];
    let target:i32 = 6;
    let mut a: HashMap<i32, usize> = HashMap::new();
    let sol:Vec<i32> = Vec::new();
        for i in 0..nums.len(){
            let b = target - nums[i];
            if let Some(x) = a.get(&b){
                println!("{},{}",i as i32,*x);
                
            }
            else{
            a.insert(nums[i], i);
        }}
    // for i in 0..3{
    //     if a.contains_key(&(target-&nums[i])){
    //         // return [i,a.entry(target-nums[i])]
    //         if i == a.get(&(target-&nums[i])?{
    //             continue;
    //         }
    //         println!("{},{:?}",i , a.get(&(target-&nums[i])));
    //         a.remove(&nums[i]);
    //     }
        
    // }
    
    
    
}
