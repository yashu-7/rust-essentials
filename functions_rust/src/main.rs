use std::io;

fn sum(numbers:&[i32]) -> i32 {
    numbers.iter().sum()
}

fn taking_number(size: i32) -> Vec<i32>{
    let mut arr = Vec::new();
    for _ in 0..size{
        let mut sz = String::new();
        println!("Enter number to append to array => ");
        io::stdin().read_line(&mut sz).expect("Unable to read input");
        let nums = sz.trim().parse().expect("Unable to convert input");
        arr.push(nums);
    }
    arr.to_vec()
}
fn main() {
    let mut input_size = String::new();
    println!("Enter size of array");
    io::stdin().read_line(&mut input_size).expect("Unable to read input size");
    let input_size:i32 = input_size.trim().parse().expect("Unable to convert string to int");
    
    let arr = taking_number(input_size);
    let sum = sum(&arr);
    println!("\nSum of array is: {sum}")
}
