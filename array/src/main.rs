use std::io;

fn main() {
    let a = [1,2,3,4,5];

    let mut index = String::new();
    println!("Enter the index of array");
    io::stdin()
    .read_line(&mut index)
    .expect("error in input");

    let index: usize = index.trim().parse().expect("Index is not number");

    let element = a[index];

    println!("{element}");

}
