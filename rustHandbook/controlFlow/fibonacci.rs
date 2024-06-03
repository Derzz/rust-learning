use std::io;

fn fibonacci(x: i32) -> i32{
  if x == 0 || x == 1{
      x
  }
    else {
        fibonacci(x - 1) + fibonacci(x - 2)
    }
}

fn main(){
    println!("What nth fibonacci number do you want to generate?");
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("failed to read");
    let x: i32 = x.trim().parse().expect("Please type a number!");
    println!("The sum is {} ", fibonacci(x))
}