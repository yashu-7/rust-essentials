fn main() {
    // let message = "HI";
    // let weight = 15;
    // let kilo = weight / 2.2;

    // println!("{},{}",message,kilo);
    // Cannot perform mathematical operation on different datatypes 

    let weight = 15.0;
    let kilo = weight / 2.2;
    println!("{}",kilo);

    let signal = false;
    if signal {
        println!("Recieved signal");
    }
    else {
        println!("Not recieving signal");
    }

    // mut -> allows it to change value
    let mut height = 190;
    height = height - 10;
    println!("{}",height);

    // inlining if-else condition
    let health = if height <180 {"good"} else {"not good"};
    println!("{}",health);
}
