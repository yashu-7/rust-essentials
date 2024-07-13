use lib_with_cargo::read_stdin;
use lib_with_cargo::colors::red;
fn main(){
    println!("Hello, World!");
    let hello = read_stdin();
    println!("{}",hello);
    println!("{}",red(&hello));
}