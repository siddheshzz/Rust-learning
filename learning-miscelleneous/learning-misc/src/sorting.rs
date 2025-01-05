fn bubble_sort(mut arr:Vec<i32>) -> Vec<i32>{

    for i in 0..arr.len(){
        for j in 1..arr.len(){
            if arr[j]<arr[j-1]{
                let temp = arr[j];
                arr[j] = arr[j-1];
                arr[j-1] = temp;
            }
        }
    }
    arr
}


fn selection_sort(mut arr: Vec<i32>) -> Vec<i32>{
    for i in 0..arr.len(){
        let mut min = i;
        for j in i+1..arr.len(){
            if arr[min]>arr[j]{
                min = j;
            }
        }
        if i != min{
            let temp = arr[min];
            arr[min] = arr[i];
            arr[i] = temp; 
        }
        
    }
    arr
}

fn insertion_sort(mut arr:Vec<i32>) -> Vec<i32>{


    for i in 1..arr.len(){
        let temp = arr[i];
        //  3,2,5,1,4
        let mut j = i-1; 
        while arr[j]>temp && j>0{
            arr[j+1] = arr[j];
            j = j-1;
        }
        arr[j+1] = temp;
    }
    arr
}
fn main(){
    println!("This is bubble sort:--");

    let arr:Vec<i32> = vec![3,4,1,2,5];

    println!("{:?}",bubble_sort(arr));

    let arr1:Vec<i32> = vec![3,4,1,2,5];

    println!("{:?}",selection_sort(arr1));

    let arr2:Vec<i32> = vec![3,4,1,2,5];

    println!("{:?}",selection_sort(arr2));
}