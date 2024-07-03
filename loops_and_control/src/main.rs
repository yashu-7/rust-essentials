fn main() {
    let mut x = 5;
    while x > 0{
        println!("{}",x);
        x-=1;
    }

    for i in 1..5{
        println!("{}",i);
    }
    for i in 1..=5{
        println!("{}",i);
    }
    for i in (1..=5).rev(){
        println!("{}",i);
    }
}
