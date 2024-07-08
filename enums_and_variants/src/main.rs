#[derive(Debug)]
enum DiskType{
    SSD,
    HDD,
}

#[derive(Debug)]
struct Details {
    name: String,
    dsk_type: DiskType,
}

fn main() {
    let disk_type = DiskType::SSD;
    match disk_type {
        DiskType::SSD => println!("It's a SSD"),
        DiskType::HDD => println!("It's a HDD"),
    }

    let user1 = Details {
        name: String::from("JOHN"),
        dsk_type: DiskType::SSD,
    };
    println!("{:?}",user1);
}
