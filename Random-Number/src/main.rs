use std::io::stdin;
use rand;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    // create random number
    let ran = rand::thread_rng().gen_range(1,100);
    println!("guess a number");

    loop{
        // mut = mutable
        let mut var = String::new();
        //&mut ew thats gross
        stdin().read_line(&mut var)
            .expect("need to read number");

        //converts from a string to i32
        let var :i32 = var.trim().parse()
            .expect("error parsing");

        println!("You Guessed: {}" , var);

        match var.cmp(&ran){
            Ordering::Less => println!("Too low!{}"),
            Ordering::Greater => println!("Too High!"),
            Ordering::Equal => {
                println!("Correct!");
                break;
            }
        }
    }

}
