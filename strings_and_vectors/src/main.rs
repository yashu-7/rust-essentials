fn print_str(s: &str){
    println!("{}",s);
}

fn print_string(s: String){
    println!("{}",s);
}

fn longest_sentence(s: &String) -> usize{
    let words = s.split(" ").collect::<Vec<&str>>();
    let mut largest: usize = 0;
    for word in words{
        if word.len() > largest{
            largest = word.len();
        }
        else{
            continue;
        }
    }
    largest
}

fn main() {
    let s = "Hello, World!";
    print_str(s);

    let growable_string = String::from("Hello");
    print_string(growable_string);

    let mut vovel_count: i32 = 0;

    let sample = "Hello".to_string();
    for c in sample.chars(){
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                println!("Found vovel: {}",c);
                vovel_count+=1;},
            _ => continue,
        }
    }

    let words = sample.split(' ').collect::<Vec<&str>>();
    println!("{:?}",words);

    println!("{}",vovel_count);

    let s = "Hello this is a test to see longest word in a sentence!!".to_string();
    let largest = longest_sentence(&s);
    println!("{}",largest);

    ownership();
    modifiable();
    accessing_elements();
}

fn ownership(){
    let numbers = vec![1,2,3,4,5];
    let slice = &numbers[..];
    println!("{:?}",slice);
}

fn modifiable(){
    let mut numbers = vec![1,2,3,4];
    let slice = &mut numbers[..];
    slice[0] = 0;
    println!("{:?}",slice);
}

fn accessing_elements(){
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    let one = &numbers[0];
    println!("{}",one);

    let last = numbers.last();
    match last{
        Some(last)=> println!("Last element --> {last}"),
        None => println!("Empty"),
    }
}