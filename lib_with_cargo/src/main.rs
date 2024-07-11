use lib_with_cargo::read_stdin;
fn main(){
    println!("Hello, World!");
    let hello = read_stdin();
    println!("{}",hello);
}