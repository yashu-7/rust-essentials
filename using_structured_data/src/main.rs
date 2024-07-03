#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
    email: String,
    phone_number: i64,
}
impl Person{
    fn concatenate(&self) -> String{
        format!("{} {}",self.first_name,self.last_name)
    }
}
fn main() {
    println!("{:?}",Person{
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        age: 25,
        email: "abc@mail.com".to_string(),
        phone_number: 987654321,
    });
    println!("{:?}",Person{
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        age: 25,
        email: "abc@mail.com".to_string(),
        phone_number: 987654321,
    }.concatenate());
    
}
