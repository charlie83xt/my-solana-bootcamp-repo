
fn main() {
    let mut counter: i32 = 0;
    println!("Welcome to Fizz Buzz Game!");
    for i in 0..=301 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("fizz buzz");
            counter += 1;
        } else if i % 3 == 0 {
            println!("fizz");
        } else if i % 5 == 0 {
            println!("buzz");
        }
    }
    println!("Fizz Buzz occurred {} times", counter);
}
