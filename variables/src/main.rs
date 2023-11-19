const PI: f64 = 3.14;
fn main() {
    let mut x = 5;
    println!("{x}");
    x = 6;
    println!("{x}");
    
    println!("{PI}");

    let y = 5;
    let y = y+1;
    {
        let y = y*2;
        println!("{y}");
    }
    println!("{y}");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("{spaces}");
}
