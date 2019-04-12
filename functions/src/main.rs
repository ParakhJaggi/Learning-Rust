use std::io::stdin;

fn main() {
    println!("{}",test());
}

fn test() -> i32{
    println!("Please type a number: ");
    let mut x = String::new();
    stdin().read_line(&mut x)
        .expect("test");

    let x :i32 = x.trim().parse()
        .expect("parsing");

    println!("You entered {} ", x);

    let y = x*2;
    //returning values is cool
    y

}
