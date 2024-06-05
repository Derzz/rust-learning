// Generate fibonacci sequence until user stops

fn main() {
    let mut input = String::new();
    println!("Up to what number should the sequence stop?");
    std::io::stdin().read_line(&mut input).expect("Failed to read!");
    let ceil: i32 = input.trim().parse().expect("Failed to convert!");

    let mut prev: i32 = 0;
    let mut count: i32 = 1;

    loop{
        if count > ceil{
            break;
        }
        println!("{}", count);
        let temp = prev;
        prev = count;
        count = temp + count;
    }
}
